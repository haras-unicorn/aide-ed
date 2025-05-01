use crate::Route;
use dioxus::prelude::*;
use dioxus_router::components::Link;

#[component]
pub fn Index() -> Element {
  rsx! {
    div {
      id: "index",
      Link {
        to: Route::Subjects { },
        "Subjects"
      }
    }
  }
}
