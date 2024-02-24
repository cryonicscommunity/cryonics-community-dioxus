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
  h2 {
    "Discussion List"
  }
  p {
  r#"Help shape the future of the Cryonics Community by joining our Google
  Groups discussion list:"#
  br { }
  a {
    href: "https://groups.google.com/g/cryonicscommunity/",
    target: "_blank",
  "https://groups.google.com/g/cryonicscommunity/"
  }
  }
  hr { }
  h2 {
    "Related Websites"
  }
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
