use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use chrono::Utc;
use gix::{
    bstr::ByteSlice,
    object::tree::diff::{change::Event, Action},
    path::Utf8Error,
    Commit, Id, ThreadSafeRepository,
};
use log::*;

pub struct GitInfo {
    mtimes: HashMap<String, u32>,
    default_time: String,
}

impl GitInfo {
    pub fn new(repo_dir: &str) -> Option<Self> {
        let repo = ThreadSafeRepository::discover(repo_dir)
            .ok()?
            .to_thread_local();
        let rewalk = repo.rev_walk(Some(repo.head_id().unwrap().detach()));
        let mut changes = rewalk.all().ok()?.filter_map(Result::ok);
        let mut last = changes.next()?.id();
        let mut cont: Vec<_> = Default::default();
        for next in changes {
            cont.push((last, next.id()));
            last = next.id();
        }

        let systime = std::time::SystemTime::UNIX_EPOCH
            .checked_add(Duration::new(
                Self::id_to_commit(&cont.last().unwrap().0)
                    .unwrap()
                    .time()
                    .ok()?
                    .seconds as u64,
                0,
            ))
            .unwrap();
        let default_time: chrono::DateTime<Utc> = systime.into();
        let default_time = default_time.format("%Y-%m-%d %H:%M:%S").to_string();

        let mtimes: HashMap<String, u32> = cont
            .into_iter()
            .map(|(last, next_shad)| {
                let last = Self::id_to_commit(&last).unwrap();
                let next_shad = Self::id_to_commit(&next_shad).unwrap();
                let mut res: Vec<(String, u32)> = Default::default();
                if let Some((time, set)) = Self::change_from_commit(&last, Some(&next_shad)) {
                    res = set.into_iter().map(|filename| (filename, time)).collect();
                }

                res
            })
            .flatten()
            .rev()
            .collect();

        debug!("获取的文件修改时间为：{:?}", mtimes);
        Some(Self {
            mtimes,
            default_time,
        })
    }

    pub fn get_last_commit_time_of_file(&self, file_name: &str) -> Option<String> {
        info!("获取文件日期：{}", file_name);
        match self.mtimes.get(file_name) {
            Some(time) => {
                let systime = std::time::SystemTime::UNIX_EPOCH
                    .checked_add(Duration::new(*time as u64, 0))
                    .unwrap();
                let time: chrono::DateTime<Utc> = systime.into();
                trace!("文件 {file_name} 最后一次时间为：{:?}", systime);
                Some(time.format("%Y-%m-%d %H:%M:%S").to_string())
            }
            None => Some(self.default_time.clone()),
        }
    }

    fn id_to_commit<'a>(id: &'a Id<'a>) -> Option<Commit<'a>> {
        Some(
            id.try_object()
                .ok()?
                .expect("empty")
                .try_into_commit()
                .ok()?,
        )
    }

    fn change_from_commit(last: &Commit, next: Option<&Commit>) -> Option<(u32, HashSet<String>)> {
        let tree = last.tree().ok()?;
        let mut changes = tree.changes().ok()?;
        let changes = changes.track_path();
        let last_tree = next?.tree().ok()?;
        let mut filenames = HashSet::new();
        changes
            .for_each_to_obtain_tree(
                &last_tree,
                |change| -> Result<gix::object::tree::diff::Action, _> {
                    let is_file_change = match change.event {
                        Event::Deletion {
                            entry_mode: _,
                            id: _,
                        } => false,
                        _ => true,
                    };
                    if is_file_change {
                        let path = change.location.to_os_str().unwrap().to_string_lossy();
                        filenames.insert(format!("{}", path));
                    }

                    Ok::<Action, Utf8Error>(Action::Continue)
                },
            )
            .ok()?;

        let time = last.time().ok()?;
        Some((time.seconds.try_into().unwrap(), filenames))
    }
}
