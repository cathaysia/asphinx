use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tracing::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub title: Option<String>,
    pub path: String,
    pub url: Option<String>,
    pub is_directory: bool,
    pub children: Vec<FileTreeNode>,
    pub level: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileTreeData {
    pub root: Vec<FileTreeNode>,
    pub flat_list: Vec<FileTreeNode>,
}

impl FileTreeData {
    pub async fn generate(content_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut root = Vec::new();
        let mut flat_list = Vec::new();

        let content_path = Path::new(content_dir);
        if !content_path.exists() {
            warn!("Content directory doesn't exist: {}", content_dir);
            return Ok(FileTreeData { root, flat_list });
        }

        scan_directory(content_path, &mut root, &mut flat_list, 0).await?;

        // Sort children by name (directories first, then files)
        Self::sort_tree_nodes(&mut root);

        Ok(FileTreeData { root, flat_list })
    }

    fn sort_tree_nodes(nodes: &mut Vec<FileTreeNode>) {
        nodes.sort_by(|a, b| {
            // Directories first, then files
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        // Recursively sort children
        for node in nodes {
            Self::sort_tree_nodes(&mut node.children);
        }
    }
}

// Use Box::pin to handle async recursion safely
fn scan_directory<'a>(
    dir_path: &'a Path,
    nodes: &'a mut Vec<FileTreeNode>,
    flat_list: &'a mut Vec<FileTreeNode>,
    level: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + 'a>>
{
    Box::pin(async move {
        let mut entries = fs::read_dir(dir_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // Skip hidden files and directories
            if file_name.starts_with('.') {
                continue;
            }

            let relative_path = path
                .strip_prefix("content")
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            if path.is_dir() {
                let mut children = Vec::new();
                scan_directory(&path, &mut children, flat_list, level + 1).await?;

                let node = FileTreeNode {
                    name: file_name,
                    title: None,
                    path: relative_path.clone(),
                    url: None,
                    is_directory: true,
                    children,
                    level,
                };

                flat_list.push(node.clone());
                nodes.push(node);
            } else if path.extension().and_then(|s| s.to_str()) == Some("adoc") {
                let title = extract_title(&path).await;
                let url = Some(relative_path.replace(".adoc", ".html"));

                let node = FileTreeNode {
                    name: file_name,
                    title,
                    path: relative_path,
                    url,
                    is_directory: false,
                    children: Vec::new(),
                    level,
                };

                flat_list.push(node.clone());
                nodes.push(node);
            }
        }

        Ok(())
    })
}

async fn extract_title(file_path: &Path) -> Option<String> {
    match fs::read_to_string(file_path).await {
        Ok(content) => {
            // Extract title from first line starting with '='
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('=') && !trimmed.starts_with("==") {
                    // Remove the '=' and trim whitespace
                    let title = trimmed.trim_start_matches('=').trim();
                    if !title.is_empty() {
                        return Some(title.to_string());
                    }
                }
                // Stop at first non-empty, non-comment line that doesn't start with ':'
                if !trimmed.is_empty() && !trimmed.starts_with(':') && !trimmed.starts_with("//") {
                    break;
                }
            }
            None
        }
        Err(err) => {
            warn!("Failed to read file {}: {}", file_path.display(), err);
            None
        }
    }
}

pub async fn generate_filetree_json(
    content_dir: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Generating file tree from: {}", content_dir);

    let filetree_data = FileTreeData::generate(content_dir).await?;
    let json_content = serde_json::to_string_pretty(&filetree_data)?;

    fs::write(output_path, json_content).await?;

    info!(
        "File tree generated: {} nodes",
        filetree_data.flat_list.len()
    );
    Ok(())
}
