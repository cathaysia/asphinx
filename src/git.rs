use std::{cmp::max, collections::HashMap, time::Duration};

use chrono::Utc;
use git2::Repository;
use tracing::*;

pub struct GitInfo {
    mtimes: HashMap<String, i64>,
}

impl GitInfo {
    // COPYRIGHT: modify by https://github.com/rust-lang/git2-rs/issues/588#issuecomment-856757971
    // FIXME:head 上的文件会丢失
    pub fn new(repo_dir: &str) -> Option<Self> {
        let mut mtimes: HashMap<String, i64> = HashMap::new();
        let repo = Repository::open(repo_dir).ok()?;
        let mut revwalk = repo.revwalk().ok()?;
        revwalk.set_sorting(git2::Sort::TIME).ok()?;
        revwalk.push_head().ok()?;
        for commit_id in revwalk {
            let commit_id = commit_id.ok()?;
            let commit = repo.find_commit(commit_id).ok()?;
            // Ignore merge commits (2+ parents) because that's what 'git whatchanged' does.
            // Ignore commit with 0 parents (initial commit) because there's nothing to diff against
            if commit.parent_count() == 1 {
                let prev_commit = commit.parent(0).ok()?;
                let tree = commit.tree().ok()?;
                let prev_tree = prev_commit.tree().ok()?;
                let diff = repo
                    .diff_tree_to_tree(Some(&prev_tree), Some(&tree), None)
                    .ok()?;
                for delta in diff.deltas() {
                    let file_path = delta.new_file().path().unwrap();
                    let file_mod_time = commit.time();
                    let unix_time = file_mod_time.seconds();
                    mtimes
                        .entry(file_path.to_owned().to_string_lossy().to_string())
                        .and_modify(|t| *t = max(*t, unix_time))
                        .or_insert(unix_time);
                }
            }
        }
        debug!("获取的文件修改时间为：{:?}", mtimes);
        Some(Self { mtimes })
    }

    pub fn get_last_commit_time_of_file(&self, file_name: &str) -> Option<String> {
        let time = self.mtimes.get(file_name)?;
        let systime = std::time::SystemTime::UNIX_EPOCH
            .checked_add(Duration::new(*time as u64, 0))
            .unwrap();
        let time: chrono::DateTime<Utc> = systime.into();
        trace!("文件 {file_name} 最后一次时间为：{:?}", systime);
        Some(time.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}
