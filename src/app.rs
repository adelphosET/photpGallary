use crate::components::navigation::NavigationBar;
use crate::pages::index::Home;
use crate::pages::{
    kalkidan::KalkidanImages, 
    religious_rituals::ReligiousRituals,
    uploader::UploadImageHander,
    uploader_inteface::UploadImageIntrface,
    we_celeb::HomeWeddingPic, 
    we_mesk::WeedingMesk,
};
use dioxus::document::{Script, Stylesheet};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,PartialEq, Serialize, Deserialize)]
pub struct ImageDB {
    pub id: Option<i64>,   // None when inserting, Some(id) when retrieved
    pub content: Vec<u8>,   // BLOB ↔ Vec<u8>
    pub name: String,
    pub category: String,
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
#[component]
pub fn AppStarter() -> Element {
    info!("AppStarter rendered");
    rsx! {
          Stylesheet {
            href: TAILWIND_CSS,
            rel:"stylesheet",
        }

        Router::<URLRoute> {}

    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum URLRoute {
    #[layout(NavigationBar)]
    #[route("/:idid")]
    HomeWeddingPic { idid: String },
    #[route("/")]
    Home {},
    #[route("/mesk")]
    WeedingMesk {},
    #[route("/rituals")]
    ReligiousRituals {},
    #[route("/kalkidan")]
    KalkidanImages {},
    #[route("/up")]
    UploadImageIntrface {},
    #[route("/uploder/:category/")]
    UploadImageHander { category: String },
    // #[route("/allimg/:type/")]
    // ImageGalleryByCategory{ type: String },
}
