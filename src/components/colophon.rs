use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
  main {
    class: "app-colophon",
  h1 { "Colophon" }
  p {
    "This website was created using the Rust library ",
  a {
    href: "https://dioxuslabs.com/",
    target: "_blank",
    "Dioxus",
  },
  ". To learn how to use Dioxus to make a website like this, see the tutorial "
    a {
      href: "https://www.croftsoft.com/library/tutorials/rust-dioxus-project-setup/",
      target: "_blank",
      "Rust-Dioxus Project Setup"
    }
    "."
  }
  p {
    "The open source repositories for this website are hosted on GitHub:"
  }
  ul {
  li {
  a {
    href: "https://github.com/cryonicscommunity/cryonics-community-dioxus",
    target: "_blank",
    "https://github.com/cryonicscommunity/cryonics-community-dioxus"
  }
  }
  li {
  a {
    href: "https://github.com/cryonicscommunity/cryonics-community-amplify",
    target: "_blank",
    "https://github.com/cryonicscommunity/cryonics-community-amplify"
  }
  }
  }
  }
  }
}
