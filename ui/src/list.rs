use dioxus::prelude::*;

const LIST_CSS: Asset = asset!("/assets/styling/list.css");

#[component]
pub fn List() -> Element {
    let mut total_saves = use_resource(move || async move {
        match api::list_a_save().await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Error fetching saves: {e:?}");
                None
            }
        }
    });

    rsx! {
        link { rel: "stylesheet", href: LIST_CSS }

        div { class: "list-container",
            match total_saves.read().as_ref() {
                Some(Some(saves)) => rsx! {
                    h2 { class: "list-title", "Total saves: {saves.len()}" }

                    div { class: "cards",
                        for save in saves {
                            div { class: "card",
                                h3 { class: "card-name", "{save.name}" }
                                a {
                                    class: "card-link",
                                    href: "{save.link}",
                                    target: "_blank",
                                    "{save.link}"
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    div { class: "card",
                        p { class: "loading", "Loading saves or an error occurred…" }
                    }
                },
            }
        }
    }
}
