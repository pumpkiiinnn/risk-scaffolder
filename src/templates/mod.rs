pub fn generate_root_pom(project_name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.example</groupId>
    <artifactId>{}</artifactId>
    <version>1.0-SNAPSHOT</version>
    <packaging>pom</packaging>

    <modules>
        <module>{}-entity</module>
        <module>{}-backend</module>
        <module>{}-listener</module>
        <module>{}-analysis</module>
    </modules>
</project>"#,
        project_name, project_name, project_name, project_name, project_name
    )
}

pub fn generate_module_pom(project_name: &str, module_type: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <parent>
        <groupId>com.example</groupId>
        <artifactId>{}</artifactId>
        <version>1.0-SNAPSHOT</version>
    </parent>

    <artifactId>{}-{}</artifactId>
</project>"#,
        project_name, project_name, module_type
    )
}

pub fn generate_logback_config() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<configuration>
    <appender name="CONSOLE" class="ch.qos.logback.core.ConsoleAppender">
        <encoder>
            <pattern>%d{yyyy-MM-dd HH:mm:ss.SSS} [%thread] %-5level %logger{36} - %msg%n</pattern>
        </encoder>
    </appender>

    <root level="INFO">
        <appender-ref ref="CONSOLE" />
    </root>
</configuration>"#
        .to_string()
}

pub fn generate_application_yml() -> String {
    r#"spring:
  application:
    name: example-service
  
server:
  port: 8080
  
logging:
  level:
    root: INFO"#
        .to_string()
}
