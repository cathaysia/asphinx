use std::collections::{HashMap, HashSet};
use std::time::Duration;

use gix::{bstr::ByteSlice, path::Utf8Error, Commit, Id, ThreadSafeRepository};

use gix::object::tree::diff::change::Event;
use gix::object::tree::diff::Action;

use chrono::Utc;
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
        let mut mtimes: HashMap<String, u32> = HashMap::new();
        let mut last = Self::id_to_commit(changes.next()?.id())?;
        for next in changes {
            let next_shad = Self::id_to_commit(next.id()).unwrap();
            match Self::change_from_commit(&last, Some(&next_shad)) {
                Some((time, set)) => {
                    set.iter().for_each(|filename| {
                        mtimes.entry(filename.into()).or_insert_with(|| time);
                    });
                }
                None => {}
            }
            last = next_shad;
        }

        let systime = std::time::SystemTime::UNIX_EPOCH
            .checked_add(Duration::new(
                last.time().ok()?.seconds_since_unix_epoch as u64,
                0,
            ))
            .unwrap();
        let default_time: chrono::DateTime<Utc> = systime.into();
        let default_time = default_time.format("%Y-%m-%d %H:%M:%S").to_string();

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

    fn id_to_commit(id: Id) -> Option<Commit> {
        let object = id.try_object().ok()?;
        let object = object.expect("empty");
        let commit = object.try_into_commit().ok()?;
        Some(commit)
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
        Some((time.seconds_since_unix_epoch, filenames))
    }
}
