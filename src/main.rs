use regex::Regex;
use tracing::*;
use tracing_subscriber;

async fn generate_html(file_path: String) {
    let file_path = std::path::Path::new(&file_path);
    let parent_path = Into::<String>::into(file_path.parent().unwrap().to_str().unwrap())
        .replace("content", "public");
    if let Err(e) = std::fs::create_dir_all(&parent_path) {
        error!("创建 {} 时发生错误：{}", parent_path, e);
        return;
    }

    debug!("为 {} 生成 html", file_path.display());

    std::process::Command::new("asciidoctor")
        .arg(file_path)
        .arg("-D")
        .arg(parent_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn handle_file(file_path: String) {
    let file_path = std::path::Path::new(&file_path);
    info!("处理文件：{}", file_path.display());
    if !file_path.exists() {
        warn!("文件 {} 不存在", file_path.display());
        return;
    }

    let dir_path = file_path.parent().unwrap();

    if file_path.ends_with("index.adoc") {
        tokio::spawn(generate_html(file_path.to_str().unwrap().into()));
        let content = std::fs::read_to_string(file_path).unwrap();
        let re = Regex::new(r"xref:(.*)\[.*\]").unwrap();
        for item in re.captures_iter(&content) {
            let file_name: String = item.get(1).unwrap().as_str().into();
            let file_path: String = dir_path.join(file_name.as_str()).to_str().unwrap().into();
            handle_file(file_path);
        }
    } else {
        tokio::spawn(generate_html(file_path.to_str().unwrap().into()));
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let file_path = "content/index.adoc";

    handle_file(file_path.into());

    Ok(())
}
