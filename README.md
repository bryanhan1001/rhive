# Rust版本Hive数据读取工具 🦀⚡

高性能的Hive数据读取库，使用Rust实现核心逻辑，通过PyO3提供Python接口，支持Polars DataFrame。

## 特性

- 🚀 **高性能**: 使用Rust实现，性能比Python版本更优
- 🐍 **Python兼容**: 提供完整的Python接口，可直接替换Python版本
- 📊 **Polars集成**: 原生支持Polars DataFrame，零拷贝数据传输
- 🔧 **多种连接方式**: 支持beeline命令行和模拟数据模式
- 🎯 **类型安全**: Rust的类型系统确保运行时安全
- 📝 **上下文管理**: 支持Python的with语句，自动管理资源

## 系统要求

- Python 3.8+
- Rust 1.70+ (用于编译)
- Polars 0.19+

## 配置管理

本项目使用配置管理器，支持多种配置方式，优先级如下：
1. **函数参数** (最高优先级)
2. **环境变量** (HIVE_HOST, HIVE_PORT, HIVE_AUTH 等)
3. **config.py 文件** 
4. **默认值** (localhost:10000, auth=NONE)

### 配置方式

#### 方式1: 环境变量
```bash
export HIVE_HOST=your_hive_host
export HIVE_PORT=10000
export HIVE_AUTH=NONE
```

#### 方式2: 配置文件
```bash
# 复制配置示例
cp config.example.py config.py
# 编辑配置文件
vim config.py
```

#### 方式3: 运行时参数
```python
with hive_reader_rs.connect_hive(host="your_host", port=10000) as hive:
    # ...
```

## 安装步骤

### 1. 安装Rust (如果还没有)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. 安装maturin

```bash
pip install maturin
```

### 3. 克隆或进入项目目录

```bash
cd test/hive/rust_hive
```

### 4. 开发模式编译 (推荐)

```bash
maturin develop --release
```

### 5. 或者从GitHub Releases安装

```bash
# 安装预编译的wheel包
pip install hive-reader-rs

# 或者从GitHub Releases下载特定版本
# https://github.com/YOUR_USERNAME/hive-reader-rs/releases
```

### 6. 本地构建wheel包

```bash
maturin build --release
pip install target/wheels/hive_reader_rs-*.whl
```

## 快速开始

### 基础使用

```python
import hive_reader_rs

# 创建配置 (使用配置管理器)
config = hive_reader_rs.create_default_config()

# 基础使用
reader = hive_reader_rs.RustHiveReader(config)
reader.connect()

# 执行查询
df = reader.query_to_polars("SELECT * FROM your_table LIMIT 10")
print(df)

reader.disconnect()
```

### 使用上下文管理器 (推荐)

```python
import hive_reader_rs

# 方式1: 使用配置管理器
config = hive_reader_rs.create_default_config()

with hive_reader_rs.RustHiveContext(config) as hive:
    # 显示表
    tables = hive.show_tables()
    print(tables)
    
    # 查询数据
    df = hive.query_to_polars("SELECT COUNT(*) FROM your_table")
    print(df)

# 方式2: 使用便捷函数 (自动从配置加载)
with hive_reader_rs.connect_hive() as hive:
    df = hive.query_to_polars("SELECT 'Hello Rust!' as message")
    print(df)
```

## 连接模式

### 1. 模拟模式 (默认)

用于开发和测试，返回模拟数据：

```python
# 默认使用模拟数据
with hive_reader_rs.connect_hive() as hive:
    df = hive.query_to_polars("SELECT * FROM users")  # 返回模拟用户数据
```

### 2. Beeline模式

连接真实的Hive服务：

```bash
# 设置环境变量启用beeline模式
export USE_BEELINE=true
```

```python
# 现在会使用beeline连接真实Hive
with hive_reader_rs.connect_hive(host="your_hive_host", port=10000) as hive:
    df = hive.query_to_polars("SELECT * FROM real_table")
```

## API参考

### HiveConfig

配置Hive连接参数：

```python
config = hive_reader_rs.HiveConfig(
    host="localhost",      # Hive服务器地址
    port=10000,           # 端口号
    username="default",   # 用户名
    database="default",   # 数据库名
    auth="NONE"          # 认证方式
)
```

### RustHiveReader

主要的Hive读取器类：

```python
reader = hive_reader_rs.RustHiveReader(config)

# 连接管理
reader.connect()                    # 连接到Hive
reader.disconnect()                 # 断开连接
reader.is_connected()               # 检查连接状态

# 数据查询
df = reader.query_to_polars(sql)                      # 执行SQL查询
tables = reader.show_tables()                         # 显示所有表
schema = reader.describe_table("table_name")          # 查看表结构
sample = reader.get_table_sample("table_name", 10)    # 获取样本数据
```

### RustHiveContext

上下文管理器，支持with语句：

```python
with hive_reader_rs.RustHiveContext(config) as hive:
    # 自动管理连接
    df = hive.query_to_polars("SELECT * FROM table")
```

## 性能基准测试

```python
import hive_reader_rs

# 使用配置管理器
config = hive_reader_rs.create_default_config()

# 基准测试
results = hive_reader_rs.benchmark_query(
    config=config,
    sql="SELECT COUNT(*) FROM large_table",
    iterations=10
)

print(f"平均查询时间: {results['average_time']:.4f}秒")
print(f"每秒查询数: {results['queries_per_second']:.2f}")
```

## 错误处理

```python
try:
    with hive_reader_rs.connect_hive(host="invalid_host") as hive:
        df = hive.query_to_polars("SELECT 1")
except Exception as e:
    print(f"连接失败: {e}")
```

## 开发和调试

### 编译选项

```bash
# 开发模式 (包含调试信息)
maturin develop

# 发布模式 (优化性能)
maturin develop --release

# 清理构建缓存
cargo clean
```

### 运行测试

```bash
# 运行Rust测试
cargo test

# 运行Python示例
python examples/basic_usage.py
```

### 启用日志

```bash
# 设置日志级别
export RUST_LOG=debug
python examples/basic_usage.py
```

## 🚀 自动发布和安装

### GitHub Actions自动构建

本项目配置了完整的CI/CD流程，支持自动构建跨平台安装包：

#### 支持的平台
- ✅ **Linux** (x86_64, aarch64)
- ✅ **Windows** (x86_64) 
- ✅ **macOS** (Intel, Apple Silicon)
- ✅ **Python 3.8-3.12**

#### 发布流程
```bash
# 1. 发布新版本
./scripts/release.sh patch    # 0.1.0 → 0.1.1
./scripts/release.sh minor    # 0.1.0 → 0.2.0
./scripts/release.sh major    # 0.1.0 → 1.0.0

# 2. GitHub Actions自动构建所有平台的wheel包
# 3. 自动发布到GitHub Releases
# 4. 可选发布到PyPI
```

#### 用户安装
```bash
# 直接从PyPI安装 (推荐)
pip install hive-reader-rs

# 或从GitHub Releases下载
# https://github.com/YOUR_USERNAME/hive-reader-rs/releases
```

详细发布指南请查看 [RELEASING.md](RELEASING.md)

## 与Python版本的比较

| 特性 | Python版本 | Rust版本 |
|------|------------|----------|
| 查询性能 | 标准 | 🚀 更快 |
| 内存使用 | 标准 | 🔥 更少 |
| 类型安全 | 运行时检查 | ✅ 编译时保证 |
| 安装复杂度 | 简单 | 📦 预编译包 |
| 功能完整性 | 完整 | 核心功能 |
| 错误处理 | Python异常 | Rust Result + Python异常 |
| 跨平台支持 | 手动 | 🤖 自动化 |

## 故障排除

### 编译错误

```bash
# 更新Rust工具链
rustup update

# 检查Python版本
python --version

# 重新安装maturin
pip install --upgrade maturin
```

### 运行时错误

```bash
# 检查扩展模块是否正确安装
python -c "import hive_reader_rs; print('OK')"

# 检查依赖
pip list | grep polars
```

### 连接问题

1. **模拟模式**: 无需实际Hive服务，用于开发测试
2. **Beeline模式**: 需要安装并配置beeline命令行工具
3. **直接连接**: 未来版本可能支持直接Thrift连接

## 贡献

欢迎贡献代码！请确保：

1. 代码通过 `cargo test`
2. 代码通过 `cargo clippy`
3. 使用 `cargo fmt` 格式化代码

## 许可证

MIT License

## 更新日志

### v0.1.0
- 初始版本
- 基础Hive连接功能
- Polars DataFrame支持
- 上下文管理器支持
- 性能基准测试 