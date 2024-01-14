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
        div(d-comp="newsletters", class="relative bg-white dark:bg-slate-900 lg:px-8 px-6 py-10 rounded-xl shadow dark:shadow-gray-800 overflow-hidden") {
          div(class="grid md:grid-cols-2 grid-cols-1 items-center gap-[30px]") {
            div(class="md:text-left text-center z-1") {
              h3(class="md:text-3xl text-2xl md:leading-normal leading-normal font-semibold") {
                "Subscribe to Newsletter!"
              }
              p(class="text-slate-400 max-w-xl mx-auto mt-2") {
                "Subscribe to get latest updates and information."
              }
            }
            div(class="subcribe-form z-1") {
              form(class="relative max-w-xl") {
                input(name="email", type="email", id="subcribe", class="pt-4 pr-40 pb-4 pl-6 w-full h-[50px] outline-none text-black dark:text-white rounded-full bg-white dark:bg-slate-900 shadow dark:shadow-gray-800", placeholder="Your Email Address :") {
                }
                button(class="btn absolute top-[2px] right-[3px] h-[46px] bg-indigo-600 hover:bg-indigo-700 border-indigo-600 hover:border-indigo-700 text-white rounded-full", type="submit") {
                  "Subscribe"
                }
              }
            }
          }
          div(class="absolute -top-5 -left-5") {
            div(class="uil uil-envelope lg:text-[150px] text-7xl text-slate-900/5 dark:text-white/5 -rotate-45") {
            }
          }
          div(class="absolute -bottom-5 -right-5") {
            div(class="uil uil-pen lg:text-[150px] text-7xl text-slate-900/5 dark:text-white/5") {
            }
          }
        }

    }
}
