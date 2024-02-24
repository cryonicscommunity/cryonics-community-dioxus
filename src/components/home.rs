use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
  main {
    class: "app-home",
  h1 {
    "Cryonics Community"
  }
  hr { }
  ul {

  li {
  a {
    href: "https://www.meetup.com/cryonicscommunity/",
    target: "_blank",
  "Cryonics Community Dallas"
  }
  }

  li {
  a {
    href: "https://www.crinco.org/",
    target: "_blank",
  "Cryonics Industry Consortium"
  }
  }

  li {
  a {
    href: "https://www.cryothanasia.org/",
    target: "_blank",
  "Cryothanasia"
  }
  }

  li {
  a {
    href: "https://www.organsfor.life/",
    target: "_blank",
  "Organs for Life"
  }
  }

  }
  }
  }
}
