use anyhow::Result;

pub fn validate_tech_stack(tech_stack: &str) -> bool {
    // 当前支持的技术栈
    let supported_stacks = ["go", "java", "python", "rust"];
    supported_stacks.contains(&tech_stack)
}

pub fn generate_config_files(tech_stack: &str, project_name: &str) -> Result<()> {
    // 获取模板内容
    let template_content = super::repositories::get_template_content(tech_stack)?;
    
    // 生成.clinerules文件
    let rules_path = format!("{}/cline_docs/.clinerules", project_name);
    super::repositories::write_file(&rules_path, &template_content)?;

    // 创建空的projectBrief.md
    let brief_path = format!("{}/cline_docs/projectBrief.md", project_name);
    super::repositories::write_file(&brief_path, "")?;

    // 生成.gitignore文件
    let gitignore_template = super::repositories::get_gitignore_template(tech_stack)?;
    let gitignore_path = format!("{}/.gitignore", project_name);
    super::repositories::write_file(&gitignore_path, &gitignore_template)?;

    Ok(())
}
