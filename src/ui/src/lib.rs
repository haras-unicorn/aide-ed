#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use dioxus::prelude::*;
use dioxus_router::components::{Link, Outlet};

mod views;
use views::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
  #[layout(WebNavbar)]
  #[route("/")]
  Index {},
  #[route("/subject/:id")]
  Subject { id: String },
  #[route("/subjects")]
  Subjects,
}

pub fn app() -> Element {
  rsx! {
    Router::<Route> {}
  }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
  rsx! {
    Navbar {
      Link {
        to: Route::Index {},
        "Index"
      }
      Link {
        to: Route::Subjects,
        "Subjects"
      }
    }

    Outlet::<Route> {}
  }
}

#[component]
pub fn Navbar(children: Element) -> Element {
  rsx! {
      div {
          id: "navbar",
          {children}
      }
  }
}
