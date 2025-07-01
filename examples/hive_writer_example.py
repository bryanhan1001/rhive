#!/usr/bin/env python3
"""
Hive Writer 使用示例

演示如何使用 rhive 库将 Polars DataFrame 写入 Hive 表
"""

import polars as pl
import os
from datetime import datetime, date

# 导入 rhive 写入器功能
from hive_reader_rs import (
    connect_writer,
    WriteMode,
    HiveWriter,
    HiveWriteContext,
    HiveConfig,
)


def create_sample_dataframe():
    """创建示例 DataFrame"""
    data = {
        "id": [1, 2, 3, 4, 5],
        "name": ["张三", "李四", "王五", "赵六", "孙七"],
        "age": [25, 30, 35, 28, 32],
        "salary": [5000.0, 8000.0, 12000.0, 6500.0, 9500.0],
        "is_active": [True, True, False, True, True],
        "hire_date": [date(2020, 1, 15), date(2019, 6, 10), date(2018, 3, 20), 
                     date(2021, 9, 5), date(2022, 2, 28)],
        "department": ["IT", "HR", "Finance", "IT", "Marketing"]
    }
    
    df = pl.DataFrame(data)
    print("📊 创建的示例DataFrame:")
    print(df)
    print(f"DataFrame shape: {df.shape}")
    return df


def example_basic_write():
    """基本写入示例"""
    print("\n" + "="*50)
    print("🚀 基本写入示例")
    print("="*50)
    
    # 创建示例数据
    df = create_sample_dataframe()
    
    # 使用上下文管理器写入数据
    with connect_writer(host="localhost", port=10000, database="test") as writer:
        try:
            # 写入到新表（默认模式：如果表存在则报错）
            writer.write_table(
                df=df,
                table_name="employees_basic",
                create_table=True
            )
            print("✅ 基本写入完成")
        except Exception as e:
            print(f"❌ 写入失败: {e}")


def example_write_modes():
    """不同写入模式示例"""
    print("\n" + "="*50)
    print("🔄 写入模式示例")
    print("="*50)
    
    df = create_sample_dataframe()
    
    with connect_writer() as writer:
        try:
            # 1. 覆盖模式
            print("\n📝 使用覆盖模式写入...")
            writer.write_table(
                df=df,
                table_name="employees_modes",
                mode=WriteMode.overwrite,
                create_table=True
            )
            
            # 2. 追加模式
            print("\n📝 使用追加模式写入...")
            new_data = pl.DataFrame({
                "id": [6, 7],
                "name": ["周八", "吴九"],
                "age": [29, 31],
                "salary": [7500.0, 10500.0],
                "is_active": [True, False],
                "hire_date": [date(2023, 1, 10), date(2023, 4, 15)],
                "department": ["Sales", "IT"]
            })
            
            writer.write_table(
                df=new_data,
                table_name="employees_modes",
                mode=WriteMode.append,
                create_table=False
            )
            
            # 3. 忽略模式
            print("\n📝 使用忽略模式写入...")
            writer.write_table(
                df=df,
                table_name="employees_modes",
                mode=WriteMode.ignore,
                create_table=False
            )
            
            print("✅ 所有写入模式测试完成")
            
        except Exception as e:
            print(f"❌ 写入失败: {e}")


def example_partitioned_table():
    """分区表写入示例"""
    print("\n" + "="*50)
    print("📂 分区表写入示例")
    print("="*50)
    
    df = create_sample_dataframe()
    
    with connect_writer() as writer:
        try:
            # 创建按部门分区的表
            writer.write_table(
                df=df,
                table_name="employees_partitioned",
                mode=WriteMode.overwrite,
                partition_cols=["department"],  # 按部门分区
                create_table=True
            )
            print("✅ 分区表写入完成")
        except Exception as e:
            print(f"❌ 分区表写入失败: {e}")


def example_table_management():
    """表管理示例"""
    print("\n" + "="*50)
    print("🗂️  表管理示例")
    print("="*50)
    
    df = create_sample_dataframe()
    
    with connect_writer() as writer:
        try:
            # 1. 仅创建表结构（不插入数据）
            print("\n🏗️  创建表结构...")
            writer.create_table_from_dataframe(
                df=df,
                table_name="employees_schema_only",
                partition_cols=None
            )
            
            # 2. 删除表（如果存在）
            print("\n🗑️  删除表...")
            writer.drop_table(
                table_name="employees_schema_only",
                if_exists=True
            )
            
            print("✅ 表管理操作完成")
        except Exception as e:
            print(f"❌ 表管理操作失败: {e}")


def example_custom_config():
    """自定义配置示例"""
    print("\n" + "="*50)
    print("⚙️  自定义配置示例")
    print("="*50)
    
    # 创建自定义配置
    config = HiveConfig(
        host="my-hive-server",
        port=10000,
        username="admin",
        database="production",
        auth="KERBEROS"
    )
    
    df = create_sample_dataframe()
    
    # 使用直接实例化的方式
    writer = HiveWriter(config)
    try:
        writer.connect()
        print(f"📋 连接配置: {writer.get_config()}")
        print(f"🔗 连接状态: {writer.is_connected()}")
        
        # 这里通常会执行实际的写入操作
        # writer.write_table(df, "employees_custom", ...)
        print("💡 自定义配置设置完成")
        
    except Exception as e:
        print(f"❌ 自定义配置失败: {e}")
    finally:
        writer.disconnect()


def example_environment_based():
    """基于环境变量的配置示例"""
    print("\n" + "="*50)
    print("🌍 环境变量配置示例")
    print("="*50)
    
    # 设置环境变量（实际使用中这些应该在系统环境中设置）
    os.environ["HIVE_HOST"] = "prod-hive.company.com"
    os.environ["HIVE_PORT"] = "10000"
    os.environ["HIVE_DATABASE"] = "analytics"
    os.environ["HIVE_USERNAME"] = "analyst"
    
    # 环境变量控制写入方式
    os.environ["USE_CSV_LOAD"] = "true"  # 使用CSV方式导入
    # os.environ["USE_PARQUET_LOAD"] = "true"  # 使用Parquet方式导入
    # os.environ["USE_BEELINE"] = "true"  # 使用beeline命令行
    
    df = create_sample_dataframe()
    
    # connect_writer 会自动读取环境变量
    with connect_writer() as writer:
        try:
            print(f"📋 环境配置: {writer.get_config()}")
            # 这里会使用环境变量中设置的连接参数
            print("💡 环境变量配置演示完成")
        except Exception as e:
            print(f"❌ 环境变量配置失败: {e}")


def example_error_handling():
    """错误处理示例"""
    print("\n" + "="*50)
    print("⚠️  错误处理示例")
    print("="*50)
    
    df = create_sample_dataframe()
    
    with connect_writer() as writer:
        try:
            # 尝试写入到已存在的表（使用错误模式）
            writer.write_table(
                df=df,
                table_name="existing_table",
                mode=WriteMode.error_if_exists,  # 如果表存在则报错
                create_table=True
            )
        except Exception as e:
            print(f"🔥 预期的错误: {e}")
            print("💡 这是正常的错误处理演示")
            
        try:
            # 再次写入同一个表，但使用忽略模式
            writer.write_table(
                df=df,
                table_name="existing_table",
                mode=WriteMode.ignore,  # 如果表存在则忽略
                create_table=True
            )
            print("✅ 忽略模式处理成功")
        except Exception as e:
            print(f"❌ 意外错误: {e}")


def main():
    """主函数 - 运行所有示例"""
    print("🎯 Hive Writer 功能演示")
    print("本演示展示了 rhive 库的各种写入功能")
    print("注意: 这些示例使用模拟模式，不会连接到真实的Hive服务器")
    
    try:
        # 运行所有示例
        example_basic_write()
        example_write_modes()
        example_partitioned_table()
        example_table_management()
        example_custom_config()
        example_environment_based()
        example_error_handling()
        
        print("\n" + "="*50)
        print("🎉 所有示例运行完成！")
        print("="*50)
        
        print("\n💡 使用提示:")
        print("1. 在生产环境中，请设置正确的Hive连接参数")
        print("2. 对于大数据量，建议使用CSV或Parquet导入方式")
        print("3. 使用分区表可以提高查询性能")
        print("4. 始终使用上下文管理器确保连接正确关闭")
        
    except ImportError as e:
        print(f"❌ 导入错误: {e}")
        print("请确保已正确安装并编译 rhive 库")
    except Exception as e:
        print(f"❌ 运行错误: {e}")


if __name__ == "__main__":
    main() 