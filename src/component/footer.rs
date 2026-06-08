use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Footer() -> Element {
  rsx! {
  footer {
    class: "app-footer",
  hr { }
  "Cryonics Community © 2024-2026 "
  a {
    href: "https://github.com/cryonicscommunity",
    target: "_blank",
    "Cryonics Community Contributors",
  }
  "."
  }
  }
}
