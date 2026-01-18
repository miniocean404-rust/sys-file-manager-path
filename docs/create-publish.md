# 如何发布 Rust Crate 到 crates.io

本文档介绍如何将 Rust 包（Crate）发布到官方仓库 [crates.io](https://crates.io/)。

## 1. 包名来源

发布的包名称完全取决于 `Cargo.toml` 文件中 `[package]` 部分的 `name` 字段。

例如：

```toml
[package]
name = "my-awesome-crate"  # <--- 这就是发布后的包名
version = "0.1.0"
edition = "2021"
```

**注意**：

- 包名必须在 crates.io 上是唯一的。如果名称已被占用，发布将失败。
- 使用 `cargo search <name>` 可以检查名称是否已被使用。

## 2. 发布前的准备

在发布之前，你需要确保 `Cargo.toml` 中包含必要的元数据。crates.io 要求以下字段至少包含一部分：

- `description`: 包的简短描述。
- `license` 或 `license-file`: 许可证（如 "MIT" 或 "Apache-2.0"）。
- `documentation`: 文档链接（可选，通常自动链接到 docs.rs）。
- `repository`: 源代码仓库链接（可选）。
- `homepage`: 主页链接（可选）。

示例：

```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
authors = ["miniocean404@qq.com"]
edition = "2021"
description = "获取 Mac Finder、Windows Explore 窗口的路径"
license = "MIT"
repository = "https://github.com/miniocean404-rust/sys-explore"
```

## 3. 发布步骤

### 步骤 1: 注册并获取 API Token

1. 访问 [crates.io](https://crates.io/)。
2. 使用 GitHub 账号登录。
3. 进入 [Account Settings](https://crates.io/me) (个人设置)。
4. 创建一个新的 API Token。

### 步骤 2: 本地登录

在终端中运行以下命令，将 API Token 保存到本地（只需执行一次）：

```bash
cargo login <your-api-token>
```

这会将 token 保存到 `~/.cargo/credentials` 文件中。

### 步骤 3: 检查包内容

你可以使用以下命令查看将要上传的文件列表，确保没有包含敏感文件或排除了必要文件：

```bash
cargo package --list
```

### 步骤 4: 发布

运行以下命令将包发布到 crates.io：

```bash
cargo publish
```

Cargo 会先打包你的项目，验证依赖，并上传到 crates.io。如果是首次发布，crates.io 会立即注册该包名。

## 4. 常见问题

- **Dry Run**: 如果你想模拟发布过程而不实际上传，可以使用 `cargo publish --dry-run`。
- **撤销发布**: 发布后的版本**不能删除**，这是为了保证依赖该版本的项目不会破坏。但你可以使用 `cargo yank --vers <version>` 撤回某个版本，阻止新项目依赖它。
- **工作区 (Workspace)**: 如果你在一个包含多个 crate 的工作区中，需要分别进入每个 crate 的目录进行发布，或者使用脚本按依赖顺序发布。
