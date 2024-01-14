use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn MenuBlock<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // div(id = "links", style = "margin-top: 1rem;") {
        //     a(id = "index-link", href = "") { "Index" }
        //     br {}
        //     a(id = "about-link", href = "about") { "About" }
        //     br {}
        //     a(id = "test-link", href = "test") { "test" }
        // }
        //
        //
    div(class="fixed top-1/4 -right-2 z-50") {
      span(class="relative inline-block rotate-90") {
        input(class="checkbox opacity-0 absolute", type="checkbox", id="chk") {
        }
        label(class="label bg-slate-900 dark:bg-white shadow dark:shadow-gray-800 cursor-pointer rounded-full flex justify-between items-center p-1 w-14 h-8", for="chk") {
          i(class="uil uil-moon text-[20px] text-yellow-500") {
          }
          i(class="uil uil-sun text-[20px] text-yellow-500") {
          }
          span(class="ball bg-white dark:bg-slate-900 rounded-full absolute top-[2px] left-[2px] w-7 h-7") {
          }
        }
      }
    }

    }
}
