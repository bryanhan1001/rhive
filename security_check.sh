#!/bin/bash

echo "🔒 安全检查：查找敏感信息"
echo "==============================="

# 检查IP地址
echo "🔍 检查IP地址..."
if grep -r "192\." . --exclude="security_check.sh" --exclude-dir=".git" --exclude-dir="target" 2>/dev/null; then
    echo "❌ 发现IP地址"
else
    echo "✅ 未发现IP地址"
fi

# 检查密码相关
echo "🔍 检查密码相关..."
if grep -ri "password.*=" . --exclude="*.example.py" --exclude="security_check.sh" --exclude-dir=".git" --exclude-dir="target" 2>/dev/null; then
    echo "❌ 发现密码配置"
else
    echo "✅ 未发现硬编码密码"
fi

# 检查access key
echo "🔍 检查access key..."
if grep -ri "access.*key" . --exclude="*.example.py" --exclude="security_check.sh" --exclude-dir=".git" --exclude-dir="target" 2>/dev/null; then
    echo "❌ 发现access key"
else  
    echo "✅ 未发现硬编码access key"
fi

# 检查其他可能的敏感信息
echo "🔍 检查其他敏感信息..."
if grep -ri "secret\|token\|credential" . --exclude="*.example.py" --exclude="security_check.sh" --exclude-dir=".git" --exclude-dir="target" 2>/dev/null; then
    echo "❌ 发现可能的敏感信息"
else
    echo "✅ 未发现其他敏感信息"
fi

echo ""
echo "✅ 安全检查完成"
echo "💡 提示: 确保 .gitignore 包含 config.py 和 .env 等敏感文件"
