pub fn generate_listener_application(group_id: &str) -> String {
    format!(
        r#"package {}.listener;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;

@SpringBootApplication
@EntityScan("{}.entity")
public class ListenerApplication {{
    public static void main(String[] args) {{
        SpringApplication.run(ListenerApplication.class, args);
    }}
}}"#,
        group_id, group_id
    )
}

pub fn generate_redis_listener(group_id: &str) -> String {
    format!(
        r#"package {}.listener.redis;

import org.springframework.data.redis.connection.Message;
import org.springframework.data.redis.connection.MessageListener;
import org.springframework.stereotype.Component;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
public class RedisMessageListener implements MessageListener {{
    
    @Override
    public void onMessage(Message message, byte[] pattern) {{
        log.info("Received Redis message: {{}}", new String(message.getBody()));
        // TODO: 处理消息
    }}
}}"#,
        group_id
    )
} 