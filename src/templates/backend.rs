pub fn generate_backend_application(group_id: &str) -> String {
    format!(
        r#"package {}.backend;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.data.jpa.repository.config.EnableJpaAuditing;

@SpringBootApplication
@EntityScan("{}.entity")
@EnableJpaAuditing
public class BackendApplication {{
    public static void main(String[] args) {{
        SpringApplication.run(BackendApplication.class, args);
    }}
}}"#,
        group_id, group_id
    )
}

pub fn generate_user_controller(group_id: &str) -> String {
    format!(
        r#"package {}.backend.controller;

import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.GetMapping;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@RestController
@RequestMapping("/api/v1")
public class UserController {{
    
    @GetMapping("/hello")
    public String hello() {{
        return "Hello World!";
    }}
}}"#,
        group_id
    )
} 