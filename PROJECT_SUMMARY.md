# Rust版本Hive读取器项目总结

## 项目概述

这是一个高性能的Hive数据读取工具，使用Rust实现核心逻辑，通过PyO3提供Python接口。相比纯Python版本，该工具具有更好的性能和内存效率。

## 项目结构

```
rust_hive/
├── src/lib.rs                    # 核心Rust代码
├── python/hive_reader_rs/        # Python接口
├── examples/basic_usage.py       # 使用示例
├── Cargo.toml                    # Rust项目配置
├── pyproject.toml                # Python打包配置
├── Makefile                      # 构建工具
├── install.sh                    # 一键安装脚本
├── test_rust_extension.py        # 测试文件
└── README.md                     # 详细文档
```

## 主要特性

- 🚀 **高性能**: Rust实现，比Python版本更快
- 🐍 **Python兼容**: 提供完整的Python接口
- 📊 **Polars集成**: 原生支持Polars DataFrame
- 🔧 **多种连接方式**: 支持beeline和模拟数据模式
- 📝 **上下文管理**: 支持Python with语句
- ⚡ **基准测试**: 内置性能测试功能

## 快速开始

### 1. 一键安装
```bash
./install.sh
```

### 2. 手动安装
```bash
make install
```

### 3. 验证安装
```bash
python3 test_rust_extension.py
```

## 基本使用

```python
import hive_reader_rs

# 自动从配置管理器加载配置
with hive_reader_rs.connect_hive() as hive:
    df = hive.query_to_polars("SELECT * FROM table LIMIT 10")
    print(df)
```

## 许可证

MIT License
