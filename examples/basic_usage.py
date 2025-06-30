#!/usr/bin/env python3
"""
Rust版本Hive读取器使用示例
"""

import sys
import os

# 添加路径以便导入
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

try:
    import hive_reader_rs
    print("✅ 成功导入Rust扩展")
except ImportError as e:
    print(f"❌ 导入Rust扩展失败: {e}")
    print("请先编译Rust扩展: maturin develop")
    sys.exit(1)


def basic_example():
    """基础使用示例"""
    print("\n=== 基础使用示例 ===")
    
    # 创建配置 - 使用环境变量或默认值
    config = hive_reader_rs.create_default_config()
    print(f"配置: {config}")
    
    # 创建读取器
    reader = hive_reader_rs.RustHiveReader(config)
    
    try:
        # 连接
        reader.connect()
        print("✅ 连接成功")
        
        # 查询
        df = reader.query_to_polars("SELECT 'Hello from Rust!' as message, 42 as number")
        print("查询结果:")
        print(df)
        
        # 显示表
        tables = reader.show_tables()
        print(f"\n表列表:")
        print(tables)
        
    except Exception as e:
        print(f"❌ 执行失败: {e}")
    finally:
        reader.disconnect()


def context_manager_example():
    """上下文管理器示例"""
    print("\n=== 上下文管理器示例 ===")
    
    # 使用默认配置
    config = hive_reader_rs.create_default_config()
    
    try:
        with hive_reader_rs.RustHiveContext(config) as hive:
            print("✅ 使用with语句连接成功")
            
            # 多个查询
            queries = [
                "SELECT 'query1' as name, 100 as value",
                "SELECT 'query2' as name, 200 as value",
                "SELECT 'query3' as name, 300 as value"
            ]
            
            for i, sql in enumerate(queries, 1):
                print(f"\n执行查询 {i}:")
                df = hive.query_to_polars(sql)
                print(df)
                
    except Exception as e:
        print(f"❌ 上下文管理器执行失败: {e}")


def benchmark_example():
    """性能基准测试示例"""
    print("\n=== 性能基准测试示例 ===")
    
    # 使用默认配置
    config = hive_reader_rs.create_default_config()
    
    try:
        sql = "SELECT 'benchmark' as test, 1 as num"
        results = hive_reader_rs.benchmark_query(config, sql, iterations=5)
        
        print("基准测试结果:")
        for key, value in results.items():
            print(f"  {key}: {value:.4f}")
            
    except Exception as e:
        print(f"❌ 基准测试失败: {e}")


def convenience_function_example():
    """便捷函数示例"""
    print("\n=== 便捷函数示例 ===")
    
    try:
        # 使用便捷函数，会自动从配置加载默认值
        with hive_reader_rs.connect_hive() as hive:
            print("✅ 使用便捷函数连接成功")
            
            df = hive.query_to_polars("SELECT 'convenience' as method")
            print("结果:")
            print(df)
            
    except Exception as e:
        print(f"❌ 便捷函数示例失败: {e}")


def config_from_env_example():
    """从环境变量和配置文件示例"""
    print("\n=== 配置管理示例 ===")
    
    try:
        # 显示当前配置管理器状态
        config_mgr = hive_reader_rs.get_config_manager()
        print(f"配置管理器: {config_mgr}")
        
        # 获取默认配置
        default_config = hive_reader_rs.get_default_hive_config()
        print(f"默认配置: {default_config}")
        
        # 创建自定义配置（如果需要覆盖某些值）
        custom_config = hive_reader_rs.create_hive_config(
            host=default_config.get("host"),
            port=default_config.get("port"),
            username=default_config.get("username"),
            database=default_config.get("database"),
            auth=default_config.get("auth"),
        )
        
        print(f"创建的配置: {custom_config}")
        
        print("\n💡 配置优先级:")
        print("   1. 函数参数 (最高)")
        print("   2. 环境变量 (HIVE_HOST, HIVE_PORT 等)")
        print("   3. config.py 文件")
        print("   4. 默认值 (最低)")
        
    except Exception as e:
        print(f"❌ 配置管理失败: {e}")


if __name__ == "__main__":
    print("🚀 Rust版本Hive读取器示例")
    print("=" * 50)
    
    # 显示版本信息
    try:
        print(f"版本: {hive_reader_rs.__version__}")
        print(f"作者: {hive_reader_rs.__author__}")
    except AttributeError:
        print("版本信息不可用")
    
    # 运行示例
    try:
        basic_example()
        context_manager_example()
        benchmark_example()
        convenience_function_example()
        config_from_env_example()
        
        print("\n" + "=" * 50)
        print("🎉 所有示例运行完成！")
        
    except KeyboardInterrupt:
        print("\n\n⚠️  用户中断执行")
    except Exception as e:
        print(f"\n\n❌ 运行示例时出错: {e}")
        import traceback
        traceback.print_exc()
        
    print("\n💡 提示:")
    print("   1. 这些示例使用模拟数据")
    print("   2. 要连接真实Hive，请设置环境变量 USE_BEELINE=true")
    print("   3. 确保beeline命令可用且Hive服务正在运行") 