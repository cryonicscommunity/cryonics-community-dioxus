use super::super::route::Route;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Nav() -> Element {
  rsx! {
    nav {
      class: "app-nav",
      ul {
        li {
          Link {
            to: Route::Home {},
            "Home"
          }
        }
        li {
          Link {
            to: Route::About {},
            "About"
          }
        }
        li {
          Link {
            to: Route::Colophon {},
            "Colophon"
          }
        }
      }
    }
  }
}
