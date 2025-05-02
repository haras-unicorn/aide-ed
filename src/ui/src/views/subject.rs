use std::str::FromStr;

use aide_ed_api::fns::subject::create_subject;
use aide_ed_api::fns::subject::delete_subject;
use aide_ed_api::fns::subject::get_subject;
use aide_ed_api::fns::subject::update_subject;
use aide_ed_api::fns::subject::CreateSubjectArgs;
use aide_ed_api::fns::subject::UpdateSubjectArgs;
use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Subject(id: String) -> Element {
  let title = use_signal(String::new);
  let description = use_signal(String::new);
  let feedback = use_signal(String::new);
  let edit = use_signal(|| false);

  let nav = navigator();

  let resource_id = id.clone();
  let resource = use_resource(move || {
    let mut feedback = feedback.clone();
    let mut title = title.clone();
    let mut description = description.clone();
    let resource_id = resource_id.clone();
    async move {
      let id = match uuid::Uuid::from_str(&resource_id) {
        Ok(id) => id,
        Err(e) => {
          feedback.set(e.to_string());
          return None;
        }
      };
      match get_subject(id).await {
        Ok(subject) => {
          title.set(subject.title.clone());
          description.set(subject.description.clone());
          Some(subject)
        }
        Err(e) => {
          feedback.set(e.to_string());
          None
        }
      }
    }
  });

  rsx! {
      div { id: "subject", class: "p-4 space-y-4",
          h1 { class: "text-xl font-bold", "Subject {id}" }

          if !feedback().is_empty() {
              p { class: "text-green-600", "{feedback}" }
          }

          if let Some(subject) = resource.cloned().unwrap_or_default() {
              button {
                  onclick: move |_| {
                      let mut edit = edit.clone();
                      edit.set(!edit());
                  },
                  "Edit"
              }

              button {
                  onclick: move |_| {
                      let subject = subject.clone();
                      let mut feedback = feedback.clone();
                      async move {
                          match delete_subject(subject.id).await {
                            Ok(_) => {
                                if let Some(e) = nav.push(Route::Subjects {  }) {
                                  feedback.set(format!("Error in navigation: {:?}", e));
                                }
                            },
                            Err(e) => {
                              feedback.set(format!("Error deleting subject: {}", e));
                            },
                          }
                      }
                  },
                  "Delete"
              }

              if !edit() {
                  div {
                      h5 { "{subject.title}" }
                      p { "{subject.description}" }
                  }
              } else {
                  form {
                      onsubmit: move |_| {
                          let mut title = title.clone();
                          let mut desc = description.clone();
                          let mut feedback = feedback.clone();
                          let mut resource = resource.clone();

                          spawn(async move {
                              let args = UpdateSubjectArgs {
                                  title: title.read().clone(),
                                  description: desc.read().clone(),
                              };

                              match update_subject(subject.id, args).await {
                                  Ok(updated_subject) => {
                                      title.set(String::new());
                                      desc.set(String::new());
                                      resource.restart();
                                      feedback.set(format!("Subject '{}' updated!", updated_subject.title));
                                  }
                                  Err(e) => {
                                      feedback.set(format!("Error updating subject: {}", e));
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
                                let mut title = title.clone();
                                title.set(event.value());
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
                                let mut desc = description.clone();
                                desc.set(event.value());
                              },
                              required: true,
                          }
                      }

                      button {
                          r#type: "submit",
                          class: "mt-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                          "Update Subject"
                      }
                  }
              }
          } else {
              form {
                  onsubmit: move |_| {
                      let mut title = title.clone();
                      let mut desc = description.clone();
                      let mut feedback = feedback.clone();

                      spawn(async move {
                          let args = CreateSubjectArgs {
                              title: title.read().clone(),
                              description: desc.read().clone(),
                          };

                          match create_subject(args).await {
                              Ok(created_subject) => {
                                  title.set(String::new());
                                  desc.set(String::new());
                                  feedback.set(format!("Subject '{}' created!", created_subject.title));
                              }
                              Err(e) => {
                                  feedback.set(format!("Error creating subject: {}", e));
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
                            let mut title = title.clone();
                            title.set(event.value());
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
                            let mut desc = description.clone();
                            desc.set(event.value());
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
}
