use dialoguer::{Input, Select, theme::ColorfulTheme};
use console::style;
use java_scaffold::create_project;
use java_scaffold::config::ProjectConfig;

fn main() {
    println!("{}", style("欢迎使用Java项目脚手架工具！").cyan().bold());
    println!("{}", style("这个工具将帮助你创建一个标准的Java多模块项目。").cyan());
    
    // 获取项目名称
    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("请输入项目名称")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("项目名称不能为空")
            } else if input.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
                Err("项目名称只能包含字母、数字、横线和下划线")
            } else {
                Ok(())
            }
        })
        .interact()
        .expect("无法读取输入");

    // 获取组织名称
    let group_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("请输入组织名称 (例如: com.example)")
        .default("com.example".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("组织名称不能为空")
            } else if !input.contains('.') {
                Err("组织名称格式不正确，应该包含至少一个点号 (例如: com.example)")
            } else {
                Ok(())
            }
        })
        .interact()
        .expect("无法读取输入");

    // 选择JDK版本
    let jdk_versions = vec!["8", "11", "17", "21"];
    let jdk_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择JDK版本")
        .default(2) // 默认选择JDK 17
        .items(&jdk_versions)
        .interact()
        .expect("无法读取选择");
    let java_version = jdk_versions[jdk_selection];

    // 使用当前目录作为输出路径
    let output_path = ".";
    
    println!("\n{}", style("正在创建项目...").green());
    
    let config = ProjectConfig {
        name: project_name.clone(),
        group_id,
        java_version: java_version.to_string(),
        output_path: output_path.to_string(),
    };
    
    create_project(&config);
    
    println!("\n{}", style("✨ 项目创建成功！").green().bold());
    println!("\n项目结构：");
    println!("{}
├── pom.xml
├── {}-entity/
├── {}-backend/
├── {}-listener/
└── {}-analysis/", 
        project_name, project_name, project_name, project_name, project_name
    );
    
    println!("\n{}", style("下一步：").yellow().bold());
    println!("1. cd {}", project_name);
    println!("2. mvn install");
    println!("3. 开始开发你的项目！");
}
