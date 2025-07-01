"""
配置管理模块

支持从多种来源加载配置：
1. 环境变量 (最高优先级)
2. config.py 文件
3. 默认值 (最低优先级)
"""

import os
from typing import Dict, Any, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from .hive_reader_rs import HiveConfig


class ConfigManager:
    """配置管理器"""
    
    def __init__(self):
        self._config = self._load_config()
    
    def _load_config(self) -> Dict[str, Any]:
        """加载配置，按优先级合并"""
        config = self._get_default_config()
        
        # 尝试从config.py加载
        try:
            file_config = self._load_from_file()
            config.update(file_config)
        except ImportError:
            pass  # config.py不存在，使用默认配置
        
        # 从环境变量加载（最高优先级）
        env_config = self._load_from_env()
        config.update(env_config)
        
        return config
    
    def _get_default_config(self) -> Dict[str, Any]:
        """默认配置"""
        return {
            "host": "localhost",
            "port": 10000,
            "username": "default",
            "database": "default", 
            "auth": "NONE",
            "connection_timeout": 30,
            "query_timeout": 300,
            "retry_attempts": 3,
            "use_beeline": False,
        }
    
    def _load_from_file(self) -> Dict[str, Any]:
        """从config.py文件加载配置"""
        try:
            from config import HIVE_CONNECT_INFO, SETTINGS
            
            # 解析URI
            uri = HIVE_CONNECT_INFO.get('uri', 'thrift://localhost:10000')
            if uri.startswith('thrift://'):
                uri_parts = uri.replace('thrift://', '').split(':')
                host = uri_parts[0]
                port = int(uri_parts[1]) if len(uri_parts) > 1 else 10000
            else:
                host = "localhost"
                port = 10000
            
            config = {
                "host": host,
                "port": port,
                "username": HIVE_CONNECT_INFO.get('username', 'default'),
                "database": HIVE_CONNECT_INFO.get('database', 'default'),
                "auth": HIVE_CONNECT_INFO.get('auth', 'NONE'),
            }
            
            # 合并其他设置
            config.update(SETTINGS)
            
            return config
            
        except ImportError:
            return {}
    
    def _load_from_env(self) -> Dict[str, Any]:
        """从环境变量加载配置"""
        config = {}
        
        # 基础连接配置
        if os.getenv('HIVE_HOST'):
            config['host'] = os.getenv('HIVE_HOST')
        if os.getenv('HIVE_PORT'):
            config['port'] = int(os.getenv('HIVE_PORT'))
        if os.getenv('HIVE_USERNAME'):
            config['username'] = os.getenv('HIVE_USERNAME')
        if os.getenv('HIVE_DATABASE'):
            config['database'] = os.getenv('HIVE_DATABASE')
        if os.getenv('HIVE_AUTH'):
            config['auth'] = os.getenv('HIVE_AUTH')
        
        # 其他配置
        if os.getenv('USE_BEELINE'):
            config['use_beeline'] = os.getenv('USE_BEELINE').lower() == 'true'
        if os.getenv('CONNECTION_TIMEOUT'):
            config['connection_timeout'] = int(os.getenv('CONNECTION_TIMEOUT'))
        if os.getenv('QUERY_TIMEOUT'):
            config['query_timeout'] = int(os.getenv('QUERY_TIMEOUT'))
        if os.getenv('RETRY_ATTEMPTS'):
            config['retry_attempts'] = int(os.getenv('RETRY_ATTEMPTS'))
        
        return config
    
    def get(self, key: str, default: Any = None) -> Any:
        """获取配置值"""
        return self._config.get(key, default)
    
    def get_hive_config(self) -> Dict[str, Any]:
        """获取Hive连接配置"""
        return {
            "host": self.get("host"),
            "port": self.get("port"),
            "username": self.get("username"),
            "database": self.get("database"),
            "auth": self.get("auth"),
        }
    
    def get_all_config(self) -> Dict[str, Any]:
        """获取所有配置"""
        return self._config.copy()
    
    def __repr__(self) -> str:
        # 不显示敏感信息
        safe_config = self._config.copy()
        if 'password' in safe_config:
            safe_config['password'] = '***'
        return f"ConfigManager({safe_config})"


# 全局配置管理器实例
config_manager = ConfigManager()


def get_default_hive_config():
    """获取默认的Hive配置"""
    return config_manager.get_hive_config()


def load_config_from_env():
    """从环境变量加载配置（兼容旧接口）"""
    return config_manager.get_hive_config()


def create_config_from_env() -> 'HiveConfig':
    """从环境创建HiveConfig对象"""
    from . import create_hive_config
    
    config = config_manager.get_hive_config()
    return create_hive_config(
        host=config.get("host"),
        port=config.get("port"),
        username=config.get("username"),
        database=config.get("database"),
        auth=config.get("auth"),
    ) 