use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    info!("photo Gallery App Lunch...");
    dioxus::launch(photo_gallary::AppStarter);
}
