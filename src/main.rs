use dialoguer::{Input, theme::ColorfulTheme};
use console::style;
use java_scaffold::create_project;

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

    // 使用当前目录作为输出路径
    let output_path = ".";
    
    println!("\n{}", style("正在创建项目...").green());
    create_project(&project_name, output_path);
    
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
