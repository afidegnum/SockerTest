#[cfg(client)]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(client)]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

// pub mod configs;
// mod capsules;
mod components;
mod error_views;
// pub mod global_state;
// pub mod global_state;
mod httpreq;
mod templates;

use components::footer::FooterBlock;
use components::menu::MenuBlock;
use std::collections::HashMap;

use std::env;
use std::path::Path;
use sycamore::view;
use walkdir::WalkDir;

pub static SOCKET_ADDR: &str = "/tmp/iso-uds.socket";

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    let mut static_paths: HashMap<String, String> = HashMap::new();

    //let pathstr = env!("CARGO_MANIFEST_DIR");
    // TODO: Suggest "static" as the default static directory name
    let pathstr = "/app/";
    let p = Path::new(pathstr);
    let target = p.join("static");
    // let target = Path::new("static");

    let rp = target.file_name().unwrap();

    for x in WalkDir::new(&target)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
    {
        let a = x
            .path()
            .strip_prefix(&target)
            .unwrap()
            .to_str()
            .expect("invalid utf8 in path")
            .to_owned();

        let k = Path::new("/assets").join(a.clone());

        let bf = Path::new(rp).join(a.clone());

        static_paths.insert(k.display().to_string(), bf.display().to_string());
        // log::debug!("Content -> {:#?}", static_paths);
        // web_log!("Content -> {:#?}", static_paths);
    }

    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        //.template(crate::templates::latest::get_template())        
        //.template(crate::templates::libraries::get_template())
        //.capsule_ref(&*crate::capsules::menu::MENU)
        // .error_views(ErrorViews::unlocalized_development_default())
        .error_views(crate::error_views::get_error_views())
        // .panic_handler(|_panic_info| perseus::web_log!("The app has panicked."))
        //.global_state_creator(crate::global_state::get_global_state_creator())
        .static_aliases(static_paths)
        .index_view(|cx| {
            view! { cx,
                html (lang="en", class="light scroll-smooth", dir="ltr") {
                    head {
                        //
                            meta(charset="UTF-8") {
    }
                            title() {
                            "Isodec Integrated Seocial Development Center"
                            }
                            meta(content="IE=edge", http-equiv="X-UA-Compatible") {
                            }
                            link(rel="shortcut icon", href="assets/images/favicon.ico") {
                            }
                            link(href="assets/libs/tobii/css/tobii.min.css", rel="stylesheet") {
                            }
                            link(rel="stylesheet", href="assets/libs/tiny-slider/tiny-slider.css") {
                            }
                            link(href="assets/libs/@iconscout/unicons/css/line.css", type="text/css", rel="stylesheet") {
                            }
                            link(href="assets/css/icons.css", rel="stylesheet") {
                            }
                            link(rel="stylesheet", href="assets/css/tailwind.css") {
                            }
                        // Perseus automatically resolves `/.perseus/static/` URLs to the contents of the `static/` directory at the project root
                        // link(rel = "stylesheet", href = ".perseus/static/style.css")
                    }
                    body {
                        // Quirk: this creates a wrapper `<div>` around the root `<div>` by necessity
                        MenuBlock{}
                        PerseusRoot()
                        FooterBlock()
                        // (MENU.widget(cx, "", ())) // Capsule is the same as Drupal/widget, never use it here. user component instead.
                    }
                }
            }
        })
}
