use dioxus::prelude::*;
use tracing::info;

use crate::ui::models::login_model::LoginRequest;
use crate::ui::shared::http::get_client;
#[component]
pub fn SignInForm() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut is_loading = use_signal(|| false);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        is_loading.set(true);
        error.set(None);

        let email = email.read().clone();
        let password = password.read().clone();

        info!("email: {}, password: {}", email, password);

        spawn({
            let nav = use_navigator();

            async move {
                let login_request = LoginRequest {
                    email: email.to_string(),
                    password: password.to_string(),
                };

                let response = get_client()
                    .post("http://localhost:3000/api/auth/login")
                    .json(&login_request)
                    .send()
                    .await;

                // console reponse
                info!("Response: {:#?}", response);

                match response {
                    Ok(response) => {
                        if response.status().is_success() {
                            nav.push("/dashboard");
                        } else {
                            info!("Invalid email or password");
                            error.set(Some("Invalid email or password".to_string()));
                        }
                    }
                    Err(_err) => {
                        info!("Failed to connect to server");
                        error.set(Some("Failed to connect to server".to_string()));
                    }
                }
                is_loading.set(false);
            }
        });
    };

    rsx! {
        div { class: "w-full h-full flex items-center justify-center",
            div { class: "relative w-[420px] rounded-2xl border border-base-300 bg-base-200/20 shadow-xl p-6 md:p-8 text-white transition-all duration-300",

                // Content
                div { class: "relative z-10",
                    // Header
                    h1 { class: "text-primary text-4xl font-serif font-bold tracking-tight text-center",
                        "Welcome back!"
                    }
                    p { class: "text-white/70 text-sm mt-1 text-center",
                        "Sign in to continue to your account"
                    }

                    // Form
                    form { onsubmit: handle_submit, class: "mt-6 space-y-4",
                        // Email
                        div { class: "space-y-2",
                            label {
                                class: "text-sm text-white/80",
                                r#for: "email",
                                "Email"
                            }
                            input {
                                id: "email",
                                name: "email",
                                r#type: "email",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                disabled: is_loading,
                                required: true,
                                placeholder: "invoker@example.com",
                                class: "w-full rounded-lg bg-base-200/20 text-white placeholder-white/50 px-4 py-3 outline-none ring-1 ring-base-300 focus:ring-2 focus:ring-primary transition",
                            }
                        }
                        // Password
                        div { class: "space-y-2",
                            label {
                                class: "text-sm text-white/80",
                                r#for: "password",
                                "Password"
                            }
                            input {
                                id: "password",
                                name: "password",
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{password}",
                                disabled: is_loading,
                                required: true,
                                oninput: move |e| password.set(e.value()),
                                class: "w-full rounded-lg bg-base-200/20 text-white placeholder-white/50 px-4 py-3 outline-none ring-1 ring-base-300 focus:ring-2 focus:ring-primary transition",
                            }
                        }
                        // Actions
                        button {
                            disabled: is_loading,
                            class: "text-primary cursor-pointer text-sm",
                            "Forgot password?"
                        }
                        button {
                            r#type: "submit",
                            disabled: is_loading,
                            class: "cursor-pointer w-full mt-2 rounded-lg bg-primary active:bg-primary px-4 py-3 font-medium shadow-lg text-black transition",
                            if is_loading() {
                                span { class: "loading loading-spinner loading-sm" }
                                "Signing in..."
                            } else {
                                "Sign in"
                            }
                        }
                    }

                    // Footer
                    div { class: "mt-6 text-center text-sm text-white/70",
                        span { "Don't have an account? " }
                        p { class: "text-primary", "Go to Casher" }
                    }
                }
            }
        }
    }
}
