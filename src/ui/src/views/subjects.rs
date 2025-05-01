use crate::Route;
use aide_ed_api::fns::subject::list_subjects;
use dioxus::prelude::*;
use dioxus_router::components::Link;
use uuid::Uuid;

#[component]
pub fn Subjects() -> Element {
  let feedback = use_signal(String::new);

  let subjects = use_resource(move || {
    let mut feedback = feedback.clone();
    async move {
      match list_subjects().await {
        Ok(subjects) => subjects,
        Err(e) => {
          feedback.set(format!("Error fetching subjects: {}", e));
          vec![]
        }
      }
    }
  });

  rsx! {
      div { id: "subjects", class: "p-4 space-y-4",
          h1 { class: "text-xl font-bold", "Create New Subject" }

          for subject in subjects.cloned().unwrap_or_default() {
              div {
                  Link {
                      to: Route::Subject { id: subject.id.to_string() },
                      h5 { "{subject.title}" }
                  }
                  p { "{subject.description}" }
              }
          }

          Link {
              to: Route::Subject { id: "new".to_owned() },
              h3 { "Create subject" }
          }

          if !feedback().is_empty() {
              p { class: "text-green-600", "{feedback}" }
          }
      }
  }
}
