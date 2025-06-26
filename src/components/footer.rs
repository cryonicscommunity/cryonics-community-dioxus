use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
  footer {
    class: "app-footer",
  hr { }
  "Cryonics Community © 2025 "
  a {
    href: "https://github.com/cryonicscommunity",
    target: "_blank",
    "Cryonics Community Contributors",
  }
  "."
  }
  }
}
