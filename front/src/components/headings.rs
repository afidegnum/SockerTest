use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn MenuBlock<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
    /////////////////////////

        div(d-comp="headings", class="container-fluid") {
          div(class="profile-banner relative text-transparent") {
            input(id="pro-banner", name="profile-banner", type="file", class="hidden", onchange="loadFile(event)") {
            }
            div(class="relative shrink-0") {
              img(id="profile-banner", alt="", src="assets/images/blog/bg.jpg", class="h-96 w-full object-cover") {
              }
              div(class="absolute inset-0 bg-black/70") {
              }
              label(for="pro-banner", class="absolute inset-0 cursor-pointer") {
              }
            }
          }
        }

    /////////////////////////
            }
}
