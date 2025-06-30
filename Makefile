# Rust版本Hive读取器 Makefile

.PHONY: help install dev build test clean release test-wheels check-release

help:
	@echo "🦀 Rust版本Hive读取器构建工具"
	@echo "可用命令:"
	@echo "  install       - 安装并编译扩展"
	@echo "  dev           - 开发模式编译"
	@echo "  build         - 构建wheel包"
	@echo "  test          - 运行测试"
	@echo "  test-wheels   - 本地测试wheel构建"
	@echo "  clean         - 清理构建缓存"
	@echo "  release       - 发布新版本 (patch)"
	@echo "  check-release - 检查发布准备状态"

install:
	@echo "📦 安装依赖并编译..."
	@pip install maturin polars
	@maturin develop --release

dev:
	@echo "🔧 开发模式编译..."
	@maturin develop

build:
	@echo "📦 构建wheel包..."
	@maturin build --release

test:
	@echo "🧪 运行测试..."
	@cargo test
	@python test_rust_extension.py

test-wheels:
	@echo "🧪 本地测试wheel构建..."
	@./scripts/test_wheels.sh

clean:
	@echo "🧹 清理..."
	@cargo clean
	@rm -rf target/ wheelhouse/ dist/

release:
	@echo "🚀 发布patch版本..."
	@./scripts/release.sh patch

check-release:
	@echo "🔍 检查发布准备状态..."
	@echo "Git状态:"
	@git status --porcelain || echo "❌ 工作目录不干净"
	@echo "当前分支:"
	@git branch --show-current
	@echo "当前版本:"
	@grep '^version = ' Cargo.toml
	@echo "测试状态:"
	@cargo test >/dev/null 2>&1 && echo "✅ Rust测试通过" || echo "❌ Rust测试失败"
	@python test_rust_extension.py >/dev/null 2>&1 && echo "✅ Python测试通过" || echo "❌ Python测试失败" 