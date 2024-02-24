use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
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
