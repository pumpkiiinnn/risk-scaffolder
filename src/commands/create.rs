use crate::config::ProjectConfig;
use crate::templates::{
    generate_analysis_application, generate_application_yml, generate_backend_application,
    generate_base_entity, generate_listener_application, generate_logback_config,
    generate_module_pom, generate_root_pom, generate_spark_service, generate_user_controller,
    generate_user_entity,
};
use std::fs;
use std::path::Path;

pub fn create_project(config: &ProjectConfig) {
    let base_path = Path::new(&config.output_path).join(&config.name);
    fs::create_dir_all(&base_path).expect("无法创建项目目录");

    // 创建根 pom.xml
    fs::write(base_path.join("pom.xml"), generate_root_pom(config)).expect("无法创建根pom.xml");

    // 创建各个模块
    create_entity_module(&base_path, config);
    create_backend_module(&base_path, config);
    create_listener_module(&base_path, config);
    create_analysis_module(&base_path, config);
}

fn create_module_structure(module_path: &Path) {
    let src_main_java = module_path.join("src/main/java");
    let src_main_resources = module_path.join("src/main/resources");

    fs::create_dir_all(&src_main_java).expect("无法创建Java源码目录");
    fs::create_dir_all(&src_main_resources).expect("无法创建资源目录");
}

fn create_entity_module(base_path: &Path, config: &ProjectConfig) {
    let module_path = base_path.join(format!("{}-entity", config.name));
    create_module_structure(&module_path);

    // 创建 pom.xml
    fs::write(
        module_path.join("pom.xml"),
        generate_module_pom(config, "entity"),
    )
    .expect("无法创建entity pom.xml");

    // 创建Java包路径
    let package_path = config.group_id.replace(".", "/");
    let java_path = module_path.join(format!("src/main/java/{}/entity", package_path));
    fs::create_dir_all(&java_path).expect("无法创建entity包路径");

    // 创建基础实体类
    fs::write(
        java_path.join("BaseEntity.java"),
        generate_base_entity(&config.group_id),
    )
    .expect("无法创建BaseEntity.java");

    // 创建示例实体类
    fs::write(
        java_path.join("User.java"),
        generate_user_entity(&config.group_id),
    )
    .expect("无法创建User.java");
}

fn create_listener_module(base_path: &Path, config: &ProjectConfig) {
    let module_path = base_path.join(format!("{}-listener", config.name));
    create_module_structure(&module_path);

    // 创建 pom.xml
    fs::write(
        module_path.join("pom.xml"),
        generate_module_pom(&config, "listener"),
    )
    .expect("无法创建listener pom.xml");

    // 创建Java代码目录
    let java_path = module_path.join("src/main/java/com/example/listener");
    fs::create_dir_all(&java_path).expect("无法创建listener包路径");

    // 创建应用主类
    fs::write(
        java_path.join("ListenerApplication.java"),
        generate_listener_application(&config.group_id),
    )
    .expect("无法创建ListenerApplication.java");

    // 创建配置文件
    let resources_path = module_path.join("src/main/resources");
    fs::create_dir_all(&resources_path).expect("无法创建resources目录");
    fs::write(
        resources_path.join("application.yml"),
        generate_application_yml(),
    )
    .expect("无法创建application.yml");
}

fn create_analysis_module(base_path: &Path, config: &ProjectConfig) {
    let module_path = base_path.join(format!("{}-analysis", config.name));
    create_module_structure(&module_path);

    // 创建 pom.xml
    fs::write(
        module_path.join("pom.xml"),
        generate_module_pom(&config, "analysis"),
    )
    .expect("无法创建analysis pom.xml");

    // 创建Java代码目录
    let java_path = module_path.join("src/main/java/com/example/analysis");
    fs::create_dir_all(&java_path).expect("无法创建analysis包路径");

    // 创建应用主类
    fs::write(
        java_path.join("AnalysisApplication.java"),
        generate_analysis_application(&config.group_id),
    )
    .expect("无法创建AnalysisApplication.java");

    // 创建服务类
    let service_path = java_path.join("service");
    fs::create_dir_all(&service_path).expect("无法创建service包路径");
    fs::write(
        service_path.join("SparkAnalysisService.java"),
        generate_spark_service(&config.group_id),
    )
    .expect("无法创建SparkAnalysisService.java");

    // 创建配置文件
    let resources_path = module_path.join("src/main/resources");
    fs::create_dir_all(&resources_path).expect("无法创建resources目录");
    fs::write(
        resources_path.join("application.yml"),
        generate_application_yml(),
    )
    .expect("无法创建application.yml");
    fs::write(
        resources_path.join("logback-spring.xml"),
        generate_logback_config(),
    )
    .expect("无法创建logback-spring.xml");
}

fn create_backend_module(base_path: &Path, config: &ProjectConfig) {
    let module_path = base_path.join(format!("{}-backend", config.name));
    create_module_structure(&module_path);

    // 创建 pom.xml
    fs::write(
        module_path.join("pom.xml"),
        generate_module_pom(config, "backend"),
    )
    .expect("无法创建backend pom.xml");

    // 创建Java包路径
    let package_path = config.group_id.replace(".", "/");
    let java_path = module_path.join(format!("src/main/java/{}/backend", package_path));
    fs::create_dir_all(&java_path).expect("无法创建backend包路径");

    // 创建主应用类
    fs::write(
        java_path.join("BackendApplication.java"),
        generate_backend_application(&config.group_id),
    )
    .expect("无法创建BackendApplication.java");

    // 创建控制器目录和示例控制器
    let controller_path = java_path.join("controller");
    fs::create_dir_all(&controller_path).expect("无法创建controller包路径");
    fs::write(
        controller_path.join("UserController.java"),
        generate_user_controller(&config.group_id),
    )
    .expect("无法创建UserController.java");

    // 创建配置文件
    let resources_path = module_path.join("src/main/resources");
    fs::create_dir_all(&resources_path).expect("无法创建resources目录");
    fs::write(
        resources_path.join("application.yml"),
        generate_application_yml(),
    )
    .expect("无法创建application.yml");
}

// ... 类似地实现其他模块的创建函数
