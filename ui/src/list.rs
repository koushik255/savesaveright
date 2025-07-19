use dioxus::prelude::*;

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
        match total_saves.read().as_ref() {
            Some(Some(saves)) => rsx! {
                p { "Total saves: {saves.len()}" }
                ul {
                    for save in saves {
                        li { "{save.name}" }
                        li {
                            a {
                                href: "{save.link}",
                                target: "_blank",
                                "{save.link}"
                            }
                        }
                    }
                }
            },
            _ => rsx! {
                p { "Loading saves or an error occurred…" }
            },
        }
    }
}
