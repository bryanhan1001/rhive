"""
Rust版本的Hive数据读取和写入工具

高性能的Hive数据读取和写入库，使用Rust实现核心逻辑，
通过PyO3提供Python接口，支持Polars DataFrame。

配置优先级：
1. 函数参数 (最高优先级)
2. 环境变量
3. config.py 文件
4. 默认值 (最低优先级)
"""

from typing import Optional
import polars as pl

try:
    from .hive_reader_rs import (
        # 配置相关
        HiveConfig,
        create_hive_config,
        config_from_dict,
        
        # 读取器相关
        RustHiveReader,
        RustHiveContext,
        
        # 写入器相关
        WriteMode,
        RustHiveWriter,
        RustHiveWriteContext,
        connect_hive_writer,
        
        # 工具函数
        benchmark_query,
        __version__,
        __author__,
    )
except ImportError as e:
    raise ImportError(f"无法导入Rust扩展: {e}") from e

from .config_manager import (
    config_manager,
    get_default_hive_config,
    create_config_from_env,
)

__all__ = [
    # 配置相关
    "HiveConfig",
    "create_hive_config",
    "config_from_dict",
    
    # 读取器相关
    "RustHiveReader", 
    "RustHiveContext",
    "connect_hive",
    
    # 写入器相关
    "WriteMode",
    "RustHiveWriter",
    "RustHiveWriteContext", 
    "connect_hive_writer",
    
    # 工具函数
    "benchmark_query",
    "create_default_config",
    "get_config_manager",
    
    # 版本信息
    "__version__",
    "__author__",
]


def connect_hive(host=None, port=None, username=None, database=None, auth=None):
    """
    便捷连接函数，支持配置优先级
    
    Args:
        host: Hive服务器地址 (可选，默认从配置获取)
        port: 端口号 (可选，默认从配置获取)
        username: 用户名 (可选，默认从配置获取)
        database: 数据库名 (可选，默认从配置获取)
        auth: 认证方式 (可选，默认从配置获取)
        
    Returns:
        RustHiveContext: 上下文管理器
        
    配置优先级：参数 > 环境变量 > config.py > 默认值
    """
    # 获取默认配置
    default_config = get_default_hive_config()
    
    # 使用参数覆盖默认配置
    config = create_hive_config(
        host=host or default_config.get("host"),
        port=port or default_config.get("port"),
        username=username or default_config.get("username"),
        database=database or default_config.get("database"),
        auth=auth or default_config.get("auth"),
    )
    
    return RustHiveContext(config)


def connect_writer(host=None, port=None, username=None, database=None, auth=None):
    """
    便捷写入器连接函数，支持配置优先级
    
    Args:
        host: Hive服务器地址 (可选，默认从配置获取)
        port: 端口号 (可选，默认从配置获取)
        username: 用户名 (可选，默认从配置获取)
        database: 数据库名 (可选，默认从配置获取)
        auth: 认证方式 (可选，默认从配置获取)
        
    Returns:
        RustHiveWriteContext: 写入器上下文管理器
        
    配置优先级：参数 > 环境变量 > config.py > 默认值
    """
    # 获取默认配置
    default_config = get_default_hive_config()
    
    # 使用参数覆盖默认配置
    config = create_hive_config(
        host=host or default_config.get("host"),
        port=port or default_config.get("port"),
        username=username or default_config.get("username"),
        database=database or default_config.get("database"),
        auth=auth or default_config.get("auth"),
    )
    
    return RustHiveWriteContext(config)


def create_default_config():
    """创建基于环境的默认配置"""
    return create_config_from_env()


def get_config_manager():
    """获取配置管理器实例"""
    return config_manager

# 添加便捷的写入器别名
HiveWriter = RustHiveWriter
HiveWriteContext = RustHiveWriteContext

# 扩展__all__列表
__all__.extend([
    "connect_writer",
    "HiveWriter", 
    "HiveWriteContext",
]) 