# RustFS Manager & win-service-rs

English｜[中文](README_zh.md)

## Project Overview

`rustfs-manager` is a Windows service management tool developed in Rust, built on top of the `win-service-rs` library,
designed to provide functionality similar to Linux's `systemctl`. It manages the lifecycle of `rustfs.exe` or similar
services, supporting operations such as installation, starting, stopping, status querying, and uninstallation.
`win-service-rs` is a standalone Rust crate that encapsulates the Windows Service Control Manager (SCM) APIs, offering a
safe and user-friendly interface for any Windows service management needs.

This project leverages Rust's memory safety and the robust capabilities of `windows-rs` to deliver an efficient and
reliable service management solution. Whether for enterprise applications or automation scripts, `rustfs-manager` and
`win-service-rs` provide flexible support.

## Features

- **Service Management**: Supports installation, starting, stopping, status querying, and uninstallation of services.
- **Configuration Management**: Dynamically configure service parameters (e.g., service name, executable path) via TOML
  files.
- **Structured Logging**: Uses `tracing` to output JSON-formatted logs, facilitating integration with ELK or Graylog.
- **Asynchronous Support**: Provides async interfaces via `tokio` for high-concurrency scenarios.
- **Safety**: Built on `windows-rs` safe bindings, ensuring memory safety and reliable operations.
- **Production Optimization**: Includes PowerShell deployment scripts for automated deployment.

## Installation

1. **Install Rust**:
   Ensure the Rust toolchain is installed and updated to the latest stable version:
   ```bash
   rustup update stable
   ```

2. **Clone the Project**:
   ```bash
   git clone https://github.com/houseme/win-service-rs.git
   cd win-service-rs/examples/rustfs_manager
   ```

3. **Install Dependencies**:
   Dependencies are specified in `Cargo.toml`. Build the project with:
   ```bash
   cargo build --release
   ```

4. **Configure Environment**:
   Edit `config/config.toml` to update parameters like `exe_path`, ensuring the `rustfs.exe` path is valid.

## Usage

`rustfs-manager` provides a command-line interface (CLI) with the following commands:

- **Install Service**:
  ```bash
  cargo run -- install
  ```
  Registers `rustfs.exe` as a Windows service.

- **Start Service**:
  ```bash
  cargo run -- start
  ```

- **Stop Service**:
  ```bash
  cargo run -- stop
  ```

- **Query Status**:
  ```bash
  cargo run -- status
  ```

- **Uninstall Service**:
  ```bash
  cargo run -- uninstall
  ```

Run `cargo run -- --help` to see the full list of commands.

## Deployment Guide

1. **Build Production Binary**:
   ```bash
   cargo build --release
   ```

2. **Copy Files**:
   Copy `target/release/rustfs-manager.exe` and `config/config.toml` to the target server.

3. **Run Deployment Script**:
   Use the provided PowerShell script for automated deployment:
   ```powershell
   .\scripts\deploy.ps1
   ```

4. **Verify Deployment**:
   Check service status:
   ```powershell
   sc query RustFSManager
   ```

## win-service-rs Usage

`win-service-rs` is a standalone Rust crate that encapsulates Windows SCM APIs, providing a safe and user-friendly
interface for service management. Here's how to use it:

1. **Add Dependency**:
   Add to your `Cargo.toml`:
   ```toml
   [dependencies]
   win-service-rs = { version = "0.1.0", features = ["async"] }
   ```

2. **Example Code**:
   ```rust
   use win_service_rs::{ServiceConfig, ServiceManager, StartType};

   #[tokio::main]
   async fn main() -> Result<(), win_service_rs::Error> {
       let mut manager = ServiceManager::new("RustFS")?;
       let config = ServiceConfig::new(
           "C:\\path\\to\\rustfs.exe".to_string(),
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

3. **Asynchronous Support**:
   With the `async` feature enabled, use `async_start`, `async_stop`, and `async_status` methods for high-concurrency
   scenarios.

4. **Error Handling**:
   The crate provides a custom `Error` type to handle SCM errors (e.g., access denied, service not found).

## Notes

- **Permissions**: Running `rustfs-manager` or `win-service-rs` requires administrator privileges.
- **Log Integration**: Redirect `tracing` JSON logs to files for integration with ELK or Graylog.
- **Security**: Restrict file permissions for `rustfs.exe` and `rustfs-manager` to prevent unauthorized access.
- **Extensibility**: Extend `win-service-rs` with features like log monitoring or health checks.

## References

1. **windows-rs Resources**:
    - [GitHub: windows-rs](https://github.com/microsoft/windows-rs)
    - [Rust for Windows Documentation](https://learn.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows)
2. **Windows API Documentation**:
    - [OpenSCManager](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
    - [CreateService](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
    - [StartService](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)
    - [Service Control Manager](https://learn.microsoft.com/en-us/windows/win32/services/service-control-manager)
3. **Rust Ecosystem**:
    - [Rust Official Documentation](https://www.rust-lang.org/)
    - [Tokio Documentation](https://tokio.rs/docs/tokio/)
    - [Clap Documentation](https://docs.rs/clap/latest/clap/)
    - [Serde Documentation](https://serde.rs/)
    - [Config Documentation](https://docs.rs/config/latest/config/)
    - [Tracing Documentation](https://docs.rs/tracing/latest/tracing/)

## License

This project is licensed under the MIT ,Apache License. See the `LICENSE-APACHE`,`LICENSE-MIT` file for details.