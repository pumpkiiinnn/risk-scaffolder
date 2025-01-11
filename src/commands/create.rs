use std::fs;
use std::path::Path;
use crate::templates::{
    generate_root_pom,
    generate_module_pom,
    generate_logback_config,
    generate_application_yml
};

pub fn create_project(project_name: &str, output_path: &str) {
    let base_path = Path::new(output_path).join(project_name);
    
    // 创建主项目目录
    fs::create_dir_all(&base_path).expect("无法创建项目目录");
    
    // 创建子模块目录
    let modules = vec![
        "entity",
        "backend",
        "listener",
        "analysis",
    ];
    
    for module in &modules {
        let module_path = base_path.join(format!("{}-{}", project_name, module));
        create_module_structure(&module_path, module);
    }
    
    // 创建根pom.xml
    create_root_pom(&base_path, project_name);
    
    // 为每个模块创建pom.xml
    for module in &modules {
        let module_path = base_path.join(format!("{}-{}", project_name, module));
        create_module_pom(&module_path, project_name, module);
    }
    
    println!("项目 {} 创建成功！", project_name);
}

fn create_module_structure(module_path: &Path, module_type: &str) {
    let src_main_java = module_path.join("src/main/java");
    let src_main_resources = module_path.join("src/main/resources");
    
    fs::create_dir_all(&src_main_java).expect("无法创建Java源码目录");
    fs::create_dir_all(&src_main_resources).expect("无法创建资源目录");
    
    match module_type {
        "backend" | "analysis" => {
            fs::create_dir_all(src_main_resources.join("META-INF"))
                .expect("无法创建META-INF目录");
            create_logback_config(&src_main_resources);
            create_application_yml(&src_main_resources);
        }
        "listener" => {
            create_application_yml(&src_main_resources);
        }
        _ => {}
    }
}

fn create_root_pom(base_path: &Path, project_name: &str) {
    let content = generate_root_pom(project_name);
    fs::write(base_path.join("pom.xml"), content)
        .expect("无法创建根pom.xml");
}

fn create_module_pom(module_path: &Path, project_name: &str, module_type: &str) {
    let content = generate_module_pom(project_name, module_type);
    fs::write(module_path.join("pom.xml"), content)
        .expect("无法创建模块pom.xml");
}

fn create_logback_config(resources_path: &Path) {
    let content = generate_logback_config();
    fs::write(resources_path.join("logback-spring.xml"), content)
        .expect("无法创建logback配置文件");
}

fn create_application_yml(resources_path: &Path) {
    let content = generate_application_yml();
    fs::write(resources_path.join("application.yml"), content)
        .expect("无法创建application.yml");
} 