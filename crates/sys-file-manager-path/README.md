# sys-file-manager-path

跨平台获取系统文件管理器当前窗口路径的 Rust 库。

## 功能特性

- 🪟 **Windows 支持**：获取 Windows 资源管理器（Explorer）当前激活窗口的路径
- 🍎 **macOS 支持**：获取 macOS Finder 当前激活窗口的路径
- 🔒 **类型安全**：使用 Rust 类型系统保证安全性
- 🚀 **跨平台**：统一的 API 接口，自动适配不同操作系统

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
sys-file-manager-path = "0.1.1"
```

## 使用示例

### 基础用法

```rust
use sys_file_manager_path::export::dir::get_os_explore_info;

fn main() -> anyhow::Result<()> {
    unsafe {
        let info = get_os_explore_info()?;

        println!("窗口标题: {}", info.title);
        println!("当前路径: {}", info.dir);
        println!("执行程序: {}", info.exec);
        println!("是否激活: {}", info.is_active);
        println!("平台: {}", info.platform.as_ref());

        #[cfg(target_os = "windows")]
        println!("窗口句柄: {}", info.hwnd_id);

        #[cfg(target_os = "macos")]
        println!("Bundle ID: {}", info.bundle_id);
    }

    Ok(())
}
```

### 平台特定用法

#### Windows

```rust
#[cfg(target_os = "windows")]
use sys_file_manager_path::windows::index::get_explore_info;

#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    unsafe {
        let info = get_explore_info()?;
        println!("资源管理器路径: {}", info.dir);
        println!("窗口句柄: {}", info.hwnd_id);
    }
    Ok(())
}
```

#### macOS

```rust
#[cfg(target_os = "macos")]
use sys_file_manager_path::macos::index::get_finder_info;

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    unsafe {
        let info = get_finder_info()?;
        println!("Finder 路径: {}", info.dir);
        println!("Bundle ID: {}", info.bundle_id);
    }
    Ok(())
}
```

## API 文档

### 核心类型

#### `AppInfo`

包含文件管理器窗口的完整信息：

```rust
pub struct AppInfo {
    pub hwnd_id: isize,        // Windows 窗口句柄
    pub bundle_id: String,     // macOS Bundle ID
    pub title: String,         // 窗口标题
    pub is_active: bool,       // 是否为激活窗口
    pub dir: String,           // 当前目录路径
    pub exec: String,          // 执行程序路径
    pub platform: Platform,    // 当前平台
}
```

#### `Platform`

平台枚举：

```rust
pub enum Platform {
    Unknown,
    Windows,
    MacOS,
}
```

### 主要函数

#### `get_os_explore_info()`

跨平台统一接口，自动根据操作系统调用对应实现：

```rust
pub unsafe fn get_os_explore_info() -> anyhow::Result<AppInfo>
```

- **Windows**：调用 `get_explore_info()`
- **macOS**：调用 `get_finder_info()`
- **Linux**：返回默认值（暂未实现）

## 实现原理

### Windows 实现

通过 Windows API 实现：

1. 使用 `CoCreateInstance` 创建 `ShellWindows` COM 对象
2. 枚举所有资源管理器窗口
3. 通过 `GetForegroundWindow` 获取当前激活窗口
4. 使用 `IShellBrowser` 接口获取窗口路径
5. 匹配进程 ID 确定激活窗口的路径

关键技术：
- COM 接口调用
- Windows Shell API
- 进程 ID 匹配

### macOS 实现

通过 Objective-C 运行时和 AppleScript 实现：

1. 使用 `NSWorkspace` 获取前台应用信息
2. 检查是否为 Finder 应用（`com.apple.finder`）
3. 通过 `osascript` 执行 AppleScript 获取 Finder 路径
4. 如果获取失败，返回桌面路径作为后备

关键技术：
- Objective-C 运行时（`objc` crate）
- NSWorkspace API
- AppleScript 命令执行

## 安全性说明

⚠️ **重要**：本库的核心函数使用 `unsafe` 标记，因为涉及：

- **Windows**：COM 接口调用、原始指针操作
- **macOS**：Objective-C 运行时调用、外部进程执行

使用时需要注意：
- 确保在正确的线程上下文中调用（Windows COM 需要初始化）
- 处理可能的错误情况（窗口不存在、权限不足等）
- 不要在多线程环境中并发调用（除非正确处理 COM 初始化）

## 依赖项

### 核心依赖

- `anyhow` - 错误处理
- `tokio` - 异步运行时
- `urlencoding` - URL 编码
- `strum` / `strum_macros` - 枚举工具

### 平台特定依赖

#### Windows
- `windows` - Windows API 绑定

#### macOS
- `cocoa` - Cocoa 框架绑定
- `objc` / `objc2` - Objective-C 运行时

## 限制与已知问题

1. **Linux 支持**：目前仅返回默认值，未实现实际功能
2. **多窗口场景**：仅获取当前激活的文件管理器窗口
3. **权限要求**：
   - macOS 可能需要辅助功能权限
   - Windows 需要足够的进程访问权限
4. **线程安全**：Windows COM 调用需要在正确的线程模型下执行

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 参考资料

### Windows
- [Get current selection in Windows Explorer](https://stackoverflow.com/questions/8292953/get-current-selection-in-windowsexplorer-from-a-c-sharp-application)
- [Get the Windows Explorer path which has the focus](https://stackoverflow.com/questions/27590086/c-sharp-get-the-windows-explore-path-which-has-the-focus)

### Rust
- [Get path to selected files in active Explorer window](https://stackoverflow.com/questions/73311644/get-path-to-selected-files-in-active-explorer-window)
