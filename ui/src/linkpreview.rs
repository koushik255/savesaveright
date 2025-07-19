use api::OgData;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LinkPreviewProps {
    pub url: String,
}

#[component]
pub fn LinkPreview(props: LinkPreviewProps) -> Element {
    let mut og = use_signal(|| None::<OgData>);
    let url = props.url.clone();

    use_effect(move || {
        spawn(async move {
            let data = fetch_og(url).await;
            og.set(Some(data.unwrap_or_default()));
        });
    });

    match og() {
        Some(data) => rsx! {
            div {
                class: "link-preview",
                if let Some(img) = &data.image {
                    img { src: "{img}", style: "max-width: 100%;" }
                }
                if let Some(title) = &data.title {
                    h3 { "{title}" }
                }
                if let Some(desc) = &data.description {
                    p { "{desc}" }
                }
            }
        },
        None => rsx! { "Loading preview…" },
    }
}

#[server]
async fn fetch_og(url: String) -> Result<OgData, String> {
    api::fetch_og(&url).await
}
