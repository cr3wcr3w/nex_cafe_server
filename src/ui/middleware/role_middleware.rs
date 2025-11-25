use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Staff,
    User,
    // Add more roles as needed
}

#[derive(Clone, Routable, PartialEq)]
pub enum RoleProtectedRoute {
    #[route("/admin/:..route")]
    Admin { route: Vec<Segment> },
    
    #[route("/staff/:..route")]
    Staff { route: Vec<Segment> },
}

#[component]
pub fn RoleProtected(roles: Vec<UserRole>, children: Element) -> Element {
    // In a real app, you would get the current user's role from your auth state
    // For this example, we'll use a signal to simulate the current user's role
    let current_user_role = use_signal(|| UserRole::User);
    
    let has_permission = use_memo(move || {
        roles.contains(&current_user_role())
    });

    rsx! {
        if *has_permission {
            {children}
        } else {
            div { class: "flex items-center justify-center min-h-screen",
                div { class: "text-center",
                    h1 { "Access Denied" }
                    p { "You don't have permission to access this page." }
                    Link { to: "/", "Go back to home" }
                }
            }
        }
    }
}

// Example usage in your app:
// rsx! {
//     Router::<RoleProtectedRoute> {
//         RoleProtectedRoute::Admin { route } => rsx! {
//             RoleProtected {
//                 roles: vec![UserRole::Admin],
//                 "Admin Dashboard"
//             }
//         },
//         RoleProtectedRoute::Staff { route } => rsx! {
//             RoleProtected {
//                 roles: vec![UserRole::Admin, UserRole::Staff],
//                 "Staff Dashboard"
//             }
//         },
//     }
// }
