use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_notification::{ToastContainer, show_info, show_success, show_warning, show_error};

#[component]
fn Demo() -> impl IntoView {
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h1>"Leptos Notification Demo"</h1>
            
            <div style="display: flex; gap: 10px; margin-top: 20px; flex-wrap: wrap;">
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
            
            <ToastContainer />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Demo);
}
