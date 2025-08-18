use crate::app::URLRoute;
use dioxus::prelude::*;

#[component]
pub fn NavigationBar() -> Element {
    rsx! {
     div { class: "fixed z-50 w-full h-16 max-w-lg -translate-x-1/2 bg-white border border-gray-200 rounded-full bottom-4 left-1/2 dark:bg-gray-700 dark:border-gray-600",
         div { class: "grid h-full max-w-lg grid-cols-5 mx-auto",
             Link {
                 class: "inline-flex flex-col items-center justify-center px-5 rounded-s-full hover:bg-gray-50 dark:hover:bg-gray-800 group",
                 "data-tooltip-target": "tooltip-family",
                 to:  URLRoute::GallaryEexplorer { category: "family_picture".to_string() },
                 svg {
                     class: "w-5 h-5 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500",
                     fill: "none",
                     stroke: "currentColor",
                     view_box: "0 0 24 24",
                     xmlns: "http://www.w3.org/2000/svg",
                     path {
                         d: "M5.121 17.804A6 6 0 1112 14a6 6 0 016.879 3.804M12 14v6m-6-6v6m12-6v6",
                         stroke_linecap: "round",
                         stroke_linejoin: "round",
                         stroke_width: "2",
                     }
                 }
                 span { class: "sr-only", "Family" }
             }
             div {
                 class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-xs opacity-0 tooltip dark:bg-gray-700",
                 id: "tooltip-family",
                 role: "tooltip",
                 "Family"
                 div { class: "tooltip-arrow", "data-popper-arrow": "false" }
             }
              Link {
                 class: "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-50 dark:hover:bg-gray-800 group",
                 "data-tooltip-target": "tooltip-wedding",
                 to: URLRoute::GallaryEexplorer { category: "wedding_pic".to_string() },
                 svg {
                     class: "w-5 h-5 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-pink-600 dark:group-hover:text-pink-500",
                     fill: "currentColor",
                     view_box: "0 0 24 24",
                     xmlns: "http://www.w3.org/2000/svg",
                     path { d: "M12 21s-6.716-4.43-9.657-9.657A5.657 5.657 0 0112 3a5.657 5.657 0 019.657 8.343C18.716 16.57 12 21 12 21z" }
                 }
                 span { class: "sr-only", "Wedding" }
             }
             div {
                 class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-xs opacity-0 tooltip dark:bg-gray-700",
                 id: "tooltip-wedding",
                 role: "tooltip",
                 "Wedding"
                 div { class: "tooltip-arrow", "data-popper-arrow": "false" }
             }
             div { class: "flex items-center justify-center",
                   Link {
                     class: "inline-flex items-center justify-center w-10 h-10 font-medium bg-blue-600 rounded-full hover:bg-blue-700 group focus:ring-4 focus:ring-blue-300 focus:outline-none dark:focus:ring-blue-800",
                     "data-tooltip-target": "tooltip-new",
                     to: URLRoute::UploadImageIntrface {},
                     svg {
                         class: "w-4 h-4 text-white",
                         fill: "none",
                         stroke: "currentColor",
                         view_box: "0 0 18 18",
                         xmlns: "http://www.w3.org/2000/svg",
                         path {
                             d: "M9 1v16M1 9h16",
                             stroke_linecap: "round",
                             stroke_linejoin: "round",
                             stroke_width: "2",
                         }
                     }
                     span { class: "sr-only", "New item" }
                 }
             }
             div {
                 class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-xs opacity-0 tooltip dark:bg-gray-700",
                 id: "tooltip-new",
                 role: "tooltip",
                 "Create new item"
                 div { class: "tooltip-arrow", "data-popper-arrow": "false" }
             }
             Link {
                 class: "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-50 dark:hover:bg-gray-800 group",
                 "data-tooltip-target": "tooltip-nature",
                 to: URLRoute::GallaryEexplorer { category: "mesk_pictures".to_string() },
                 svg {
                     class: "w-5 h-5 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-green-600 dark:group-hover:text-green-500",
                     fill: "none",
                     stroke: "currentColor",
                     view_box: "0 0 24 24",
                     xmlns: "http://www.w3.org/2000/svg",
                     path {
                         d: "M12 2a7 7 0 017 7c0 3.866-3.134 7-7 7m0-14a7 7 0 00-7 7c0 3.866 3.134 7 7 7m0 0v6",
                         stroke_linecap: "round",
                         stroke_linejoin: "round",
                         stroke_width: "2",
                     }
                 }
                 span { class: "sr-only", "Nature" }
             }
             div {
                 class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-xs opacity-0 tooltip dark:bg-gray-700",
                 id: "tooltip-nature",
                 role: "tooltip",
                 "Nature"
                 div { class: "tooltip-arrow", "data-popper-arrow": "false" }
             }

             Link {
                 class: "inline-flex flex-col items-center justify-center px-5 rounded-e-full hover:bg-gray-50 dark:hover:bg-gray-800 group",
                 "data-tooltip-target": "tooltip-profile",
                 to: URLRoute::GallaryEexplorer { category:"religious_ritual".to_string() } ,
                 svg {
                     class: "w-5 h-5 mb-1 text-gray-500 dark:text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-500",
                     fill: "currentColor",
                     view_box: "0 0 20 20",
                     xmlns: "http://www.w3.org/2000/svg",
                     path { d: "M9 2h2v6h6v2h-6v8H9v-8H3V8h6V2z" }
                 }
                 span { class: "sr-only", "Profile" }
             }
             div {
                 class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-xs opacity-0 tooltip dark:bg-gray-700",
                 id: "tooltip-profile",
                 role: "tooltip",
                 "Relogious Ritual"
                 div { class: "tooltip-arrow", "data-popper-arrow": "false" }
             }

         }
     }
        Outlet::<URLRoute> {}
    }
}
