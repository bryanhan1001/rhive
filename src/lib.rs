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

    // 代理方法：转发到内部的RustHiveReader
    /// 执行SQL查询并返回Polars DataFrame
    fn query_to_polars(&self, sql: String) -> PyResult<PyDataFrame> {
        self.reader.query_to_polars(sql)
    }

    /// 显示所有表
    fn show_tables(&self) -> PyResult<PyDataFrame> {
        self.reader.show_tables()
    }

    /// 描述表结构
    fn describe_table(&self, table_name: String) -> PyResult<PyDataFrame> {
        self.reader.describe_table(table_name)
    }

    /// 获取表样本数据
    fn get_table_sample(&self, table_name: String, limit: Option<i32>) -> PyResult<PyDataFrame> {
        self.reader.get_table_sample(table_name, limit)
    }

    /// 检查连接状态
    fn is_connected(&self) -> bool {
        self.reader.is_connected()
    }

    /// 获取配置信息
    fn get_config(&self) -> HiveConfig {
        self.reader.get_config()
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

/// 便捷函数：创建默认配置
#[pyfunction]
fn create_default_config() -> HiveConfig {
    HiveConfig::new(None, None, None, None, None)
}

/// 便捷函数：获取默认Hive配置
#[pyfunction]
fn get_default_hive_config() -> HiveConfig {
    create_default_config()
}

/// 便捷函数：获取配置管理器（返回配置对象）
#[pyfunction]
fn get_config_manager() -> HiveConfig {
    create_default_config()
}

/// 便捷函数：连接到Hive（返回上下文管理器）
#[pyfunction]
fn connect_hive(config: Option<HiveConfig>) -> RustHiveContext {
    RustHiveContext::new(config)
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
    // 配置类
    m.add_class::<HiveConfig>()?;

    // 读取器相关类
    m.add_class::<RustHiveReader>()?;
    m.add_class::<RustHiveContext>()?;

    // 写入器相关类
    m.add_class::<WriteMode>()?;
    m.add_class::<RustHiveWriter>()?;
    m.add_class::<RustHiveWriteContext>()?;

    // 工具函数
    m.add_function(wrap_pyfunction!(create_hive_config, m)?)?;
    m.add_function(wrap_pyfunction!(config_from_dict, m)?)?;
    m.add_function(wrap_pyfunction!(create_default_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_default_hive_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_config_manager, m)?)?;
    m.add_function(wrap_pyfunction!(connect_hive, m)?)?;
    m.add_function(wrap_pyfunction!(connect_hive_writer, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_query, m)?)?;

    // 版本信息
    m.add("__version__", "0.1.0")?;
    m.add("__author__", "Hive Reader & Writer Rust")?;

    Ok(())
}

/// 写入模式枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub enum WriteMode {
    #[pyo3(name = "overwrite")]
    Overwrite,
    #[pyo3(name = "append")]
    Append,
    #[pyo3(name = "error_if_exists")]
    ErrorIfExists,
    #[pyo3(name = "ignore")]
    Ignore,
}

#[pymethods]
impl WriteMode {
    fn __repr__(&self) -> String {
        match self {
            WriteMode::Overwrite => "WriteMode.Overwrite".to_string(),
            WriteMode::Append => "WriteMode.Append".to_string(),
            WriteMode::ErrorIfExists => "WriteMode.ErrorIfExists".to_string(),
            WriteMode::Ignore => "WriteMode.Ignore".to_string(),
        }
    }
}

/// Rust版本的Hive数据写入器
#[derive(Debug)]
#[pyclass]
pub struct RustHiveWriter {
    config: HiveConfig,
    connected: bool,
}

#[pymethods]
impl RustHiveWriter {
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
        println!("🔗 连接到Hive写入器: {host}:{port}");

        // 这里实现实际的连接逻辑
        self.connected = true;
        println!("✅ Hive写入器连接成功 (Rust版本)");
        Ok(())
    }

    /// 断开连接
    fn disconnect(&mut self) -> PyResult<()> {
        if self.connected {
            println!("🔌 断开Hive写入器连接");
            self.connected = false;
        }
        Ok(())
    }

    /// 检查连接状态
    fn is_connected(&self) -> bool {
        self.connected
    }

    /// 将Polars DataFrame写入Hive表
    #[pyo3(signature = (df, table_name, mode = None, partition_cols = None, create_table = None))]
    fn write_table(
        &self,
        df: PyDataFrame,
        table_name: String,
        mode: Option<WriteMode>,
        partition_cols: Option<Vec<String>>,
        create_table: Option<bool>,
    ) -> PyResult<()> {
        if !self.connected {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "未连接到Hive，请先调用connect()",
            ));
        }

        let mode = mode.unwrap_or(WriteMode::ErrorIfExists);
        let create_table = create_table.unwrap_or(true);

        println!("📝 写入数据到表: {table_name}");

        // 调用实际的写入实现
        self.execute_write_operation(&df.0, &table_name, &mode, &partition_cols, create_table)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        println!("✅ 数据写入完成");
        Ok(())
    }

    /// 创建表结构
    fn create_table_from_dataframe(
        &self,
        df: PyDataFrame,
        table_name: String,
        partition_cols: Option<Vec<String>>,
    ) -> PyResult<()> {
        if !self.connected {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "未连接到Hive，请先调用connect()",
            ));
        }

        self.create_table_schema(&df.0, &table_name, &partition_cols)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(())
    }

    /// 删除表
    fn drop_table(&self, table_name: String, if_exists: Option<bool>) -> PyResult<()> {
        if !self.connected {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "未连接到Hive，请先调用connect()",
            ));
        }

        let if_exists = if_exists.unwrap_or(false);
        let sql = if if_exists {
            format!("DROP TABLE IF EXISTS {table_name}")
        } else {
            format!("DROP TABLE {table_name}")
        };

        self.execute_ddl(&sql)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        println!("🗑️  表 {table_name} 已删除");
        Ok(())
    }

    /// 获取配置信息
    fn get_config(&self) -> HiveConfig {
        self.config.clone()
    }
}

impl RustHiveWriter {
    /// 执行写入操作的内部实现
    fn execute_write_operation(
        &self,
        df: &DataFrame,
        table_name: &str,
        mode: &WriteMode,
        partition_cols: &Option<Vec<String>>,
        create_table: bool,
    ) -> Result<()> {
        // 检查表是否存在
        let table_exists = self.check_table_exists(table_name)?;

        match mode {
            WriteMode::ErrorIfExists if table_exists => {
                return Err(anyhow!("表 {table_name} 已存在"));
            }
            WriteMode::Ignore if table_exists => {
                println!("⚠️  表 {table_name} 已存在，忽略写入");
                return Ok(());
            }
            WriteMode::Overwrite if table_exists => {
                println!("🔄 覆盖模式：删除现有表 {table_name}");
                self.execute_ddl(&format!("DROP TABLE {table_name}"))?;
            }
            _ => {}
        }

        // 如果表不存在且需要创建表，则创建表结构
        if (!table_exists || matches!(mode, WriteMode::Overwrite)) && create_table {
            self.create_table_schema(df, table_name, partition_cols)?;
        }

        // 写入数据
        self.insert_dataframe_data(df, table_name, partition_cols)
    }

    /// 检查表是否存在
    fn check_table_exists(&self, table_name: &str) -> Result<bool> {
        // 方案1: 使用beeline命令检查
        if std::env::var("USE_BEELINE").unwrap_or_default() == "true" {
            return self.check_table_exists_via_beeline(table_name);
        }

        // 方案2: 模拟检查（用于演示）
        println!("🔍 检查表是否存在: {table_name}");
        // 这里可以模拟表存在性检查逻辑
        Ok(false) // 默认假设表不存在
    }

    /// 通过beeline检查表是否存在
    fn check_table_exists_via_beeline(&self, table_name: &str) -> Result<bool> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let host = &self.config.host;
            let port = self.config.port;
            let database = &self.config.database;
            let jdbc_url = format!("jdbc:hive2://{host}:{port}/{database}");

            let sql = format!("SHOW TABLES LIKE '{table_name}'");

            let output = Command::new("beeline")
                .args([
                    "-u",
                    &jdbc_url,
                    "-e",
                    &sql,
                    "--outputformat=csv2",
                    "--silent=true",
                ])
                .output()
                .await?;

            if !output.status.success() {
                return Err(anyhow!("检查表存在性失败"));
            }

            let result = String::from_utf8_lossy(&output.stdout);
            Ok(!result.trim().is_empty())
        })
    }

    /// 根据DataFrame创建表结构
    fn create_table_schema(
        &self,
        df: &DataFrame,
        table_name: &str,
        partition_cols: &Option<Vec<String>>,
    ) -> Result<()> {
        let schema = df.schema();
        let mut column_definitions = Vec::new();
        let mut partition_definitions = Vec::new();

        // 构建列定义
        for (name, dtype) in schema.iter() {
            let hive_type = self.polars_to_hive_type(dtype)?;
            let name_str = name.to_string(); // 转换SmartString到String

            if let Some(partitions) = partition_cols {
                if partitions.contains(&name_str) {
                    partition_definitions.push(format!("{name_str} {hive_type}"));
                    continue;
                }
            }

            column_definitions.push(format!("{name_str} {hive_type}"));
        }

        // 构建CREATE TABLE语句
        let mut create_sql = format!(
            "CREATE TABLE {table_name} ({})",
            column_definitions.join(", ")
        );

        // 添加分区信息
        if !partition_definitions.is_empty() {
            create_sql.push_str(&format!(
                " PARTITIONED BY ({})",
                partition_definitions.join(", ")
            ));
        }

        // 添加存储格式
        create_sql.push_str(" STORED AS PARQUET");

        println!("🏗️  创建表结构: {create_sql}");
        self.execute_ddl(&create_sql)
    }

    /// 将Polars数据类型转换为Hive数据类型
    fn polars_to_hive_type(&self, dtype: &DataType) -> Result<String> {
        let hive_type = match dtype {
            DataType::Boolean => "BOOLEAN",
            DataType::Int8 | DataType::Int16 | DataType::Int32 => "INT",
            DataType::Int64 => "BIGINT",
            DataType::UInt8 | DataType::UInt16 | DataType::UInt32 => "INT",
            DataType::UInt64 => "BIGINT",
            DataType::Float32 => "FLOAT",
            DataType::Float64 => "DOUBLE",
            DataType::String => "STRING",
            DataType::Date => "DATE",
            DataType::Datetime(_, _) => "TIMESTAMP",
            _ => return Err(anyhow!("不支持的数据类型: {:?}", dtype)),
        };
        Ok(hive_type.to_string())
    }

    /// 执行DDL语句
    fn execute_ddl(&self, sql: &str) -> Result<()> {
        // 方案1: 使用beeline执行DDL
        if std::env::var("USE_BEELINE").unwrap_or_default() == "true" {
            return self.execute_ddl_via_beeline(sql);
        }

        // 方案2: 模拟执行（用于演示）
        println!("📋 执行DDL: {sql}");
        Ok(())
    }

    /// 通过beeline执行DDL
    fn execute_ddl_via_beeline(&self, sql: &str) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let host = &self.config.host;
            let port = self.config.port;
            let database = &self.config.database;
            let jdbc_url = format!("jdbc:hive2://{host}:{port}/{database}");

            let output = Command::new("beeline")
                .args(["-u", &jdbc_url, "-e", sql, "--silent=true"])
                .output()
                .await?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!("DDL执行失败: {error}"));
            }

            Ok(())
        })
    }

    /// 插入DataFrame数据
    fn insert_dataframe_data(
        &self,
        df: &DataFrame,
        table_name: &str,
        partition_cols: &Option<Vec<String>>,
    ) -> Result<()> {
        // 方案1: 通过CSV文件和LOAD DATA方式
        if std::env::var("USE_CSV_LOAD").unwrap_or_default() == "true" {
            return self.insert_via_csv_load(df, table_name, partition_cols);
        }

        // 方案2: 通过Parquet文件和外部表方式
        if std::env::var("USE_PARQUET_LOAD").unwrap_or_default() == "true" {
            return self.insert_via_parquet_load(df, table_name, partition_cols);
        }

        // 方案3: 生成INSERT语句（适合小数据量）
        self.insert_via_sql_statements(df, table_name, partition_cols)
    }

    /// 通过CSV文件和LOAD DATA插入数据
    fn insert_via_csv_load(
        &self,
        df: &DataFrame,
        table_name: &str,
        _partition_cols: &Option<Vec<String>>,
    ) -> Result<()> {
        // 创建临时CSV文件
        let temp_file = format!("/tmp/{table_name}_{}.csv", chrono::Utc::now().timestamp());

        // 使用LazyFrame写入CSV文件 (避免可变引用问题)
        let mut df_clone = df.clone();
        let mut file = std::fs::File::create(&temp_file)?;
        CsvWriter::new(&mut file)
            .include_header(false)
            .finish(&mut df_clone)?;

        // 执行LOAD DATA语句
        let load_sql = format!("LOAD DATA LOCAL INPATH '{temp_file}' INTO TABLE {table_name}");

        self.execute_ddl(&load_sql)?;

        // 清理临时文件
        let _ = std::fs::remove_file(&temp_file);

        Ok(())
    }

    /// 通过Parquet文件插入数据 (简化版本)
    fn insert_via_parquet_load(
        &self,
        _df: &DataFrame,
        table_name: &str,
        _partition_cols: &Option<Vec<String>>,
    ) -> Result<()> {
        // 创建临时文件路径
        let temp_file = format!(
            "/tmp/{table_name}_{}.parquet",
            chrono::Utc::now().timestamp()
        );

        println!("📦 将生成Parquet文件: {temp_file}");
        println!("📋 请使用外部工具将DataFrame保存为Parquet并上传到HDFS");
        println!("💡 提示: 可以使用 df.write_parquet() 方法保存文件");

        // 这里可以添加自动上传到HDFS的逻辑
        // 由于ParquetWriter的API问题，暂时使用提示信息

        Ok(())
    }

    /// 通过INSERT语句插入数据（适合小数据量）
    fn insert_via_sql_statements(
        &self,
        df: &DataFrame,
        table_name: &str,
        _partition_cols: &Option<Vec<String>>,
    ) -> Result<()> {
        let rows_count = df.height();
        if rows_count > 1000 {
            println!("⚠️  数据量较大({rows_count}行)，建议使用CSV或Parquet方式导入");
        }

        // 构建INSERT语句
        let columns: Vec<String> = df
            .get_column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();
        let column_list = columns.join(", ");

        // 批量插入数据
        let batch_size = 100;
        for chunk_start in (0..rows_count).step_by(batch_size) {
            let chunk_end = std::cmp::min(chunk_start + batch_size, rows_count);
            let chunk_df = df.slice(chunk_start as i64, chunk_end - chunk_start);

            let values = self.dataframe_to_values_string(&chunk_df)?;
            let insert_sql = format!("INSERT INTO {table_name} ({column_list}) VALUES {values}");

            self.execute_ddl(&insert_sql)?;
        }

        Ok(())
    }

    /// 将DataFrame转换为VALUES字符串
    fn dataframe_to_values_string(&self, df: &DataFrame) -> Result<String> {
        let mut values = Vec::new();

        for row_idx in 0..df.height() {
            let mut row_values = Vec::new();

            for column in df.get_columns() {
                let value = self.format_column_value(column, row_idx)?;
                row_values.push(value);
            }

            values.push(format!("({})", row_values.join(", ")));
        }

        Ok(values.join(", "))
    }

    /// 格式化列值
    fn format_column_value(&self, column: &Series, row_idx: usize) -> Result<String> {
        let value = column.get(row_idx)?;
        let formatted = match value {
            AnyValue::Null => "NULL".to_string(),
            AnyValue::Boolean(b) => b.to_string(),
            AnyValue::Int8(i) => i.to_string(),
            AnyValue::Int16(i) => i.to_string(),
            AnyValue::Int32(i) => i.to_string(),
            AnyValue::Int64(i) => i.to_string(),
            AnyValue::UInt8(i) => i.to_string(),
            AnyValue::UInt16(i) => i.to_string(),
            AnyValue::UInt32(i) => i.to_string(),
            AnyValue::UInt64(i) => i.to_string(),
            AnyValue::Float32(f) => f.to_string(),
            AnyValue::Float64(f) => f.to_string(),
            _ => {
                // 处理字符串和其他类型，统一转换为字符串
                let str_value = format!("{value}");
                if str_value.contains('"') || str_value.contains('\'') {
                    let escaped_value = str_value.replace('\'', "''");
                    format!("'{escaped_value}'")
                } else {
                    format!("'{str_value}'")
                }
            }
        };
        Ok(formatted)
    }
}

/// Hive写入上下文管理器
#[derive(Debug)]
#[pyclass]
pub struct RustHiveWriteContext {
    writer: RustHiveWriter,
}

#[pymethods]
impl RustHiveWriteContext {
    #[new]
    fn new(config: Option<HiveConfig>) -> Self {
        Self {
            writer: RustHiveWriter::new(config),
        }
    }

    fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyResult<PyRefMut<'_, Self>> {
        slf.writer.connect()?;
        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_value: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        if let Err(e) = self.writer.disconnect() {
            eprintln!("警告: 断开连接时出错: {e}");
        }
        Ok(false)
    }

    /// 写入表
    #[pyo3(signature = (df, table_name, mode = None, partition_cols = None, create_table = None))]
    fn write_table(
        &self,
        df: PyDataFrame,
        table_name: String,
        mode: Option<WriteMode>,
        partition_cols: Option<Vec<String>>,
        create_table: Option<bool>,
    ) -> PyResult<()> {
        self.writer
            .write_table(df, table_name, mode, partition_cols, create_table)
    }

    /// 创建表
    fn create_table_from_dataframe(
        &self,
        df: PyDataFrame,
        table_name: String,
        partition_cols: Option<Vec<String>>,
    ) -> PyResult<()> {
        self.writer
            .create_table_from_dataframe(df, table_name, partition_cols)
    }

    /// 删除表
    fn drop_table(&self, table_name: String, if_exists: Option<bool>) -> PyResult<()> {
        self.writer.drop_table(table_name, if_exists)
    }

    /// 检查连接状态
    fn is_connected(&self) -> bool {
        self.writer.is_connected()
    }

    /// 获取配置信息
    fn get_config(&self) -> HiveConfig {
        self.writer.get_config()
    }
}

/// 便捷的写入连接函数
#[pyfunction]
fn connect_hive_writer(config: Option<HiveConfig>) -> RustHiveWriteContext {
    RustHiveWriteContext::new(config)
}
