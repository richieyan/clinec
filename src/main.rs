mod cli;
mod services;
mod repositories;

use anyhow::Result;
use cli::parse_args;
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();
    
    let (tech_stack_opt, project_name_opt, verbose, list_stacks) = parse_args();
    
    if list_stacks {
        println!("Supported technology stacks:");
        println!("- go");
        println!("- java");
        println!("- python");
        println!("- rust");
        println!("- java-springboot");
        return Ok(());
    }

    let tech_stack = tech_stack_opt.ok_or_else(|| anyhow::anyhow!("Tech stack must be specified"))?;
    let project_name = project_name_opt.ok_or_else(|| anyhow::anyhow!("Project name must be specified"))?;
    
    if verbose {
        debug!("Verbose mode enabled");
        debug!("Tech stack: {}", tech_stack);
        debug!("Project name: {}", project_name);
    }

    // 验证技术栈是否支持
    if !services::validate_tech_stack(&tech_stack) {
        eprintln!("Error: Unsupported tech stack '{}'", tech_stack);
        std::process::exit(1);
    }

    // 创建项目目录结构
    repositories::create_project_structure(&project_name)?;
    if verbose {
        debug!("Project structure created");
    }

    // 生成配置文件
    services::generate_config_files(&tech_stack, &project_name)?;
    if verbose {
        debug!("Config files generated");
    }

    info!(
        "Successfully created project '{}' with {} configuration",
        project_name, tech_stack
    );

    Ok(())
}
