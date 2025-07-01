# Hive Writer 功能文档

## 概述

`hive_writer` 是 rhive 库的写入功能模块，提供高性能的 Polars DataFrame 到 Hive 表的写入能力。使用 Rust 实现核心逻辑，通过 PyO3 提供 Python 接口。

## 主要功能

- ✅ **DataFrame 写入**: 将 Polars DataFrame 直接写入 Hive 表
- ✅ **多种写入模式**: 支持覆盖、追加、错误检查、忽略等模式
- ✅ **分区表支持**: 支持创建和写入分区表
- ✅ **表管理**: 创建表结构、删除表等操作
- ✅ **多种数据格式**: 支持 CSV、Parquet、SQL INSERT 等写入方式
- ✅ **类型映射**: 自动将 Polars 数据类型映射为 Hive 数据类型
- ✅ **连接管理**: 上下文管理器确保连接正确处理

## 快速开始

### 基本使用

```python
import polars as pl
from hive_reader_rs import connect_writer, WriteMode

# 创建 DataFrame
df = pl.DataFrame({
    "id": [1, 2, 3],
    "name": ["Alice", "Bob", "Charlie"],
    "age": [25, 30, 35]
})

# 写入到 Hive 表
with connect_writer(host="localhost", port=10000) as writer:
    writer.write_table(df, "my_table")
```

### 不同写入模式

```python
from hive_reader_rs import WriteMode

with connect_writer() as writer:
    # 覆盖模式
    writer.write_table(df, "table1", mode=WriteMode.Overwrite)
    
    # 追加模式
    writer.write_table(df, "table1", mode=WriteMode.Append)
    
    # 如果表存在则报错（默认）
    writer.write_table(df, "table2", mode=WriteMode.ErrorIfExists)
    
    # 如果表存在则忽略
    writer.write_table(df, "table3", mode=WriteMode.Ignore)
```

## API 文档

### 写入模式 (WriteMode)

```python
class WriteMode:
    Overwrite        # 覆盖现有表
    Append          # 追加到现有表
    ErrorIfExists   # 表存在时报错（默认）
    Ignore          # 表存在时忽略操作
```

### 连接函数

#### connect_writer()

便捷的写入器连接函数，支持配置优先级。

```python
def connect_writer(host=None, port=None, username=None, database=None, auth=None):
    """
    Args:
        host: Hive服务器地址
        port: 端口号 (默认: 10000)
        username: 用户名 (默认: "default")
        database: 数据库名 (默认: "default")
        auth: 认证方式 (默认: "NONE")
        
    Returns:
        RustHiveWriteContext: 写入器上下文管理器
    """
```

### 写入器类

#### RustHiveWriter

核心写入器类，提供所有写入功能。

```python
class RustHiveWriter:
    def __init__(self, config: Optional[HiveConfig] = None)
    def connect(self) -> None
    def disconnect(self) -> None
    def is_connected(self) -> bool
    def get_config(self) -> HiveConfig
    
    def write_table(
        self,
        df: polars.DataFrame,
        table_name: str,
        mode: Optional[WriteMode] = None,
        partition_cols: Optional[List[str]] = None,
        create_table: Optional[bool] = None
    ) -> None
    
    def create_table_from_dataframe(
        self,
        df: polars.DataFrame,
        table_name: str,
        partition_cols: Optional[List[str]] = None
    ) -> None
    
    def drop_table(
        self,
        table_name: str,
        if_exists: Optional[bool] = None
    ) -> None
```

#### RustHiveWriteContext

上下文管理器版本的写入器，推荐使用。

```python
class RustHiveWriteContext:
    def __enter__(self) -> RustHiveWriteContext
    def __exit__(self, exc_type, exc_value, traceback) -> bool
    
    # 继承 RustHiveWriter 的所有方法
```

## 高级用法

### 分区表

```python
# 创建分区表
with connect_writer() as writer:
    writer.write_table(
        df=df,
        table_name="sales_data",
        partition_cols=["year", "month"],  # 按年月分区
        mode=WriteMode.Overwrite
    )
```

### 表管理

```python
with connect_writer() as writer:
    # 仅创建表结构
    writer.create_table_from_dataframe(df, "empty_table")
    
    # 删除表
    writer.drop_table("old_table", if_exists=True)
```

### 自定义配置

```python
from hive_reader_rs import HiveConfig, HiveWriter

config = HiveConfig(
    host="prod-hive.company.com",
    port=10000,
    username="analyst",
    database="analytics",
    auth="KERBEROS"
)

writer = HiveWriter(config)
writer.connect()
try:
    writer.write_table(df, "production_data")
finally:
    writer.disconnect()
```

## 数据类型映射

| Polars 类型 | Hive 类型 |
|------------|-----------|
| Boolean | BOOLEAN |
| Int8/Int16/Int32 | INT |
| Int64 | BIGINT |
| UInt8/UInt16/UInt32 | INT |
| UInt64 | BIGINT |
| Float32 | FLOAT |
| Float64 | DOUBLE |
| String | STRING |
| Date | DATE |
| Datetime | TIMESTAMP |

## 写入方式

库支持三种写入方式，通过环境变量控制：

### 1. INSERT 语句方式（默认）
```bash
# 适合小数据量，直接生成 INSERT 语句
```

### 2. CSV 文件方式
```bash
export USE_CSV_LOAD=true
# 先导出为 CSV 文件，然后使用 LOAD DATA 命令
```

### 3. Parquet 文件方式
```bash
export USE_PARQUET_LOAD=true
# 生成 Parquet 文件，需要手动上传到 HDFS
```

## 环境变量配置

| 环境变量 | 说明 | 默认值 |
|---------|------|--------|
| HIVE_HOST | Hive 服务器地址 | localhost |
| HIVE_PORT | 端口号 | 10000 |
| HIVE_USERNAME | 用户名 | default |
| HIVE_DATABASE | 数据库名 | default |
| HIVE_AUTH | 认证方式 | NONE |
| USE_BEELINE | 使用 beeline 命令 | false |
| USE_CSV_LOAD | 使用 CSV 导入方式 | false |
| USE_PARQUET_LOAD | 使用 Parquet 导入方式 | false |

## 错误处理

```python
from hive_reader_rs import WriteMode

with connect_writer() as writer:
    try:
        writer.write_table(
            df=df,
            table_name="sensitive_table",
            mode=WriteMode.ErrorIfExists
        )
    except RuntimeError as e:
        if "已存在" in str(e):
            print("表已存在，使用追加模式")
            writer.write_table(
                df=df,
                table_name="sensitive_table",
                mode=WriteMode.Append
            )
        else:
            raise
```

## 性能建议

1. **大数据量**: 对于大于 1000 行的数据，建议使用 CSV 或 Parquet 方式
2. **分区表**: 合理设计分区可以显著提高查询性能
3. **批量写入**: 使用批量操作而不是逐行写入
4. **连接复用**: 使用上下文管理器或手动管理连接生命周期

## 示例代码

完整的示例代码请参考 `examples/hive_writer_example.py` 文件。

## 注意事项

1. **权限**: 确保用户有创建表和写入数据的权限
2. **网络**: 确保能够访问 Hive 服务器
3. **依赖**: 使用 beeline 方式时需要安装 Hive 客户端
4. **数据类型**: 不支持的数据类型会导致错误
5. **分区列**: 分区列不能包含 NULL 值

## 故障排除

### 常见问题

1. **连接失败**: 检查主机名、端口号和网络连接
2. **权限错误**: 确认用户权限和认证设置
3. **表已存在**: 使用适当的写入模式
4. **数据类型错误**: 检查不支持的数据类型
5. **文件权限**: 确保临时文件目录有写权限

### 调试模式

```python
import os
import logging

# 启用详细日志
os.environ["RUST_LOG"] = "debug"
logging.basicConfig(level=logging.DEBUG)
```

## 版本兼容性

- Python 3.8+
- Polars 0.39+
- PyO3 0.21+
- Hive 2.0+

---

更多信息请参考项目主 README 和示例代码。 