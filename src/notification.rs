use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::time::Duration;

static NOTIFICATION_ID: AtomicU64 = AtomicU64::new(0);

// 全局配置，用于存储自定义图标和样式
static DEFAULT_ICONS: RwLock<Option<DefaultIcons>> = RwLock::new(None);
static DEFAULT_STYLES: RwLock<Option<DefaultStyles>> = RwLock::new(None);

/// 默认图标配置
#[derive(Clone, Debug)]
pub struct DefaultIcons {
    pub info: String,
    pub success: String,
    pub warning: String,
    pub error: String,
}

/// 默认样式配置
#[derive(Clone, Debug)]
pub struct DefaultStyles {
    pub info: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: u64,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Duration,
    pub custom_icon: Option<String>,
    pub custom_styles: Option<String>, // 完整的 CSS 样式字符串
}

fn generate_id() -> u64 {
    NOTIFICATION_ID.fetch_add(1, Ordering::SeqCst)
}

#[component]
pub fn ToastContainer() -> impl IntoView {
    let (notifications, set_notifications) = signal(vec![]);

    // 提供上下文以便其他组件可以添加通知
    provide_context((notifications, set_notifications));

    view! {
        <style>
            {r#"
                @keyframes slideInRight {
                    from {
                        transform: translateX(100%);
                        opacity: 0;
                    }
                    to {
                        transform: translateX(0);
                        opacity: 1;
                    }
                }
                
                @keyframes slideOutRight {
                    from {
                        transform: translateX(0);
                        opacity: 1;
                    }
                    to {
                        transform: translateX(100%);
                        opacity: 0;
                    }
                }
                
                .notification-item {
                    animation: slideInRight 0.3s ease-out;
                }
                
                .notification-item.exiting {
                    animation: slideOutRight 0.3s ease-in;
                }
                
                .progress-bar {
                    position: absolute;
                    bottom: 2px;
                    left: 5px;
                    right: 5px;
                    height: 4px;
                    background: rgba(255, 255, 255, 0.2);
                    border-radius: 8px;
                    overflow: hidden;
                }
                
                .progress-bar-fill {
                    height: 100%;
                    background: linear-gradient(90deg, rgba(255, 255, 255, 0.9), rgba(255, 255, 255, 0.7));
                    border-radius: 8px;
                    transition: width 0.1s linear;
                    box-shadow: 0 0 10px rgba(255, 255, 255, 0.3);
                }
            "#}
        </style>
        <div
            style:box-sizing="border-box"
            style:position="fixed"
            style:top="20px"
            style:right="20px"
            style:z-index="1000"
            style:gap="10px"
            style:display="flex"
            style:flex-direction="column"
            style:align-items="flex-end"
            style:justify-content="flex-start"
        >
            <For
                each=move || notifications.get()
                key=|notification: &Notification| notification.id
                children=move |notification: Notification| {
                    let id = notification.id;
                    let (is_exiting, set_is_exiting) = signal(false);

                    let start_exit = move || {
                        set_is_exiting.set(true);
                        // 等待动画完成后真正移除
                        set_timeout(
                            move || {
                                set_notifications.update(|notifs: &mut Vec<Notification>| {
                                    notifs.retain(|n| n.id != id);
                                });
                            },
                            std::time::Duration::from_millis(300),
                        );
                    };

                    // 自动移除通知
                    if notification.duration.as_secs() > 0 {
                        set_timeout(
                            move || start_exit(),
                            notification.duration,
                        );
                    }

                    view! {
                        <NotificationItem
                            notification=notification
                            on_close=start_exit
                            is_exiting=is_exiting
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn NotificationItem(
    notification: Notification,
    on_close: impl Fn() + 'static,
    is_exiting: ReadSignal<bool>,
) -> impl IntoView {
    // 使用自定义图标或默认图标
    let icon = notification.custom_icon.clone().unwrap_or_else(|| {
        // 先尝试从全局配置获取
        if let Ok(defaults) = DEFAULT_ICONS.read() {
            if let Some(default_icons) = defaults.as_ref() {
                return match notification.notification_type {
                    NotificationType::Info => default_icons.info.clone(),
                    NotificationType::Success => default_icons.success.clone(),
                    NotificationType::Warning => default_icons.warning.clone(),
                    NotificationType::Error => default_icons.error.clone(),
                };
            }
        }
        // 如果没有全局配置，使用硬编码默认值
        match notification.notification_type {
            NotificationType::Info => "ℹ️".to_string(),
            NotificationType::Success => "✅".to_string(),
            NotificationType::Warning => "⚠️".to_string(),
            NotificationType::Error => "❌".to_string(),
        }
    });

    // 获取默认背景色
    let default_bg = match notification.notification_type {
        NotificationType::Info => "#3b82f6",
        NotificationType::Success => "#10b981",
        NotificationType::Warning => "#f59e0b",
        NotificationType::Error => "#ef4444",
    };

    // 构建基础样式字符串
    let base_styles = format!(
        "background: {}; background-size: cover; background-position: center; \
         color: white; padding: 12px 16px; border-radius: 8px; \
         box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); min-width: 300px; max-width: 400px; \
         display: flex; align-items: center; position: relative; overflow: hidden; \
         transition: all 0.3s linear",
        default_bg
    );

    // 合并样式：自定义样式 > 全局默认样式 > 基础样式
    let final_styles = if let Some(custom) = &notification.custom_styles {
        // 如果有自定义样式，使用它
        format!("{}; {}", base_styles, custom)
    } else if let Ok(defaults) = DEFAULT_STYLES.read() {
        // 否则尝试使用全局默认样式
        if let Some(default_styles) = defaults.as_ref() {
            let global_style = match notification.notification_type {
                NotificationType::Info => &default_styles.info,
                NotificationType::Success => &default_styles.success,
                NotificationType::Warning => &default_styles.warning,
                NotificationType::Error => &default_styles.error,
            };

            if let Some(style) = global_style {
                format!("{}; {}", base_styles, style)
            } else {
                base_styles
            }
        } else {
            base_styles
        }
    } else {
        base_styles
    };

    let total_duration = notification.duration.as_secs_f64();
    let (remaining_time, set_remaining_time) = signal(total_duration);

    // 启动定时器更新剩余时间
    if total_duration > 0.0 {
        let interval_ms = 100; // 每100ms更新一次
        let update_interval = std::time::Duration::from_millis(interval_ms as u64);
        let decrement = interval_ms as f64 / 1000.0;

        let _ = set_interval_with_handle(
            move || {
                set_remaining_time.update(|time| {
                    *time -= decrement;
                    if *time <= 0.0 {
                        *time = 0.0;
                    }
                });
            },
            update_interval,
        );
    }

    view! {
        <div
            class="notification-item"
            class:exiting=move || is_exiting.get()
            style=move || final_styles.clone()
        >
            <span
                style="font-size: 16px; margin-right: 8px; display: flex; align-items: center;"
                inner_html=icon
            />
            <span style="flex: 1;">{notification.message.clone()}</span>
            <button
                on:click=move |_| {
                    on_close();
                }
                style="background: none; border: none; color: white; font-size: 18px; cursor: pointer; padding: 0; margin-left: 8px;"
            >
                "×"
            </button>
            <div class="progress-bar">
                <div
                    class="progress-bar-fill"
                    style:width=move || {
                        let percentage = if total_duration > 0.0 {
                            (remaining_time.get() / total_duration) * 100.0
                        } else {
                            0.0
                        };
                        format!("{}%", percentage)
                    }
                />
            </div>
        </div>
    }
}

// 辅助函数用于添加通知
pub fn add_notification(message: String, notification_type: NotificationType, duration: Duration) {
    use_context::<(
        ReadSignal<Vec<Notification>>,
        WriteSignal<Vec<Notification>>,
    )>()
    .map(|(_, set_notifications)| {
        let id = generate_id();

        set_notifications.update(|notifications| {
            notifications.push(Notification {
                id,
                message,
                notification_type,
                duration,
                custom_icon: None,
                custom_styles: None,
            });
        });
    });
}

// 高级函数：支持自定义样式
pub fn add_notification_custom(
    message: String,
    notification_type: NotificationType,
    duration: Duration,
    custom_icon: Option<String>,
    custom_styles: Option<String>, // 完整的 CSS 样式字符串
) {
    use_context::<(
        ReadSignal<Vec<Notification>>,
        WriteSignal<Vec<Notification>>,
    )>()
    .map(|(_, set_notifications)| {
        let id = generate_id();

        set_notifications.update(|notifications| {
            notifications.push(Notification {
                id,
                message,
                notification_type,
                duration,
                custom_icon,
                custom_styles,
            });
        });
    });
}

// 便捷函数
pub fn show_info(message: impl Into<String>) {
    add_notification(
        message.into(),
        NotificationType::Info,
        Duration::from_secs(5),
    );
}

pub fn show_success(message: impl Into<String>) {
    add_notification(
        message.into(),
        NotificationType::Success,
        Duration::from_secs(5),
    );
}

pub fn show_warning(message: impl Into<String>) {
    add_notification(
        message.into(),
        NotificationType::Warning,
        Duration::from_secs(5),
    );
}

pub fn show_error(message: impl Into<String>) {
    add_notification(
        message.into(),
        NotificationType::Error,
        Duration::from_secs(5),
    );
}

/// 设置默认图标配置
///
/// # 示例
/// ```rust
/// use leptos_notification::{set_default_icons, DefaultIcons};
///
/// set_default_icons(DefaultIcons {
///     info: "ℹ️".to_string(),
///     success: "✅".to_string(),
///     warning: "⚠️".to_string(),
///     error: "❌".to_string(),
/// });
/// ```
pub fn set_default_icons(icons: DefaultIcons) {
    if let Ok(mut default_icons) = DEFAULT_ICONS.write() {
        *default_icons = Some(icons);
    }
}

/// 设置默认样式配置
///
/// # 示例
/// ```rust
/// use leptos_notification::{set_default_styles, DefaultStyles};
///
/// set_default_styles(DefaultStyles {
///     info: Some("background: #3b82f6; padding: 16px; border-radius: 12px;".to_string()),
///     success: Some("background: #10b981; padding: 16px; border-radius: 12px;".to_string()),
///     warning: Some("background: #f59e0b; padding: 16px; border-radius: 12px;".to_string()),
///     error: Some("background: #ef4444; padding: 16px; border-radius: 12px;".to_string()),
/// });
/// ```
pub fn set_default_styles(styles: DefaultStyles) {
    if let Ok(mut default_styles) = DEFAULT_STYLES.write() {
        *default_styles = Some(styles);
    }
}

/// 同时设置默认图标和样式
///
/// # 示例
/// ```rust
/// use leptos_notification::{set_defaults, DefaultIcons, DefaultStyles};
///
/// set_defaults(
///     DefaultIcons {
///         info: "🔵".to_string(),
///         success: "🟢".to_string(),
///         warning: "🟡".to_string(),
///         error: "🔴".to_string(),
///     },
///     DefaultStyles {
///         info: Some("padding: 20px; border-radius: 16px;".to_string()),
///         success: None,
///         warning: None,
///         error: None,
///     }
/// );
/// ```
pub fn set_defaults(icons: DefaultIcons, styles: DefaultStyles) {
    set_default_icons(icons);
    set_default_styles(styles);
}
