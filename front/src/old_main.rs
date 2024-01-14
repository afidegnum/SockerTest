// #[cfg(client)]
// use lol_alloc::{FreeListAllocator, LockedAllocator};

// #[cfg(client)]
// #[global_allocator]
// static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

mod capsules;
mod error_views;
mod templates;
use perseus::prelude::*;

#[perseus::main(perseus_actix_web::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .template(crate::templates::test::get_template())
        .capsule_ref(&*crate::capsules::menu::MENU)
        .error_views(crate::error_views::get_error_views())
}
