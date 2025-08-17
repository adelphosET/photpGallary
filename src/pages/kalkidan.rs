use dioxus::prelude::*;


#[derive(Clone, PartialEq)]
struct ImageItem {
    thumb: Asset,
    full: Asset,
    alt: &'static str,
}
const IMAGES: &[ImageItem] = &[

	ImageItem {
	    thumb: asset!("/assets/data/kalkidan/6S4A0587.webp"),
	    full: asset!("/assets/data/kalkidan/6S4A0587.webp"),
	    alt: "6S4A0587",
	},
	ImageItem {
	    thumb: asset!("/assets/data/kalkidan/6S4A0604.webp"),
	    full: asset!("/assets/data/kalkidan/6S4A0604.webp"),
	    alt: "6S4A0604",
	},
	ImageItem {
	    thumb: asset!("/assets/data/kalkidan/6S4A0612.webp"),
	    full: asset!("/assets/data/kalkidan/6S4A0612.webp"),
	    alt: "6S4A0612",
	},
];

#[component]
pub fn KalkidanImages() -> Element {
    // Holds the index of the selected image for the modal
    let mut selected = use_signal(|| None::<usize>);

    rsx! {
        div { class: "max-w-6xl mx-auto p-4 scroll-smooth",
            FeaturedImage {
                img: IMAGES[0].clone(),
                id: 0,
                on_select: move |id| *selected.write() = Some(id),
            }
            ThumbnailRail {
                images: IMAGES,
                on_select: move |id| *selected.write() = Some(id),
            }

            if let Some(idx) = selected() {
                LightboxModal {
                    img: IMAGES[idx].clone(),
                    total: IMAGES.len(),
                    idx,
                    on_close: move |_| *selected.write() = None,
                    on_next: move |_| *selected.write() = Some((idx + 1) % IMAGES.len()),
                    on_prev: move |_| *selected.write() = Some((idx + IMAGES.len() - 1) % IMAGES.len()),
                }
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

#[component]
fn ThumbnailRail(images: &'static [ImageItem], on_select: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "relative",
            div { class: "flex items-center justify-between mb-2",
                div { class: "text-sm text-gray-500", "Click an image to view" }
            }
            // Grid instead of horizontal scroll for better UX
            div { class: "grid grid-cols-5 gap-4",
                for (i, img) in images.iter().enumerate() {
                    a {
                        onclick: move |_| on_select.call(i),
                        class: "cursor-pointer",
                        img {
                            src: img.thumb,
                            alt: img.alt,
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
    on_prev: EventHandler<()>
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
