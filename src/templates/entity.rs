pub fn generate_base_entity(group_id: &str) -> String {
    format!(
        r#"package {}.domain.entity;

import lombok.Data;
import javax.persistence.Id;
import javax.persistence.MappedSuperclass;
import java.time.LocalDateTime;

@Data
@MappedSuperclass
public class BaseEntity {{
    @Id
    private Long id;

    private LocalDateTime createTime;

    private LocalDateTime updateTime;

    private String createBy;

    private String updateBy;
}}"#,
        group_id
    )
    .to_string()
}

pub fn generate_user_entity(group_id: &str) -> String {
    format!(
        r#"package {}.entity;

import jakarta.persistence.Entity;
import jakarta.persistence.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

@Data
@Entity
@Table(name = "t_user")
@EqualsAndHashCode(callSuper = true)
public class User extends BaseEntity {{
    private String username;
    private String email;
    private String phone;
    private Integer status;
}}
"#,
        group_id
    )
}
