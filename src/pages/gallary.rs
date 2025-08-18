use crate::backend::server_backend::get_by_category;
use dioxus::prelude::*;
use std::rc::Rc;
#[derive(Clone, PartialEq)]
struct ImageItem {
    thumb: String,
    full: String,
    alt: String,
}
const PAGE_SIZE: i64 = 10;

#[component]
pub fn GallaryEexplorer(category: String) -> Element {
    let mut selected = use_signal(|| None::<usize>);
    let mut page = use_signal(|| 0);

    let images_res = use_server_future({
        let category = category.clone();
        let page = page.clone();
        move || {
            let category = category.clone();
            let page = *page.read(); // read the value inside the closure

            async move { get_by_category(category, page * PAGE_SIZE, PAGE_SIZE).await }
        }
    })?;

    rsx! {
        div {
            match (images_res.value())() {
                Some(Ok(images)) if !images.is_empty() => {
                    let img_assets: Vec<ImageItem> = images.iter().map(|img| {
                        let b64 = base64::encode(&img.content);
                        let src = format!("data:image/avif;base64,{b64}");
                        ImageItem { thumb: src.clone(), full: src.clone(), alt: img.name.clone() }
                    }).collect();
                    let img_assets = Rc::new(img_assets);
                    rsx! {
                        FeaturedImage {
                            img: img_assets[0].clone(),
                            id: 0,
                            on_select: {
                                let mut selected = selected.clone();
                                move |id| *selected.write() = Some(id)
                            }
                        }
                        ThumbnailRail {
                            images: (&img_assets).to_vec(),
                            on_select: {
                                let mut selected = selected.clone();
                                move |id| *selected.write() = Some(id)
                            }
                        }
                        div { class: "flex justify-between mt-4",
                            button {
                                class: "text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                                disabled: *page.read() == 0,
                                onclick: move |_| if *page.read() > 0 { *page.write() -= 1; },
                                "Previous"
                            }
                            button {
                                class: "text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
                                // Disable if less than PAGE_SIZE images (likely last page)
                                disabled: img_assets.len() < PAGE_SIZE as usize,
                                onclick: move |_| if img_assets.len() as i64 == PAGE_SIZE { *page.write() += 1; },
                                "Next"
                            }

                        }
                        br {}
                        br {}
                        br {}
                        if let Some(idx) = selected() {
                            LightboxModal {
                                img: img_assets[idx].clone(),
                                total: img_assets.len(),
                                idx,
                                on_close: {
                                    let mut selected = selected.clone();
                                    move |_| *selected.write() = None
                                },
                                on_next: {
                                    let mut selected = selected.clone();
                                    let mut img_assets = img_assets.clone();
                                    move |_| *selected.write() = Some((idx + 1) % img_assets.len())
                                },
                                on_prev: {
                                    let mut selected = selected.clone();
                                    let mut img_assets = img_assets.clone();
                                    move |_| *selected.write() = Some((idx + img_assets.len() - 1) % img_assets.len())
                                }
                            }
                        }
                    }
                }
                Some(Ok(_)) => rsx!(p { "No images found." }),
                Some(Err(e)) => rsx!(p { "Error: {e}" }),
                _ => rsx!(p { "Loading..." }),
            }
        }
    }
}
#[component]
fn FeaturedImage(img: ImageItem, id: usize, on_select: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "mb-4",
            a {
                onclick: move |_| on_select.call(id),
                class: "group block cursor-pointer",
                img {
                    src: img.full,
                    alt: img.alt,
                    loading: "lazy",
                    class: "w-full h-auto rounded-lg shadow-md transition-transform transform group-hover:scale-105"
                }
            }
        }
    }
}
#[derive(Props, PartialEq, Clone)]
struct ThumbnailRailProps {
    images: Vec<ImageItem>,
    on_select: EventHandler<usize>,
}
#[component]
fn ThumbnailRail(props: ThumbnailRailProps) -> Element {
    rsx! {
        div { class: "relative",
            div { class: "flex items-center justify-between mb-2",
                div { class: "text-sm text-gray-500", "Click an image to view" }
            }
            // Grid instead of horizontal scroll for better UX
            div { class: "grid grid-cols-5 gap-4",
                for (i, img) in props.images.iter().enumerate() {
                    a {
                        onclick: move |_| props.on_select.call(i),
                        class: "cursor-pointer",
                        img {
                            src: img.thumb.clone(),
                            alt: img.alt.clone(),
                            loading: "lazy",
                            class: "w-full h-32 object-cover rounded-lg shadow-sm"
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn LightboxModal(
    img: ImageItem,
    idx: usize,
    total: usize,
    on_close: EventHandler<()>,
    on_next: EventHandler<()>,
    on_prev: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/70 flex items-center justify-center p-4",
            div { class: "max-w-[95vw] max-h-[90vh] w-full",
                img {
                    src: img.full,
                    alt: img.alt,
                    loading: "lazy",
                    class: "w-full h-auto rounded-lg mx-auto max-h-[90vh] object-contain"
                }
                 nav {
                    aria_label: "Page navigation example",
                    class: "fixed bottom-0 mb-[5px] left-0 w-full",
                    div { class: "flex items-center justify-between h-10 text-base px-4",
                        button {
                            class: "flex items-center justify-center px-4 h-10 leading-tight text-gray-500 bg-white border border-gray-300 rounded-s-lg hover:bg-gray-100 hover:text-gray-700",
                            onclick: move |_| on_prev.call(()),
                            span { class: "sr-only", "Previous" }
                            svg {
                                "aria_hidden": true,
                                class: "w-3 h-3 rtl:rotate-180",
                                fill: "none",
                                view_box: "0 0 6 10",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M5 1 1 5l4 4",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                }
                            }
                        }
                        button {
                            class: "px-3 py-1 rounded bg-white/90 hover:bg-white text-gray-700 font-medium shadow border border-gray-300",
                            onclick: move |_| on_close.call(()),
                            "✕"
                        }
                        button {
                            class: "flex items-center justify-center px-4 h-10 leading-tight text-gray-500 bg-white border border-gray-300 rounded-e-lg hover:bg-gray-100 hover:text-gray-700",
                            onclick: move |_| on_next.call(()),
                            span { class: "sr-only", "Next" }
                            svg {
                                "aria_hidden": true,
                                class: "w-3 h-3",
                                fill: "none",
                                view_box: "0 0 6 10",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "m1 9 4-4-4-4",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
