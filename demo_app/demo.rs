use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_notification::{
    add_notification_custom, set_defaults, show_error, show_info, show_success, show_warning,
    DefaultIcons, DefaultStyles, NotificationType, ToastContainer,
};
use std::time::Duration;

#[component]
fn Demo() -> impl IntoView {
    // 设置自定义默认图标和样式（可选）
    // 取消下面的注释来测试自定义默认配置
    /*
    set_defaults(
        DefaultIcons {
            info: "🔵".to_string(),
            success: "🟢".to_string(),
            warning: "🟡".to_string(),
            error: "🔴".to_string(),
        },
        DefaultStyles {
            info: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
            success: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
            warning: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
            error: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
        }
    );
    */

    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Leptos Notification Demo"</h1>

            <div style="display: flex; gap: 10px; margin-top: 20px; flex-wrap: wrap;">
                <button
                    on:click=move |_| {
                        set_defaults(
                            DefaultIcons {
                                info: "🔵".to_string(),
                                success: "🟢".to_string(),
                                warning: "🟡".to_string(),
                                error: "🔴".to_string(),
                            },
                            DefaultStyles {
                                info: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
                                success: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
                                warning: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
                                error: Some("padding: 16px 20px; border-radius: 12px;".to_string()),
                            }
                        );
                        show_info("默认配置已更新！")
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Set Custom Defaults"
                </button>

                <button
                    on:click=move |_| show_info("This is an info notification")
                    style="padding: 10px 15px; background-color: #3b82f6; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Show Info"
                </button>

                <button
                    on:click=move |_| show_success("Operation completed successfully!")
                    style="padding: 10px 15px; background-color: #10b981; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Show Success"
                </button>

                <button
                    on:click=move |_| show_warning("Please pay attention to this warning")
                    style="padding: 10px 15px; background-color: #f59e0b; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Show Warning"
                </button>

                <button
                    on:click=move |_| show_error("An error occurred")
                    style="padding: 10px 15px; background-color: #ef4444; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Show Error"
                </button>
            </div>

            <h2 style="margin-top: 30px;">"Custom Notifications"</h2>
            <div style="display: flex; gap: 10px; margin-top: 20px; flex-wrap: wrap;">
                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Custom Color".to_string(),
                            NotificationType::Info,
                            Duration::from_secs(5),
                            None,
                            Some("background: #9333ea".to_string()),
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Custom Color"
                </button>

                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Custom Icon 🚀".to_string(),
                            NotificationType::Success,
                            Duration::from_secs(5),
                            Some("🚀".to_string()),
                            None,
                        );
                    }
                    style="padding: 10px 15px; background-color: #10b981; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Emoji Icon"
                </button>

                <button
                    on:click=move |_| {
                        let svg_icon = r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z" fill="white" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"#.to_string();
                        add_notification_custom(
                            "SVG Star Icon".to_string(),
                            NotificationType::Info,
                            Duration::from_secs(5),
                            Some(svg_icon),
                            None,
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "SVG Icon"
                </button>

                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Gradient BG".to_string(),
                            NotificationType::Info,
                            Duration::from_secs(5),
                            Some("⭐".to_string()),
                            Some("background: url('https://images.unsplash.com/photo-1557683316-973673baf926?w=400&h=200&fit=crop'); background-size: cover; background-position: center;".to_string()),
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Image BG 1"
                </button>

                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Ocean BG".to_string(),
                            NotificationType::Success,
                            Duration::from_secs(5),
                            Some("🌊".to_string()),
                            Some("background: url('https://images.unsplash.com/photo-1507525428034-b723cf961d3e?w=400&h=200&fit=crop'); background-size: cover; background-position: center;".to_string()),
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Image BG 2"
                </button>

                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Sunset BG".to_string(),
                            NotificationType::Warning,
                            Duration::from_secs(5),
                            Some("🌅".to_string()),
                            Some("background: url('https://images.unsplash.com/photo-1495616811223-4d98c6e9c869?w=400&h=200&fit=crop'); background-size: cover; background-position: center;".to_string()),
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #fa709a 0%, #fee140 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Image BG 3"
                </button>

                <button
                    on:click=move |_| {
                        let check_icon = r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M20 6L9 17L4 12" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>"#.to_string();
                        add_notification_custom(
                            "Check Icon".to_string(),
                            NotificationType::Success,
                            Duration::from_secs(5),
                            Some(check_icon),
                            None,
                        );
                    }
                    style="padding: 10px 15px; background-color: #10b981; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "SVG Check"
                </button>

                <button
                    on:click=move |_| {
                        add_notification_custom(
                            "Full Custom".to_string(),
                            NotificationType::Success,
                            Duration::from_secs(5),
                            Some("💎".to_string()),
                            Some("background: #6366f1; padding: 20px 24px; border-radius: 8px; box-shadow: 0 10px 25px rgba(0,0,0,0.3); min-width: 350px; max-width: 500px; font-size: 18px;".to_string()),
                        );
                    }
                    style="padding: 10px 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Full Custom"
                </button>
            </div>

            <ToastContainer />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Demo);
}
