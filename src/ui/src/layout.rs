use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
  rsx! {
      div {
          id: "navbar",
          {children}
      }
  }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
  #[layout(WebNavbar)]
  #[route("/")]
  Index {},
  #[route("/subject/:id")]
  Subject { id: i32 },
  #[route("/subjects")]
  Subjects,
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
