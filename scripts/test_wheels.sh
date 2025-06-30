#!/bin/bash

# 本地测试wheels构建脚本

set -e

echo "🧪 本地测试wheel构建..."

# 检查依赖
if ! command -v maturin &> /dev/null; then
    echo "❌ maturin 未安装，请运行: pip install maturin"
    exit 1
fi

if ! command -v cibuildwheel &> /dev/null; then
    echo "⚠️  cibuildwheel 未安装，只进行简单构建"
    echo "安装cibuildwheel: pip install cibuildwheel"
    
    # 简单构建
    echo "🔧 构建当前平台的wheel..."
    maturin build --release
    
    # 测试安装
    echo "📦 测试安装..."
    pip install target/wheels/*.whl --force-reinstall
    
    # 测试导入
    echo "🧪 测试导入..."
    python -c "import hive_reader_rs; print('✅ 导入成功')"
    
else
    # 使用cibuildwheel
    echo "🔧 使用 cibuildwheel 构建..."
    cibuildwheel --platform linux
    
    echo "📦 构建完成，wheel文件:"
    ls -la wheelhouse/
fi

echo "✅ 本地测试完成"
