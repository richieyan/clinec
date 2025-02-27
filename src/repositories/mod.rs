use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn create_project_structure(project_name: &str) -> Result<()> {
    // 创建项目根目录
    fs::create_dir_all(project_name)?;
    
    // 创建cline_docs目录
    let docs_dir = format!("{}/cline_docs", project_name);
    fs::create_dir_all(&docs_dir)?;

    Ok(())
}

pub fn get_template_content(tech_stack: &str) -> Result<String> {
    match tech_stack {
        "python" => Ok(include_str!("../../templates/python_template.md").to_string()),
        "java" => Ok(include_str!("../../templates/java_template.md").to_string()),
        "go" => Ok(include_str!("../../templates/go_template.md").to_string()),
        "rust" => Ok(include_str!("../../templates/rust_template.md").to_string()),
        _ => Err(anyhow::anyhow!("Unsupported tech stack: {}", tech_stack)),
    }
}

pub fn get_gitignore_template(tech_stack: &str) -> Result<String> {
    match tech_stack {
        "python" => Ok(include_str!("../../templates/python.gitignore").to_string()),
        "java" => Ok(include_str!("../../templates/java.gitignore").to_string()),
        "go" => Ok(include_str!("../../templates/go.gitignore").to_string()),
        "rust" => Ok(include_str!("../../templates/rust.gitignore").to_string()),
        _ => Err(anyhow::anyhow!("Unsupported tech stack: {}", tech_stack)),
    }
}

pub fn write_file(path: &str, content: &str) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
