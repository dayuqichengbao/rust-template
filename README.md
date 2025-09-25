# {{project-name}}

开箱即用 Rust VSCode 模板 / Ready-to-use Rust VSCode Template

---

## 中文简介

**Rust VSCode 模板** 是一个开箱即用的 Rust 项目模板，专为 VSCode 开发环境设计，包含完整的开发工具链、调试配置和现代单元测试工具。

### 特性

- 保存自动运行 **Clippy**，检查代码规范
- 自动 **rustfmt** 格式化
- **Error Lens** 高亮错误与警告
- **Nextest** 单测集成（支持全部/单个测试）
- **LLDB** 调试配置，支持单步调试
- 初始依赖：`serde`、`log`、`env_logger`
- VSCode 配置开箱即用：主题、图标、终端环境

### 使用场景

适合新建 Rust 项目，快速开始开发，保证代码规范、调试顺畅、测试可用。

---

## English Introduction

**Rust VSCode Template** is a ready-to-use Rust project template designed for VSCode development. It comes with a complete toolchain, debugging setup, and modern testing workflow.

### Features

- Automatically run **Clippy** on save to enforce code style
- Auto **rustfmt** formatting
- **Error Lens** highlights errors and warnings
- **Nextest** testing integration (supports full or single test)
- **LLDB** debugging configuration with step-by-step support
- Preloaded dependencies: `serde`, `log`, `env_logger`
- VSCode configuration ready-to-use: theme, icon pack, terminal environment

### Use Case

Ideal for initializing new Rust projects, ensuring code quality, smooth debugging, and ready-to-use testing setup.

---

## 安装和环境配置 / Installation & Environment Setup

#### 1. 安装 Rust 工具链

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy
```

#### 2. 安装 Nextest

```bash
cargo install cargo-nextest
```
#### 3. 安装 VSCode 插件

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension usernamehw.errorlens
code --install-extension joshpeng.dependee
code --install-extension PKief.material-icon-theme
code --install-extension zhuangtongfa.material-theme

```

#### 4.初始化项目（可选使用 cargo-generate）

```
cargo install cargo-generate
cargo generate --git https://github.com/dayuqichengbao/rust-template.git --name my_project

```

## 使用方法 / How to Use

- 保存自动运行 Clippy

- 运行 Nextest 单测

    - 全部测试：
    ```
    cargo nextest run
    ```
    - 单个测试：
    ```
    cargo nextest run --filter test_name
    ```

## VSCode 配置说明 / VSCode Settings

- 主题 & 图标：One Dark Pro + Material Icon Theme

- 自动格式化：editor.formatOnSave

- 错误高亮：Error Lens 插件

- Rust 分析器：rust-analyzer + Clippy

- 终端环境：可配置中文警告（RUST_LANG=zh-CN）

## LICENSE

MIT / Apache-2.0
