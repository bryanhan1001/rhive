#!/bin/bash

echo "🦀 Rust版本Hive读取器安装脚本"
echo "=================================="

# 检查Python
echo "�� 检查Python环境..."
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3未安装"
    exit 1
fi
echo "✅ Python OK"

# 检查Rust
echo "🔍 检查Rust环境..."
if ! command -v rustc &> /dev/null; then
    echo "📦 安装Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi
echo "✅ Rust OK"

# 安装依赖
echo "📦 安装依赖..."
pip3 install maturin polars

# 编译
echo "🔧 编译..."
maturin develop --release

# 测试
echo "🧪 测试..."
python3 test_rust_extension.py

echo "🎉 安装完成！"
