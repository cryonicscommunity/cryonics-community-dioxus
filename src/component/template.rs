use super::super::route::Route;
use super::footer::Footer;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  const STYLESHEET: Asset = asset!("/public/stylesheet.css");

  rsx! {
    document::Link {
      href: STYLESHEET,
      rel: "stylesheet",
    }
    Nav { }
    Outlet::<Route> {}
    Footer { }
  }
}
