# Leptos Notification

一个美观的 Leptos 通知系统，支持动画效果和进度条。

## 特性

- ✨ 四种通知类型：Info、Success、Warning、Error
- 🎬 流畅的进入/退出动画
- ⏱️ 可视化进度条
- 🎨 现代化设计
- 🌈 支持自定义背景颜色
- 🖼️ 支持背景图片填充
- 😊 支持自定义图标（Emoji、SVG、HTML）
- 🛠️ 支持完整 CSS 样式自定义
- ⚙️ 支持全局默认配置（图标和样式）

## 安装

```toml
[dependencies]
leptos_notification = "0.2.1"
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

#### 基础函数

```rust
show_info(message: impl Into<String>)      // 信息通知（蓝色）
show_success(message: impl Into<String>)   // 成功通知（绿色）
show_warning(message: impl Into<String>)   // 警告通知（橙色）
show_error(message: impl Into<String>)     // 错误通知（红色）
```

所有通知默认显示 5 秒后自动消失。

#### 全局配置函数

**设置默认图标：**

```rust
use leptos_notification::{set_default_icons, DefaultIcons};

// 在应用启动时设置自定义图标
set_default_icons(DefaultIcons {
    info: "🔵".to_string(),
    success: "🟢".to_string(),
    warning: "🟡".to_string(),
    error: "🔴".to_string(),
});
```

**设置默认样式：**

```rust
use leptos_notification::{set_default_styles, DefaultStyles};

// 为每种通知类型设置自定义样式
set_default_styles(DefaultStyles {
    info: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
    success: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
    warning: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
    error: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
});
```

**同时设置图标和样式：**

```rust
use leptos_notification::{set_defaults, DefaultIcons, DefaultStyles};

set_defaults(
    DefaultIcons {
        info: "ℹ️".to_string(),
        success: "✅".to_string(),
        warning: "⚠️".to_string(),
        error: "❌".to_string(),
    },
    DefaultStyles {
        info: Some("padding: 20px; border-radius: 16px;".to_string()),
        success: None,  // 使用默认样式
        warning: None,
        error: None,
    }
);
```

**提示：**
- 全局配置会影响所有未指定自定义图标的通知
- 如果同时设置了全局配置和单个通知的自定义配置，单个通知的配置优先级更高
- 可以在应用启动时调用这些函数一次，之后所有通知都会使用新配置

#### 高级自定义函数

```rust
use leptos_notification::add_notification_custom;

// 1. 使用 Emoji 图标
add_notification_custom(
    "Emoji 图标通知".to_string(),
    NotificationType::Warning,
    Duration::from_secs(5),
    Some("🚀".to_string()), // Emoji 图标
    None,
);

// 2. 使用 SVG 图标
add_notification_custom(
    "SVG 图标通知".to_string(),
    NotificationType::Success,
    Duration::from_secs(5),
    Some(r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none"><path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="white"/></svg>"#.to_string()),
    None,
);

// 3. 使用 HTML 内容作为图标（支持任何 HTML）
add_notification_custom(
    "HTML 图标通知".to_string(),
    NotificationType::Info,
    Duration::from_secs(5),
    Some(r#"<span style="font-size: 20px;">💡</span>"#.to_string()),
    None,
);

// 自定义背景颜色
add_notification_custom(
    "自定义颜色通知".to_string(),
    NotificationType::Info,
    Duration::from_secs(5),
    None,
    Some("background: #ff6b6b".to_string()), // CSS 样式字符串
);

// 自定义背景图片
add_notification_custom(
    "带背景图的通知".to_string(),
    NotificationType::Success,
    Duration::from_secs(5),
    Some("⭐".to_string()),
    Some("background: url('https://images.unsplash.com/photo-1557683316-973673baf926?w=400&h=200&fit=crop'); background-size: cover; background-position: center;".to_string()),
);

// 完整样式自定义 - 支持所有 CSS 属性
add_notification_custom(
    "完全自定义".to_string(),
    NotificationType::Success,
    Duration::from_secs(5),
    Some("💎".to_string()),
    Some("background: #6366f1; padding: 20px 24px; border-radius: 16px; box-shadow: 0 10px 25px rgba(0,0,0,0.3); min-width: 350px; max-width: 500px; font-size: 18px;".to_string()),
);
```

**提示**: 
- `custom_icon` 参数支持任何 HTML 内容，包括：
  - Emoji 表情（如 "🚀"）
  - SVG 图标（使用原始字符串 `r#"..."#` 避免转义）
  - 任何 HTML 标签（如 `<span>`、`<i>` 等）
  
- `custom_styles` 参数是一个完整的 CSS 样式字符串，可以包含任何 CSS 属性，如：
- `background` - 背景色或背景图片
- `padding` - 内边距
- `border-radius` - 圆角
- `box-shadow` - 阴影
- `min-width` / `max-width` - 宽度限制
- `font-size` - 字体大小
- 以及任何其他 CSS 属性

## 运行示例

```bash
git clone https://github.com/flunas/leptos_notification.git
cd leptos_notification
trunk serve
```

访问 http://localhost:3000

## 许可证

MIT
