use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn MenuBlock<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
    ////////////////
        div(class="grid grid-cols-1 lg:grid-cols-3 md:grid-cols-2 gap-[30px] sm:-mt-[220px] -mt-[200px] m-0", d-comp="grid") {
          div(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
            img(src="assets/images/blog/08.jpg", alt="") {
            }
            div(class="content p-6") {
              a(class="title h5 text-lg font-medium hover:text-indigo-600 duration-500 ease-in-out", href="blog-detail.html") {
                "On the other hand we provide denounce"
              }
              p(class="text-slate-400 mt-3") {
                "The phrasal sequence of the is now so that many campaign and benefit"
              }
              div(class="mt-4") {
                a(class="btn btn-link font-normal hover:text-indigo-600 after:bg-indigo-600 duration-500 ease-in-out", href="blog-detail.html") {
                  "Read More"
                  i(class="uil uil-arrow-right") {
                  }
                }
              }
            }
          }
        }
     //////////

        }
}
