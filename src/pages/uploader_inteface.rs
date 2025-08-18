use crate::app::URLRoute;

use dioxus::prelude::*;
#[component]
pub fn UploadImageIntrface() -> Element {
    rsx! {

    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
        div { class: "max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm",
            a { href: "#",
                img {
                    alt: "",
                    class: "rounded-t-lg",
                    src: asset!("/assets/data/mesk/CV6A1262.avif"),
                }
            }
            div { class: "p-5",
                Link {
                    to: URLRoute::UploadImageHander { category : "wedding_pic".to_string() },
                    h5 { class: "mb-2 text-2xl font-bold tracking-tight text-gray-900",
                        "Photos At Mesk"
                    }
                }
                p { class: "mb-3 font-normal text-gray-700",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.\r\n            "
                }
                Link {
                    class: "inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300",
                    to: URLRoute::UploadImageHander { category : "mesk_pictures".to_string() },
                    "Upload photo"
                    svg {
                        "aria_hidden": true,
                        class: "rtl:rotate-180 w-3.5 h-3.5 ms-2",
                        fill: "none",
                        view_box: "0 0 14 10",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M1 5h12m0 0L9 1m4 4L9 9",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                        }
                    }
                }
            }
        }

        div { class: "max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm",
            a { href: "#",
                img {
                    alt: "",
                    class: "rounded-t-lg",
                    src: "/docs/images/blog/image-1.jpg",
                }
            }
            div { class: "p-5",
                Link {
                    to: URLRoute::UploadImageHander { category : "kalkidn-sriat".to_string() },
                    h5 { class: "mb-2 text-2xl font-bold tracking-tight text-gray-900",
                        "kalkidan Sirat"
                    }
                }
                p { class: "mb-3 font-normal text-gray-700",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.\r\n            "
                }
                Link {
                    class: "inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300",
                    to: URLRoute::UploadImageHander { category : "religious_ritual".to_string() },
                    "Upload photo"
                    svg {
                        "aria_hidden": true,
                        class: "rtl:rotate-180 w-3.5 h-3.5 ms-2",
                        fill: "none",
                        view_box: "0 0 14 10",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M1 5h12m0 0L9 1m4 4L9 9",
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
    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
        div { class: "max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm",
            a { href: "#",
                img {
                    alt: "",
                    class: "rounded-t-lg",
                    src: asset!("/assets/static/family.JPG"),
                }
            }
            div { class: "p-5",
                Link {
                    to: URLRoute::UploadImageHander { category : "famimly".to_string() },
                    h5 { class: "mb-2 text-2xl font-bold tracking-tight text-gray-900",
                        "Family"
                    }
                }
                p { class: "mb-3 font-normal text-gray-700",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.\r\n            "
                }
                Link {
                    class: "inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300",
                    to: URLRoute::UploadImageHander { category : "famimly".to_string() },

                    "Family"
                    svg {
                        "aria_hidden": true,
                        class: "rtl:rotate-180 w-3.5 h-3.5 ms-2",
                        fill: "none",
                        view_box: "0 0 14 10",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M1 5h12m0 0L9 1m4 4L9 9",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                        }
                    }
                }
            }
        }
        div { class: "max-w-sm bg-white border border-gray-200 rounded-lg shadow-sm",
            a { href: "#",
                img {
                    alt: "",
                    class: "rounded-t-lg",
                    src: "/docs/images/blog/image-1.jpg",
                }
            }
            div { class: "p-5",
                a { href: "#",
                    h5 { class: "mb-2 text-2xl font-bold tracking-tight text-gray-900",
                        "Noteworthy technology acquisitions 2021"
                    }
                }
                p { class: "mb-3 font-normal text-gray-700",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order.\r\n            "
                }
                a {
                    class: "inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300",
                    href: "#",
                    "Read more"
                    svg {
                        "aria_hidden": true,
                        class: "rtl:rotate-180 w-3.5 h-3.5 ms-2",
                        fill: "none",
                        view_box: "0 0 14 10",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M1 5h12m0 0L9 1m4 4L9 9",
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
