use crate::components::navigation::NavigationBar;
use crate::pages::{
    gallary::GallaryEexplorer, index::Home, uploader::UploadImageHander,
    uploader_inteface::UploadImageIntrface,
};
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageDB {
    pub id: Option<i64>, // None when inserting, Some(id) when retrieved
    pub content: Vec<u8>,
    pub name: String,
    pub category: String,
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
#[component]
pub fn AppStarter() -> Element {

    info!("AppStarter rendered");
    rsx! {
        Stylesheet 
		{ href: TAILWIND_CSS, 
		  rel: "stylesheet" }

        Router::<URLRoute> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum URLRoute {
    #[layout(NavigationBar)]
    #[route("/")]
    Home {},
    #[route("/show/:category")]
    GallaryEexplorer { category: String },
    #[route("/up")]
    UploadImageIntrface {},
    #[route("/saver/:category/")]
    UploadImageHander { category: String },
}
#[component]
fn SplashScreen() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-blue-600 text-white",
            div {
                class: "animate-pulse text-3xl font-bold",
                " 📸 Photo Gallery"
            }
        }
    }
}
