use leptos::prelude::*;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

static NOTIFICATION_ID: AtomicU64 = AtomicU64::new(0);

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
    let bg_color = match notification.notification_type {
        NotificationType::Info => "#3b82f6",
        NotificationType::Success => "#10b981",
        NotificationType::Warning => "#f59e0b",
        NotificationType::Error => "#ef4444",
    };
    
    let icon = match notification.notification_type {
        NotificationType::Info => "ℹ️",
        NotificationType::Success => "✅",
        NotificationType::Warning => "⚠️",
        NotificationType::Error => "❌",
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
            style:background-color=bg_color
            style:color="white"
            style:padding="12px 16px"
            style:border-radius="8px"
            style:box-shadow="0 4px 6px rgba(0, 0, 0, 0.1)"
            style:min-width="300px"
            style:max-width="400px"
            style:display="flex"
            style:align-items="center"
            style:position="relative"
            style:overflow="hidden"
            style:transition="all 0.3s linear"
        >
            <span style="font-size: 18px;">{icon}</span>
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
    use_context::<(ReadSignal<Vec<Notification>>, WriteSignal<Vec<Notification>>)>()
        .map(|(_, set_notifications)| {
            let id = generate_id();
            
            set_notifications.update(|notifications| {
                notifications.push(Notification {
                    id,
                    message,
                    notification_type,
                    duration,
                });
            });
        });
}

// 便捷函数
pub fn show_info(message: impl Into<String>) {
    add_notification(message.into(), NotificationType::Info, Duration::from_secs(5));
}

pub fn show_success(message: impl Into<String>) {
    add_notification(message.into(), NotificationType::Success, Duration::from_secs(5));
}

pub fn show_warning(message: impl Into<String>) {
    add_notification(message.into(), NotificationType::Warning, Duration::from_secs(5));
}

pub fn show_error(message: impl Into<String>) {
    add_notification(message.into(), NotificationType::Error, Duration::from_secs(5));
}
