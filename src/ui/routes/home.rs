use crate::shared::constants::PKG_VERSION;
use crate::ui::features::home::components::signin_form::SignInForm;
use dioxus::prelude::*;

const CRABJPG: Asset = asset!("../../../icons/invoker.gif");

#[component]
pub fn Home() -> Element {
    rsx! {
        // Hero {}
        main { class: "h-dvh text-white font-sans",
            section { class: "relative w-full h-full",
                // Background image layer
                img {
                    src: CRABJPG,
                    class: "absolute inset-0 w-full h-full object-cover",
                }
                // Optional dark overlay for readability
                div { class: "absolute inset-0 bg-black/50" }

                // Foreground, centered content
                div { class: "relative z-10 w-full h-full flex items-center justify-center p-4",
                    SignInForm {}
                }

                p { class: "absolute bottom-0 right-0 text-sm", "v{PKG_VERSION}" }
            }
        }

    }
}
