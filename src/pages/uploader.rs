use crate::{app::ImageDB, backend::server_backend::insert_image};
use dioxus::prelude::dioxus_elements::FileEngine;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use std::sync::Arc;

#[component]
pub fn UploadImageHander(category: String) -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut files_uploaded = use_signal(|| Vec::<ImageDB>::new());
    let mut files_selected = use_signal(|| Vec::<(String, Vec<u8>)>::new());

    let read_files = move |file_engine: Arc<dyn FileEngine>| {
        let mut files_selected = files_selected.clone();
        async move {
            let files = file_engine.files();
            let mut buffer = Vec::new();
            for file_name in &files {
                if let Some(bytes) = file_engine.read_file(file_name).await {
                    buffer.push((file_name.clone(), bytes));
                }
            }
            files_selected.set(buffer);
        }
    };

    let submit_files = move |_| {
        let mut files_uploaded = files_uploaded.clone();
        let mut files_selected = files_selected.clone();
        let category = category.clone();

        spawn(async move {
            let selected_files: Vec<(String, Vec<u8>)> = files_selected.read().clone();

            for (file_name, bytes) in selected_files {
                match insert_image(file_name.clone(), category.clone(), bytes.clone()).await {
                    Ok(inserted_id) => {
                        files_uploaded.write().push(ImageDB {
                            id: Some(inserted_id),
                            content: bytes.clone(),
                            name: file_name.clone(),
                            category: category.clone(),
                        });
                    }
                    Err(err) => {
                        error!("Failed to insert image {}: {:?}", file_name, err);
                    }
                }
            }

            files_selected.write().clear();
        });
    };

    let upload_files = move |evt: FormEvent| {
        if let Some(file_engine) = evt.files() {
            spawn(read_files(file_engine));
        }
    };

    rsx! {
        div { class: "p-6 max-w-6xl mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Image Upload" }

            button {
                class: "mb-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                onclick: move |_| files_uploaded.write().clear(),
                "Clear All Images"
            }

            div { class: "flex items-center mb-4 gap-2",
                input {
                    r#type: "checkbox",
                    id: "directory-upload",
                    checked: enable_directory_upload,
                    oninput: move |evt| enable_directory_upload.set(evt.checked()),
                }
                label { r#for: "directory-upload", class: "text-gray-700", "Enable directory upload" }
            }

            label {
                r#for: "imagereader",
                class: "flex flex-col items-center justify-center w-full h-32 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer bg-gray-50 hover:bg-gray-100",
                "Click or drag files here to upload"
            }
            input {
                r#type: "file",
                id: "imagereader",
                accept: "image/png,image/jpeg,image/webp,image/avif",
                multiple: true,
                name: "imagereader",
                directory: enable_directory_upload,
                onchange: upload_files,
                class: "hidden"
            }

            if !files_selected.read().is_empty() {
                h3 { class: "mt-6 text-lg font-semibold text-gray-800", "Selected files (ready to upload):" }
                ul { class: "list-disc pl-5 text-gray-600",
                    for (name, _) in files_selected.read().iter() {
                        li { "{name}" }
                    }
                }
                button {
                    class: "mt-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                    onclick: submit_files,
                    "Submit Selected Photos"
                }
                br {}
                br {}
                br {}
                br {}
            }

            if !files_uploaded.read().is_empty() {
                h3 { class: "mt-6 text-lg font-semibold text-gray-800", "Uploaded images:" }
                ul { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 mt-4",
                    for image in files_uploaded.read().iter().rev() {
                        li { class: "bg-white rounded-lg shadow p-3 text-center",
                            span { class: "block text-sm text-gray-700 mb-2",
                                "{image.name} ({image.category})"
                            }
                            img {
                                class: "rounded-lg max-h-48 mx-auto",
                                src: format!("data:image/png;base64,{}", base64::encode(&image.content)),
                            }
                        }
                    }
                }
            }
        }
    }
}
