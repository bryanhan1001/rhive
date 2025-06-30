# 发布指南 📦

本文档说明如何使用GitHub Actions自动生成跨平台安装包。

## 🚀 自动发布流程

### 1. 准备工作

确保你的GitHub仓库设置正确：

```bash
# 1. 克隆或fork项目
git clone https://github.com/YOUR_USERNAME/hive-reader-rs.git
cd hive-reader-rs

# 2. 更新项目URL
# 编辑 pyproject.toml，将 YOUR_USERNAME 替换为你的GitHub用户名
```

### 2. 配置PyPI (可选)

如果要发布到PyPI，需要配置API Token：

1. 在PyPI创建账户：https://pypi.org/account/register/
2. 生成API Token：https://pypi.org/manage/account/token/
3. 在GitHub仓库设置Secrets：
   - 进入仓库 → Settings → Secrets and variables → Actions
   - 添加 `PYPI_API_TOKEN`，值为PyPI的API Token

### 3. 发布新版本

有两种方式：

#### 方式1: 使用脚本 (推荐)

```bash
# 发布patch版本 (0.1.0 → 0.1.1)
./scripts/release.sh patch

# 发布minor版本 (0.1.0 → 0.2.0)
./scripts/release.sh minor

# 发布major版本 (0.1.0 → 1.0.0)
./scripts/release.sh major
```

#### 方式2: 手动创建标签

```bash
# 1. 更新版本号
vim Cargo.toml      # 更新 version = "0.1.1"
vim pyproject.toml  # 更新 version = "0.1.1"

# 2. 提交更改
git add .
git commit -m "🔖 Release v0.1.1"

# 3. 创建标签
git tag -a v0.1.1 -m "Release v0.1.1"

# 4. 推送
git push origin main
git push origin v0.1.1
```

### 4. 监控构建过程

1. **GitHub Actions**: https://github.com/YOUR_USERNAME/hive-reader-rs/actions
2. **GitHub Releases**: https://github.com/YOUR_USERNAME/hive-reader-rs/releases
3. **PyPI页面**: https://pypi.org/project/hive-reader-rs/

## 📋 支持的平台

自动构建将为以下平台生成wheel包：

### 操作系统
- ✅ **Linux** (x86_64, aarch64)
- ✅ **Windows** (x86_64)
- ✅ **macOS** (x86_64, Apple Silicon)

### Python版本
- ✅ **Python 3.8**
- ✅ **Python 3.9** 
- ✅ **Python 3.10**
- ✅ **Python 3.11**
- ✅ **Python 3.12**

### 包格式
- 🔧 **Wheel** (.whl) - 各平台预编译包
- 📦 **Source Distribution** (.tar.gz) - 源码包

## 🔧 本地测试

发布前建议本地测试：

```bash
# 测试本地构建
./scripts/test_wheels.sh

# 手动测试
maturin build --release
pip install target/wheels/*.whl --force-reinstall
python -c "import hive_reader_rs; print('✅ 测试成功')"
```

## 🛠 CI/CD 工作流说明

### 1. `build.yml` - 主构建流程
- **触发条件**: 推送到main分支、创建标签、PR
- **功能**: 代码检查、测试、跨平台构建
- **输出**: 构建artifacts

### 2. `wheels.yml` - 专业wheel构建  
- **触发条件**: 标签推送 (v*)
- **功能**: 使用cibuildwheel构建所有平台
- **输出**: 发布到GitHub Releases和PyPI

### 3. `cibuildwheel.toml` - 构建配置
- **配置**: 支持的Python版本、平台设置
- **环境**: Rust工具链、依赖安装

## 📊 发布检查清单

发布前请确认：

- [ ] 代码已测试，所有测试通过
- [ ] 版本号已更新 (Cargo.toml, pyproject.toml)
- [ ] CHANGELOG.md 已更新 (如果有)
- [ ] README.md 已更新文档
- [ ] 没有硬编码的敏感信息
- [ ] GitHub仓库URL已更新
- [ ] PyPI Token已配置 (如果发布到PyPI)

## 🚨 故障排除

### 常见问题

1. **构建失败**
   ```bash
   # 检查Rust版本兼容性
   # 检查依赖是否正确
   # 查看GitHub Actions日志
   ```

2. **PyPI发布失败**
   ```bash
   # 检查PYPI_API_TOKEN是否正确
   # 版本号是否已存在
   # 包名是否冲突
   ```

3. **Cross-compilation失败**
   ```bash
   # Linux aarch64需要交叉编译工具
   # macOS需要Xcode Command Line Tools
   # Windows需要MSVC工具链
   ```

### 调试步骤

1. **本地重现问题**:
   ```bash
   ./scripts/test_wheels.sh
   ```

2. **检查GitHub Actions日志**:
   - 进入Actions页面查看详细日志
   - 关注Rust编译错误
   - 检查依赖安装问题

3. **测试特定平台**:
   ```bash
   # 使用Docker测试Linux
   # 使用VM测试Windows/macOS
   ```

## 📚 相关资源

- [maturin文档](https://github.com/PyO3/maturin)
- [cibuildwheel文档](https://cibuildwheel.readthedocs.io/)
- [GitHub Actions文档](https://docs.github.com/en/actions)
- [PyPI发布指南](https://packaging.python.org/en/latest/tutorials/packaging-projects/)

## 🎯 发布后验证

发布成功后，验证安装：

```bash
# 安装发布的包
pip install hive-reader-rs

# 测试基本功能
python -c "
import hive_reader_rs
with hive_reader_rs.connect_hive() as hive:
    df = hive.query_to_polars('SELECT 1 as test')
    print('✅ 发布版本工作正常')
"
```

---

�� **恭喜！现在你可以自动发布跨平台的Rust+Python包了！**
