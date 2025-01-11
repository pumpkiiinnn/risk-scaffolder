pub fn generate_analysis_application(group_id: &str) -> String {
    format!(
        r#"package {}.analysis;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;

@SpringBootApplication
@EntityScan("{}.entity")
public class AnalysisApplication {{
    public static void main(String[] args) {{
        SpringApplication.run(AnalysisApplication.class, args);
    }}
}}"#,
        group_id, group_id
    )
}

pub fn generate_spark_service(group_id: &str) -> String {
    format!(
        r#"package {}.analysis.service;

import org.apache.spark.sql.SparkSession;
import org.springframework.stereotype.Service;
import javax.annotation.PostConstruct;
import javax.annotation.PreDestroy;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Service
public class SparkAnalysisService {{
    private SparkSession sparkSession;
    
    @PostConstruct
    public void init() {{
        sparkSession = SparkSession.builder()
                .appName("DataAnalysis")
                .master("local[*]")
                .getOrCreate();
        log.info("SparkSession initialized");
    }}
    
    public void performAnalysis(String data) {{
        // TODO: 实现数据分析逻辑
        log.info("Performing analysis on data: {{}}", data);
    }}
    
    @PreDestroy
    public void cleanup() {{
        if (sparkSession != null) {{
            sparkSession.close();
            log.info("SparkSession closed");
        }}
    }}
}}"#,
        group_id
    )
}

pub fn generate_data_processor(group_id: &str) -> String {
    format!(
        r#"package {}.analysis.processor;

import org.springframework.stereotype.Component;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Component
public class DataProcessor {{
    public void processData(String data) {{
        log.info("Processing data: {{}}", data);
        // TODO: 实现数据处理逻辑
    }}
}}"#,
        group_id
    )
} 