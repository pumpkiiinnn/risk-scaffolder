// 导出所有子模块
pub mod pom;
pub mod entity;
pub mod backend;
pub mod listener;
pub mod analysis;
pub mod common;

// 从 pom 模块导出所有 pom 生成函数
pub use pom::{
    generate_root_pom,
    generate_module_pom,
};

// 从 entity 模块导出实体生成函数
pub use entity::{
    generate_base_entity,
    generate_user_entity,
};

// 从 backend 模块导出后端相关函数
pub use backend::{
    generate_backend_application,
    generate_user_controller,
};

// 从 listener 模块导出监听器相关函数
pub use listener::{
    generate_listener_application,
    generate_redis_listener,
};

// 从 analysis 模块导出分析相关函数
pub use analysis::{
    generate_analysis_application,
    generate_spark_service,
    generate_data_processor,
};

// 从 common 模块导出通用函数
pub use common::{
    generate_logback_config,
    generate_application_yml,
}; 