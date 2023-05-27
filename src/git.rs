use tokio::process;
use tracing::*;

pub async fn get_last_commit_time(file_path: &str) -> Option<String> {
    // git log -1 --pretty=%as file_path
    info!("获取文件日期：{}", file_path);
    let output = process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=%as")
        .arg(file_path)
        .output()
        .await;
    match output {
        Ok(v) => {
            let out = String::from_utf8(v.stdout);
            match out {
                Ok(v) => {
                    return Some(v);
                }
                Err(_) => {
                    return None;
                }
            }
        }
        Err(_) => None,
    }
}
