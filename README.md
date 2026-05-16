# Leptos Notification

一个基于 Leptos 框架的通知系统，提供美观的 Toast 通知组件，支持多种通知类型和动画效果。

## 特性

- ✨ **四种通知类型**：信息、成功、警告、错误
- 🎬 **流畅动画**：进入和退出动画效果
- ⏱️ **进度条**：可视化显示通知剩余时间
- 🎨 **可定制**：易于自定义样式和行为
- 🚀 **简单易用**：简洁的 API 设计

## 快速开始

### 安装依赖

确保你已经安装了 Rust 和 Trunk：

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Trunk
cargo install trunk
```

### 运行示例

```bash
# 克隆项目
git clone <repository-url>
cd leptos_notification

# 启动开发服务器
trunk serve
```

然后在浏览器中访问 `http://localhost:3000`

## 使用方法

### 基本用法

```rust
use leptos_notification::{ToastContainer, show_info, show_success, show_warning, show_error};

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| show_info("这是一条信息通知")>
                "显示信息"
            </button>
            
            <button on:click=move |_| show_success("操作成功！")>
                "显示成功"
            </button>
            
            <button on:click=move |_| show_warning("请注意这个警告")>
                "显示警告"
            </button>
            
            <button on:click=move |_| show_error("发生了一个错误")>
                "显示错误"
            </button>
            
            {/* 添加 Toast 容器 */}
            <ToastContainer />
        </div>
    }
}
```

### 自定义通知

```rust
use leptos_notification::add_notification;
use std::time::Duration;

// 自定义通知时长和类型
add_notification(
    "自定义通知消息".to_string(),
    NotificationType::Info,
    Duration::from_secs(10),  // 10秒后自动消失
);
```

## API 参考

### 组件

#### `ToastContainer`

通知容器组件，必须添加到应用中才能显示通知。

```rust
view! {
    <ToastContainer />
}
```

### 函数

#### `show_info(message)`

显示信息类型的通知。

- **参数**: `message: impl Into<String>` - 通知消息
- **默认时长**: 5秒

#### `show_success(message)`

显示成功类型的通知。

- **参数**: `message: impl Into<String>` - 通知消息
- **默认时长**: 5秒

#### `show_warning(message)`

显示警告类型的通知。

- **参数**: `message: impl Into<String>` - 通知消息
- **默认时长**: 5秒

#### `show_error(message)`

显示错误类型的通知。

- **参数**: `message: impl Into<String>` - 通知消息
- **默认时长**: 5秒

#### `add_notification(message, notification_type, duration)`

添加自定义通知。

- **参数**:
  - `message: String` - 通知消息
  - `notification_type: NotificationType` - 通知类型
  - `duration: Duration` - 显示时长

### 类型

#### `NotificationType`

通知类型枚举：

- `NotificationType::Info` - 信息（蓝色）
- `NotificationType::Success` - 成功（绿色）
- `NotificationType::Warning` - 警告（橙色）
- `NotificationType::Error` - 错误（红色）

## 项目结构

```
leptos_notification/
├── src/
│   ├── main.rs              # 主应用入口
│   └── notification.rs      # 通知模块
├── index.html               # HTML 模板
├── Cargo.toml               # 项目配置
├── Trunk.toml               # Trunk 构建配置
└── README.md                # 项目文档
```

## 技术栈

- **Leptos** - Rust WebAssembly 框架
- **Trunk** - WASM 构建工具
- **web-sys** - Web API 绑定

## 开发

### 构建项目

```bash
# 检查代码
cargo check

# 构建发布版本
trunk build --release
```

### 调试

在浏览器开发者工具的控制台中查看日志输出。

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

感谢 Leptos 社区提供的优秀框架和工具。
