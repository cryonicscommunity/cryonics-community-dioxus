use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
  render! {
  main {
    class: "app-colophon",
  h1 { "About" }
  p {
    "Please contact ",
  a {
    href: "https://www.croftpress.com/david/",
    target: "_blank",
    "David Wallace Croft",
  },
  " to update this website."
  }
  p {
    "For more about Cryonics Community, please also see my "
  a {
    href:
      "https://david-wallace-croft.blogspot.com/search?q=cryonics+community",
    target: "_blank",
    "weblog"
  }
  "."
  }
  }
  }
}
