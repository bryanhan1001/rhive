use anyhow::{anyhow, Result};
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Bound, PyRefMut};
use pyo3_polars::PyDataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

/// Hive连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HiveConfig {
    #[pyo3(get, set)]
    pub host: String,
    #[pyo3(get, set)]
    pub port: u16,
    #[pyo3(get, set)]
    pub username: String,
    #[pyo3(get, set)]
    pub database: String,
    #[pyo3(get, set)]
    pub auth: String,
}

#[pymethods]
impl HiveConfig {
    #[new]
    fn new(
        host: Option<String>,
        port: Option<u16>,
        username: Option<String>,
        database: Option<String>,
        auth: Option<String>,
    ) -> Self {
        Self {
            host: host.unwrap_or_else(|| "localhost".to_string()),
            port: port.unwrap_or(10000),
            username: username.unwrap_or_else(|| "default".to_string()),
            database: database.unwrap_or_else(|| "default".to_string()),
            auth: auth.unwrap_or_else(|| "NONE".to_string()),
        }
    }

    fn __repr__(&self) -> String {
        let host = &self.host;
        let port = self.port;
        let username = &self.username;
        let database = &self.database;
        let auth = &self.auth;
        format!("HiveConfig(host='{host}', port={port}, username='{username}', database='{database}', auth='{auth}')")
    }
}

/// Rust版本的Hive数据读取器
#[derive(Debug)]
#[pyclass]
pub struct RustHiveReader {
    config: HiveConfig,
    connected: bool,
}

#[pymethods]
impl RustHiveReader {
    #[new]
    fn new(config: Option<HiveConfig>) -> Self {
        let config = config.unwrap_or_else(|| HiveConfig::new(None, None, None, None, None));
        Self {
            config,
            connected: false,
        }
    }

    /// 连接到Hive
    fn connect(&mut self) -> PyResult<()> {
        let host = &self.config.host;
        let port = self.config.port;
        println!("🔗 连接到Hive: {host}:{port}");

        // 这里实现实际的连接逻辑
        // 为了演示，我们模拟连接成功
        self.connected = true;
        println!("✅ Hive连接成功 (Rust版本)");
        Ok(())
    }

    /// 断开连接
    fn disconnect(&mut self) -> PyResult<()> {
        if self.connected {
            println!("🔌 断开Hive连接");
            self.connected = false;
        }
        Ok(())
    }

    /// 检查连接状态
    fn is_connected(&self) -> bool {
        self.connected
    }

    /// 执行SQL查询并返回Polars DataFrame
    fn query_to_polars(&self, sql: String) -> PyResult<PyDataFrame> {
        if !self.connected {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "未连接到Hive，请先调用connect()",
            ));
        }

        let preview = &sql[..std::cmp::min(sql.len(), 50)];
        println!("🔍 执行SQL查询: {preview}");

        // 这里调用实际的查询实现
        let df = self
            .execute_sql_query(&sql)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(PyDataFrame(df))
    }

    /// 显示所有表
    fn show_tables(&self) -> PyResult<PyDataFrame> {
        self.query_to_polars("SHOW TABLES".to_string())
    }

    /// 描述表结构
    fn describe_table(&self, table_name: String) -> PyResult<PyDataFrame> {
        let sql = format!("DESCRIBE {table_name}");
        self.query_to_polars(sql)
    }

    /// 获取表样本数据
    fn get_table_sample(&self, table_name: String, limit: Option<i32>) -> PyResult<PyDataFrame> {
        let limit = limit.unwrap_or(10);
        let sql = format!("SELECT * FROM {table_name} LIMIT {limit}");
        self.query_to_polars(sql)
    }

    /// 获取配置信息
    fn get_config(&self) -> HiveConfig {
        self.config.clone()
    }
}

impl RustHiveReader {
    /// 执行SQL查询的内部实现
    fn execute_sql_query(&self, sql: &str) -> Result<DataFrame> {
        // 方案1: 使用beeline命令行客户端
        if std::env::var("USE_BEELINE").unwrap_or_default() == "true" {
            return self.execute_via_beeline(sql);
        }

        // 方案2: 模拟数据（用于演示和测试）
        self.execute_mock_query(sql)
    }

    /// 通过beeline执行查询
    fn execute_via_beeline(&self, sql: &str) -> Result<DataFrame> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let host = &self.config.host;
            let port = self.config.port;
            let database = &self.config.database;
            let jdbc_url = format!("jdbc:hive2://{host}:{port}/{database}");

            let output = Command::new("beeline")
                .args([
                    "-u",
                    &jdbc_url,
                    "-e",
                    sql,
                    "--outputformat=csv2",
                    "--silent=true",
                ])
                .output()
                .await?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!("Beeline执行失败: {error}"));
            }

            let csv_data = String::from_utf8_lossy(&output.stdout);
            self.parse_csv_to_dataframe(&csv_data)
        })
    }

    /// 模拟查询执行（用于演示）
    fn execute_mock_query(&self, sql: &str) -> Result<DataFrame> {
        println!("📊 模拟执行SQL: {sql}");

        // 根据SQL类型返回不同的模拟数据
        if sql.to_uppercase().contains("SHOW TABLES") {
            self.create_mock_tables_df()
        } else if sql.to_uppercase().contains("DESCRIBE") {
            self.create_mock_describe_df()
        } else if sql.to_uppercase().contains("SELECT") {
            self.create_mock_select_df(sql)
        } else {
            Err(anyhow!("不支持的SQL类型"))
        }
    }

    /// 创建模拟表列表
    fn create_mock_tables_df(&self) -> Result<DataFrame> {
        let tables = vec![
            "users".to_string(),
            "orders".to_string(),
            "products".to_string(),
            "analytics".to_string(),
            "logs".to_string(),
        ];

        let df = df! {
            "tab_name" => tables,
        }?;

        Ok(df)
    }

    /// 创建模拟表结构描述
    fn create_mock_describe_df(&self) -> Result<DataFrame> {
        let df = df! {
            "col_name" => vec!["id", "name", "created_at", "status"],
            "data_type" => vec!["bigint", "string", "timestamp", "string"],
            "comment" => vec!["Primary key", "User name", "Creation time", "Record status"],
        }?;

        Ok(df)
    }

    /// 创建模拟查询结果
    fn create_mock_select_df(&self, sql: &str) -> Result<DataFrame> {
        // 根据SQL生成不同的模拟数据
        if sql.to_uppercase().contains("COUNT") {
            df! {
                "count" => vec![1000i64, 2000, 3000],
                "table_name" => vec!["table1", "table2", "table3"],
            }
            .map_err(|e| anyhow!("创建DataFrame失败: {e}"))
        } else {
            // 通用的示例数据
            df! {
                "id" => vec![1i64, 2, 3, 4, 5],
                "name" => vec!["Alice", "Bob", "Charlie", "David", "Eve"],
                "score" => vec![95.5, 87.2, 92.1, 78.9, 88.7],
                "created_at" => vec![
                    "2025-01-01 10:00:00",
                    "2025-01-02 11:00:00",
                    "2025-01-03 12:00:00",
                    "2025-01-04 13:00:00",
                    "2025-01-05 14:00:00",
                ],
            }
            .map_err(|e| anyhow!("创建DataFrame失败: {e}"))
        }
    }

    /// 解析CSV数据为DataFrame
    fn parse_csv_to_dataframe(&self, csv_data: &str) -> Result<DataFrame> {
        // 这里实现CSV解析逻辑
        // 为简单起见，这里返回一个示例DataFrame
        let df = df! {
            "data" => vec![csv_data],
        }?;
        Ok(df)
    }
}

/// 上下文管理器支持
#[pyclass]
pub struct RustHiveContext {
    reader: RustHiveReader,
}

#[pymethods]
impl RustHiveContext {
    #[new]
    fn new(config: Option<HiveConfig>) -> Self {
        Self {
            reader: RustHiveReader::new(config),
        }
    }

    fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        slf.reader.connect()?;
        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_value: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        self.reader.disconnect()?;
        Ok(false) // 不抑制异常
    }
}

/// 便捷函数：创建Hive配置
#[pyfunction]
fn create_hive_config(
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    database: Option<String>,
    auth: Option<String>,
) -> HiveConfig {
    HiveConfig::new(host, port, username, database, auth)
}

/// 便捷函数：从字典创建配置
#[pyfunction]
fn config_from_dict(config_dict: &Bound<'_, PyDict>) -> PyResult<HiveConfig> {
    let host = config_dict
        .get_item("host")?
        .map(|v| v.extract::<String>())
        .transpose()?;
    let port = config_dict
        .get_item("port")?
        .map(|v| v.extract::<u16>())
        .transpose()?;
    let username = config_dict
        .get_item("username")?
        .map(|v| v.extract::<String>())
        .transpose()?;
    let database = config_dict
        .get_item("database")?
        .map(|v| v.extract::<String>())
        .transpose()?;
    let auth = config_dict
        .get_item("auth")?
        .map(|v| v.extract::<String>())
        .transpose()?;

    Ok(HiveConfig::new(host, port, username, database, auth))
}

/// 性能基准测试函数
#[pyfunction]
fn benchmark_query(
    config: HiveConfig,
    sql: String,
    iterations: Option<usize>,
) -> PyResult<HashMap<String, f64>> {
    let iterations = iterations.unwrap_or(10);
    let mut reader = RustHiveReader::new(Some(config));
    reader.connect()?;

    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _result = reader.query_to_polars(sql.clone())?;
    }

    let duration = start.elapsed();
    let avg_time = duration.as_secs_f64() / iterations as f64;

    reader.disconnect()?;

    let mut results = HashMap::new();
    results.insert("total_time".to_string(), duration.as_secs_f64());
    results.insert("average_time".to_string(), avg_time);
    results.insert("iterations".to_string(), iterations as f64);
    results.insert(
        "queries_per_second".to_string(),
        iterations as f64 / duration.as_secs_f64(),
    );

    Ok(results)
}

/// Python模块定义
#[pymodule]
fn hive_reader_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 类
    m.add_class::<HiveConfig>()?;
    m.add_class::<RustHiveReader>()?;
    m.add_class::<RustHiveContext>()?;

    // 函数
    m.add_function(wrap_pyfunction!(create_hive_config, m)?)?;
    m.add_function(wrap_pyfunction!(config_from_dict, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_query, m)?)?;

    // 版本信息
    m.add("__version__", "0.1.0")?;
    m.add("__author__", "Hive Reader Rust")?;

    Ok(())
}
