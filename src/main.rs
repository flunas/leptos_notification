mod notification;

use leptos::prelude::*;
use leptos::mount::mount_to_body;
use web_sys::console;
use notification::{ToastContainer, show_info, show_success, show_warning, show_error};

#[component]
fn Demo() -> impl IntoView {
    let btn_info = move |_| {
        show_info("这是一条信息通知");
    };
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Leptos 通知系统演示"</h1>
            
            <div style="display: flex; gap: 10px; margin-top: 20px; flex-wrap: wrap;">
                <button 
                    on:click=btn_info
                    style="padding: 10px 15px; background-color: #3b82f6; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "显示信息通知"
                </button>
                
                <button 
                    on:click=move |_| {
                        console::log_1(&"成功通知按钮被点击".into());
                        show_success("操作成功完成!");
                    }
                    style="padding: 10px 15px; background-color: #10b981; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "显示成功通知"
                </button>
                
                <button 
                    on:click=move |_| {
                        console::log_1(&"警告通知按钮被点击".into());
                        show_warning("请注意这个警告");
                    }
                    style="padding: 10px 15px; background-color: #f59e0b; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "显示警告通知"
                </button>
                
                <button 
                    on:click=move |_| {
                        console::log_1(&"错误通知按钮被点击".into());
                        show_error("发生了一个错误");
                    }
                    style="padding: 10px 15px; background-color: #ef4444; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "显示错误通知"
                </button>
            </div>
            
            // 添加 Toast 容器
            <ToastContainer />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console::log_1(&"Leptos Notification Demo started!".into());
    mount_to_body(Demo);
}
