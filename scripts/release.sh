#!/bin/bash

# 发布脚本
# 使用方法: ./scripts/release.sh [major|minor|patch]

set -e

VERSION_TYPE=${1:-patch}

echo "🚀 开始发布流程..."
echo "版本类型: $VERSION_TYPE"

# 检查工作目录是否干净
if [[ -n $(git status --porcelain) ]]; then
    echo "❌ 工作目录不干净，请先提交或储藏更改"
    exit 1
fi

# 确保在主分支
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    echo "❌ 请切换到main或master分支"
    exit 1
fi

# 获取当前版本
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "当前版本: $CURRENT_VERSION"

# 计算新版本（简单实现）
IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

case $VERSION_TYPE in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
    *)
        echo "❌ 无效的版本类型: $VERSION_TYPE"
        echo "使用 major, minor, 或 patch"
        exit 1
        ;;
esac

NEW_VERSION="$MAJOR.$MINOR.$PATCH"
echo "新版本: $NEW_VERSION"

# 确认发布
read -p "确认发布版本 $NEW_VERSION? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 发布已取消"
    exit 1
fi

# 更新版本号
echo "📝 更新版本号..."
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" pyproject.toml
rm -f Cargo.toml.bak pyproject.toml.bak

# 运行测试
echo "🧪 运行测试..."
if command -v cargo &> /dev/null; then
    cargo test
fi

# 构建检查
echo "🔧 构建检查..."
if command -v maturin &> /dev/null; then
    maturin build --release
fi

# 提交更改
echo "📦 提交版本更新..."
git add Cargo.toml pyproject.toml
git commit -m "🔖 Release v$NEW_VERSION"

# 创建标签
echo "🏷️  创建标签..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

# 推送
echo "🚀 推送到远程..."
git push origin "$CURRENT_BRANCH"
git push origin "v$NEW_VERSION"

echo ""
echo "🎉 发布完成！"
echo "标签 v$NEW_VERSION 已推送，GitHub Actions 将自动构建和发布包"
echo ""
echo "🔗 查看发布状态:"
echo "   GitHub Actions: https://github.com/YOUR_USERNAME/hive-reader-rs/actions"
echo "   GitHub Releases: https://github.com/YOUR_USERNAME/hive-reader-rs/releases"
echo "   PyPI: https://pypi.org/project/hive-reader-rs/"
