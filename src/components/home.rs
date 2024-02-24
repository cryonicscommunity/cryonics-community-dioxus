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
    "Related Articles"
  }
  ul {
  li {
  a {
    href: "https://david-wallace-croft.blogspot.com/2024/02/cryonics-community.html",
    target: "_blank",
  "Cryonics Community"
  }
  }
  li {
  a {
    href: "https://www.alcor.org/docs/cryonics-magazine-2021-02.pdf",
    target: "_blank",
  "A Look Back: Attempts to Establish a Cryonics Community"
  }
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
    href: "https://discord.gg/cryosphere",
    target: "_blank",
  "Cryosphere Discord Server"
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
