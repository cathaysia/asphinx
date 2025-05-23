use gix::object::tree::diff::Change;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::RwLock;

use chrono::{FixedOffset, Utc};
use gix::{
    bstr::ByteSlice, object::tree::diff::Action, path::Utf8Error, Commit, Id, ThreadSafeRepository,
};
use tracing::*;

pub struct GitInfo {
    mtimes: Arc<RwLock<HashMap<String, u32>>>,
    default_time: Arc<RwLock<String>>,
}

impl GitInfo {
    pub async fn new(repo_dir: String, pb: ProgressBar) -> anyhow::Result<Self> {
        let mtimes = Arc::new(RwLock::new(HashMap::<String, u32>::new()));
        let default_time: Arc<RwLock<String>> = Default::default();

        {
            let repo_dir = repo_dir.clone();
            let mut mtimes = mtimes.clone().write_owned().await;
            let mut default_time = default_time.clone().write_owned().await;
            tokio::task::spawn_blocking(move || {
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .template("{spinner:.green} {elapsed_precise} {msg}")
                        .unwrap(),
                );
                pb.set_message("Parse Git info...");

                let f = || {
                    let repo = ThreadSafeRepository::discover(repo_dir)?.to_thread_local();
                    let rewalk = repo.rev_walk(Some(repo.head_id()?.detach()));
                    let mut changes = rewalk.all()?.filter_map(Result::ok);
                    let mut last = changes.next().unwrap().id();
                    let mut cont: Vec<_> = Default::default();
                    for next in changes {
                        cont.push((last, next.id()));
                        last = next.id();
                    }

                    let systime = match cont.last() {
                        Some(v) => std::time::SystemTime::UNIX_EPOCH
                            .checked_add(Duration::new(
                                Self::id_to_commit(&v.0)?.time()?.seconds as u64,
                                0,
                            ))
                            .unwrap_or(SystemTime::now()),
                        None => SystemTime::now(),
                    };

                    *default_time = {
                        let default_time: chrono::DateTime<Utc> = systime.into();
                        default_time
                            .with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string()
                    };

                    *mtimes = cont
                        .into_iter()
                        .map(|(last, next_shad)| -> anyhow::Result<_> {
                            let last = Self::id_to_commit(&last)?;
                            let next_shad = Self::id_to_commit(&next_shad)?;
                            let mut res: Vec<(String, u32)> = Default::default();
                            if let Ok((time, set)) =
                                Self::change_from_commit(&last, Some(&next_shad))
                            {
                                res = set
                                    .into_iter()
                                    .inspect(|filename| {
                                        pb.set_message(format!("parse git info of: {}", filename));
                                    })
                                    .map(|filename| (filename, time))
                                    .collect();
                            }

                            Ok(res)
                        })
                        .filter_map(|item| item.ok())
                        .flatten()
                        .rev()
                        .collect();
                    drop(mtimes);
                    pb.set_message("parse git info completed");
                    Ok(()) as anyhow::Result<()>
                };
                let _ = f();
            });
        }

        debug!("Get modify datetime for the file is: {:?}", mtimes);
        Ok(Self {
            mtimes,
            default_time,
        })
    }

    pub async fn get_last_commit_time_of_file(&self, file_name: &str) -> Option<String> {
        debug!("Get file date: {}", file_name);
        match self.mtimes.read().await.get(file_name) {
            Some(time) => {
                let systime = std::time::SystemTime::UNIX_EPOCH
                    .checked_add(Duration::new(*time as u64, 0))
                    .unwrap();
                let time: chrono::DateTime<Utc> = systime.into();
                trace!("Last modify date for {file_name} is: {:?}", systime);
                Some(time.format("%Y-%m-%d %H:%M:%S").to_string())
            }
            None => Some(self.default_time.read().await.clone()),
        }
    }

    fn id_to_commit<'a>(id: &'a Id<'a>) -> anyhow::Result<Commit<'a>> {
        Ok(id.try_object()?.expect("empty").try_into_commit()?)
    }

    fn change_from_commit(
        last: &Commit,
        next: Option<&Commit>,
    ) -> anyhow::Result<(u32, HashSet<String>)> {
        let tree = last.tree()?;
        let mut changes = tree.changes()?;
        let last_tree = next.unwrap().tree()?;
        let mut filenames = HashSet::new();
        changes.for_each_to_obtain_tree(
            &last_tree,
            |change| -> Result<gix::object::tree::diff::Action, _> {
                let is_file_change = !matches!(
                    change,
                    Change::Deletion {
                        location: _,
                        entry_mode: _,
                        relation: _,
                        id: _
                    }
                );
                if is_file_change {
                    let path = change.location().to_os_str().unwrap().to_string_lossy();
                    filenames.insert(format!("{}", path));
                }

                Ok::<Action, Utf8Error>(Action::Continue)
            },
        )?;

        let time = last.time()?;
        Ok((time.seconds.try_into()?, filenames))
    }
}
