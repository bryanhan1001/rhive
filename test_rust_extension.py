#!/usr/bin/env python3
"""
测试Rust版本Hive读取器
使用配置管理器，无硬编码敏感信息
"""

import sys
import os
import traceback

# 添加路径
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'python'))

def test_import():
    """测试导入"""
    try:
        import hive_reader_rs
        print("✅ 成功导入 hive_reader_rs")
        
        # 测试版本信息
        try:
            print(f"   版本: {hive_reader_rs.__version__}")
            print(f"   作者: {hive_reader_rs.__author__}")
        except AttributeError:
            print("   版本信息不可用 (这是正常的)")
        
        return True
    except ImportError as e:
        print(f"❌ 导入失败: {e}")
        print("请先运行: make install 或 maturin develop")
        return False

def test_config_manager():
    """测试配置管理器"""
    try:
        import hive_reader_rs
        
        # 测试配置管理器
        config_mgr = hive_reader_rs.get_config_manager()
        print(f"✅ 配置管理器创建成功: {config_mgr}")
        
        # 测试默认配置获取
        default_config = hive_reader_rs.get_default_hive_config()
        print(f"✅ 默认配置获取成功: {default_config}")
        
        return True
        
    except Exception as e:
        print(f"❌ 配置管理器测试失败: {e}")
        traceback.print_exc()
        return False

def test_config_creation():
    """测试配置创建"""
    try:
        import hive_reader_rs
        
        # 测试手动配置创建
        config = hive_reader_rs.create_hive_config(
            host="test_host",
            port=9999,
            username="test_user",
            database="test_db",
            auth="TEST_AUTH"
        )
        
        assert config.host == "test_host"
        assert config.port == 9999
        assert config.username == "test_user"
        assert config.database == "test_db"
        assert config.auth == "TEST_AUTH"
        
        print("✅ 手动配置创建测试通过")
        
        # 测试默认配置创建
        default_config = hive_reader_rs.create_default_config()
        print(f"✅ 默认配置创建测试通过")
        
        return True
        
    except Exception as e:
        print(f"❌ 配置创建测试失败: {e}")
        traceback.print_exc()
        return False

def test_reader():
    """测试读取器"""
    try:
        import hive_reader_rs
        
        # 使用默认配置进行测试
        config = hive_reader_rs.create_default_config()
        reader = hive_reader_rs.RustHiveReader(config)
        
        # 测试连接
        reader.connect()
        assert reader.is_connected() == True
        print("✅ 连接测试通过")
        
        # 测试查询 (模拟模式)
        df = reader.query_to_polars("SELECT 'test' as message")
        print(f"✅ 查询测试通过，结果行数: {df.height}")
        
        # 测试其他方法
        tables = reader.show_tables()
        print(f"✅ 显示表测试通过，表数量: {tables.height}")
        
        # 断开连接
        reader.disconnect()
        assert reader.is_connected() == False
        print("✅ 断开连接测试通过")
        
        return True
        
    except Exception as e:
        print(f"❌ 读取器测试失败: {e}")
        traceback.print_exc()
        return False

def test_context_manager():
    """测试上下文管理器"""
    try:
        import hive_reader_rs
        
        # 使用便捷函数
        with hive_reader_rs.connect_hive() as hive:
            df = hive.query_to_polars("SELECT 'context_test' as test")
            print(f"✅ 上下文管理器测试通过")
        
        return True
        
    except Exception as e:
        print(f"❌ 上下文管理器测试失败: {e}")
        traceback.print_exc()
        return False

def test_convenience_function():
    """测试便捷函数"""
    try:
        import hive_reader_rs
        
        # 使用便捷函数，自动从配置管理器获取配置
        with hive_reader_rs.connect_hive() as hive:
            df = hive.query_to_polars("SELECT 'convenience' as test")
            print(f"✅ 便捷函数测试通过")
        
        return True
        
    except Exception as e:
        print(f"❌ 便捷函数测试失败: {e}")
        traceback.print_exc()
        return False

def test_benchmark():
    """测试基准测试功能"""
    try:
        import hive_reader_rs
        
        # 使用默认配置
        config = hive_reader_rs.create_default_config()
        
        results = hive_reader_rs.benchmark_query(
            config=config,
            sql="SELECT 1 as test",
            iterations=3
        )
        
        required_keys = ["total_time", "average_time", "iterations", "queries_per_second"]
        for key in required_keys:
            assert key in results, f"缺少结果键: {key}"
        
        print(f"✅ 基准测试功能正常")
        print(f"   平均时间: {results['average_time']:.4f}秒")
        print(f"   每秒查询: {results['queries_per_second']:.2f}")
        
        return True
        
    except Exception as e:
        print(f"❌ 基准测试失败: {e}")
        traceback.print_exc()
        return False

def main():
    """运行所有测试"""
    print("🧪 Rust版本Hive读取器测试 (无敏感信息)")
    print("=" * 60)
    
    tests = [
        ("导入测试", test_import),
        ("配置管理器测试", test_config_manager),
        ("配置创建测试", test_config_creation),
        ("读取器测试", test_reader),
        ("上下文管理器测试", test_context_manager),
        ("便捷函数测试", test_convenience_function),
        ("基准测试", test_benchmark),
    ]
    
    passed = 0
    total = len(tests)
    
    for name, test_func in tests:
        print(f"\n🔍 运行 {name}...")
        try:
            if test_func():
                passed += 1
            else:
                print(f"❌ {name} 失败")
        except Exception as e:
            print(f"❌ {name} 异常: {e}")
    
    print("\n" + "=" * 60)
    print(f"📊 测试结果: {passed}/{total} 通过")
    
    if passed == total:
        print("🎉 所有测试通过！")
        print("\n💡 提示:")
        print("   - 所有测试使用配置管理器，无硬编码敏感信息")
        print("   - 可通过环境变量或config.py自定义配置")
        print("   - 默认使用模拟数据模式")
        return 0
    else:
        print("❌ 部分测试失败")
        return 1

if __name__ == "__main__":
    print("🔧 配置说明:")
    print("   环境变量: HIVE_HOST, HIVE_PORT, HIVE_AUTH 等")
    print("   配置文件: 复制 config.example.py 为 config.py")
    print("   默认配置: localhost:10000, auth=NONE\n")
    
    sys.exit(main())
