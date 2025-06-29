# RustFS Manager & win-service-rs

[英文](README.md)｜中文

## 项目概述

`rustfs-manager` 是一个使用 Rust 语言开发的 Windows 服务管理工具，基于 `win-service-rs` 库，旨在提供类似 Linux `systemctl`
的服务管理功能。它用于管理 `rustfs.exe` 或类似服务的生命周期，支持服务的安装、启动、停止、状态查询和卸载。`win-service-rs`
是一个独立的 Rust 库，封装了 Windows 服务控制管理器（SCM）的 API，提供安全、易用的接口，适用于任何 Windows 服务管理需求。

该项目结合了 Rust 的内存安全性和 `windows-rs` 的强大功能，提供了高效、可靠的服务管理解决方案。无论是开发企业级应用还是自动化脚本，
`rustfs-manager` 和 `win-service-rs` 都能提供灵活的支持。

## 功能特性

- **服务管理**：支持服务的安装、启动、停止、状态查询和卸载。
- **配置管理**：通过 TOML 配置文件动态指定服务参数（如服务名、可执行路径）。
- **结构化日志**：使用 `tracing` 输出 JSON 格式日志，便于集成到 ELK 或 Graylog。
- **异步支持**：通过 `tokio` 提供异步接口，优化高并发场景。
- **安全性**：基于 `windows-rs` 的安全绑定，确保内存安全和操作可靠性。
- **生产环境优化**：提供 PowerShell 部署脚本，支持自动化部署。

## 安装

1. **安装 Rust**：
   确保已安装 Rust 工具链，运行以下命令更新到最新稳定版本：
   ```bash
   rustup update stable
   ```

2. **克隆项目**：
   ```bash
   git clone https://github.com/houseme/win-service-rs.git
   cd win-service-rs/examples/rustfs_manager
   ```

3. **安装依赖**：
   项目依赖已包含在 `Cargo.toml` 中，运行以下命令构建：
   ```bash
   cargo build --release
   ```

4. **配置环境**：
   编辑 `config/config.toml`，更新 `exe_path` 等参数，确保 `rustfs.exe` 路径有效。

## 使用方法

`rustfs-manager` 提供了一个命令行接口（CLI），支持以下命令：

- **安装服务**：
  ```bash
  cargo run -- install
  ```
  将 `rustfs.exe` 注册为 Windows 服务。

- **启动服务**：
  ```bash
  cargo run -- start
  ```

- **停止服务**：
  ```bash
  cargo run -- stop
  ```

- **查询状态**：
  ```bash
  cargo run -- status
  ```

- **卸载服务**：
  ```bash
  cargo run -- uninstall
  ```

运行 `cargo run -- --help` 查看完整命令列表。

## 部署指南

1. **构建生产二进制文件**：
   ```bash
   cargo build --release
   ```

2. **复制文件**：
   将 `target/release/rustfs-manager.exe` 和 `config/config.toml` 复制到目标服务器。

3. **执行部署脚本**：
   使用提供的 PowerShell 脚本自动化部署：
   ```powershell
   .\scripts\deploy.ps1
   ```

4. **验证部署**：
   检查服务状态：
   ```powershell
   sc query RustFSManager
   ```

## win-service-rs 使用说明

`win-service-rs` 是一个独立的 Rust 库，封装了 Windows SCM API，提供安全、易用的服务管理接口。以下是使用示例：

1. **添加依赖**：
   在 `Cargo.toml` 中添加：
   ```toml
   [dependencies]
   win-service-rs = { version = "0.1.0", features = ["async"] }
   ```

2. **示例代码**：
   ```rust
   use win_service_rs::{ServiceConfig, ServiceManager, StartType};

   #[tokio::main]
   async fn main() -> Result<(), win_service_rs::Error> {
       let mut manager = ServiceManager::new("RustFS")?;
       let config = ServiceConfig::new(
           "C:\\path\\to\\rust-fs.exe".to_string(),
           "RustFS Service".to_string(),
       )
       .with_description("RustFS File System Service".to_string())
       .with_start_type(StartType::Demand);

       manager.install(&config)?;
       manager.async_start().await?;
       let status = manager.async_status().await?;
       println!("Service status: {:?}", status);
       manager.async_stop().await?;
       manager.uninstall()?;
       Ok(())
   }
   ```

3. **异步支持**：
   启用 `async` 特性后，可使用 `async_start`、`async_stop` 和 `async_status` 方法，适合高并发场景。

4. **错误处理**：
   库提供了自定义 `Error` 类型，处理 SCM 错误（如权限不足、服务不存在）。

## 注意事项

- **权限要求**：运行 `rustfs-manager` 或 `win-service-rs` 需要管理员权限。
- **日志集成**：建议将 `tracing` 的 JSON 日志输出到文件，集成到 ELK 或 Graylog。
- **安全性**：限制 `rustfs.exe` 和 `rustfs-manager` 的文件权限，防止未授权访问。
- **扩展性**：可通过添加日志监控或健康检查功能扩展 `win-service-rs`。

## 参考资料

1. **windows-rs 官方资源**：
    - [GitHub: windows-rs](https://github.com/microsoft/windows-rs)
    - [Rust for Windows 文档](https://learn.microsoft.com/zh-cn/windows/dev-environment/rust/rust-for-windows)
2. **Windows API 文档**：
    - [OpenSCManager](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
    - [CreateService](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
    - [StartService](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)
    - [Service Control Manager](https://learn.microsoft.com/en-us/windows/win32/services/service-control-manager)
3. **Rust 生态系统**：
    - [Rust 官方文档](https://www.rust-lang.org/zh-CN/)
    - [Tokio 文档](https://tokio.rs/docs/tokio/)
    - [Clap 文档](https://docs.rs/clap/latest/clap/)
    - [Serde 文档](https://serde.rs/)
    - [Config 文档](https://docs.rs/config/latest/config/)
    - [Tracing 文档](https://docs.rs/tracing/latest/tracing/)

## 许可证

本项目采用 MIT、Apache 许可证，详见见 `LICENSE-APACHE`、`LICENSE-MIT` 文件。
