"""
Hive连接配置示例文件

使用方法:
1. 复制此文件为 config.py
2. 修改配置为您的实际环境
3. config.py 已在 .gitignore 中，不会被提交到版本控制
"""

# Hive连接配置
HIVE_CONNECT_INFO = {
    "uri": "thrift://YOUR_HIVE_HOST:10000",  # 替换为您的Hive服务器地址
    "auth": "NONE",  # 认证方式: NONE, PLAIN, KERBEROS
    "username": "default",  # 用户名
    "password": "",  # 密码（如需要）
    "database": "default",  # 默认数据库
}

# S3配置（如果使用）
S3_CONFIG = {
    "s3.endpoint": "http://YOUR_S3_ENDPOINT:9000",
    "s3.access-key-id": "YOUR_ACCESS_KEY",
    "s3.secret-access-key": "YOUR_SECRET_KEY",
}

# 其他配置
SETTINGS = {
    "connection_timeout": 30,  # 连接超时（秒）
    "query_timeout": 300,     # 查询超时（秒）
    "retry_attempts": 3,      # 重试次数
    "use_beeline": False,     # 是否使用beeline模式
}
