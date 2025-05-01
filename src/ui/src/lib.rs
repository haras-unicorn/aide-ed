#![deny(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::dbg_macro, clippy::print_stdout, clippy::print_stderr)]
#![deny(clippy::todo)]
#![deny(clippy::unreachable)]
#![deny(clippy::allow_attributes_without_reason)]

use aide_ed_api::fns::subject::{create_subject, CreateSubjectArgs};
use dioxus::prelude::*;

pub fn app() -> Element {
  let title = use_signal(String::new);
  let description = use_signal(String::new);
  let feedback = use_signal(String::new);

  rsx! {
      div { class: "p-4 space-y-4",
          h1 { class: "text-xl font-bold", "Create New Subject" }

          if !feedback().is_empty() {
              p { class: "text-green-600", "{feedback}" }
          }

          form {
              onsubmit: move |_| {
                  // Clone signals to move into the async block
                  let mut title_signal = title.clone();
                  let mut desc_signal = description.clone();
                  let mut feedback_signal = feedback.clone();

                  spawn(async move {
                      let args = CreateSubjectArgs {
                          title: title_signal.read().clone(),
                          description: desc_signal.read().clone(),
                      };

                      match create_subject(args).await {
                          Ok(created_subject) => {
                              title_signal.set(String::new());
                              desc_signal.set(String::new());
                              feedback_signal.set(format!("Subject '{}' created!", created_subject.title));
                          }
                          Err(e) => {
                              feedback_signal.set(format!("Error creating subject: {}", e));
                          }
                      }
                  });
              },

              div { class: "flex flex-col space-y-2",
                  label { r#for: "title", "Title:" }
                  input {
                      id: "title",
                      class: "border rounded p-1",
                      value: title(),
                      onchange: move |event| {
                        let mut title_signal = title.clone();
                        title_signal.set(event.value());
                      },
                      required: true,
                  }

                  label { r#for: "description", "Description:" }
                  textarea {
                      id: "description",
                      class: "border rounded p-1",
                      rows: 3,
                      value: description(),
                      onchange: move |event| {
                        let mut desc_signal = description.clone();
                        desc_signal.set(event.value());
                      },
                      required: true,
                  }
              }

              button {
                  r#type: "submit",
                  class: "mt-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                  "Create Subject"
              }
          }
      }
  }
}
