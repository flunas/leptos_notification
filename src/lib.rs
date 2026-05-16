//! # Leptos Notification
//!
//! A beautiful notification system for Leptos applications with animated toasts.
//!
//! ## Features
//!
//! - Four notification types: Info, Success, Warning, Error
//! - Smooth enter and exit animations
//! - Progress bar showing remaining time
//! - Easy to use API
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use leptos_notification::{ToastContainer, show_info, show_success, show_warning, show_error};
//! use leptos::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div>
//!             <button on:click=move |_| show_info("This is an info notification")>
//!                 "Show Info"
//!             </button>
//!             
//!             <ToastContainer />
//!         </div>
//!     }
//! }
//! ```

mod notification;

pub use notification::{
    add_notification, add_notification_custom, set_default_icons, set_default_styles, set_defaults,
    show_error, show_info, show_success, show_warning, DefaultIcons, DefaultStyles, Notification,
    NotificationType, ToastContainer,
};
