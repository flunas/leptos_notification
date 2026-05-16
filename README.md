# Leptos Notification

一个美观的 Leptos 通知系统，支持动画效果和进度条。

## 特性

- ✨ 四种通知类型：Info、Success、Warning、Error
- 🎬 流畅的进入/退出动画
- ⏱️ 可视化进度条
- 🎨 现代化设计

## 安装

```toml
[dependencies]
leptos_notification = "0.1.0"
```

## 快速开始

```rust
use leptos_notification::{ToastContainer, show_info, show_success};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| show_info("这是一条通知")>
                "显示通知"
            </button>
            <ToastContainer />
        </div>
    }
}
```

## API

### 组件

- `ToastContainer` - 通知容器（必须添加到应用中）

### 函数

```rust
show_info(message: impl Into<String>)      // 信息通知（蓝色）
show_success(message: impl Into<String>)   // 成功通知（绿色）
show_warning(message: impl Into<String>)   // 警告通知（橙色）
show_error(message: impl Into<String>)     // 错误通知（红色）
```

所有通知默认显示 5 秒后自动消失。

## 运行示例

```bash
git clone https://github.com/flunas/leptos_notification.git
cd leptos_notification
trunk serve
```

访问 http://localhost:3000

## 许可证

MIT
