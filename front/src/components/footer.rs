use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn FooterBlock<G: Html>(cx: Scope) -> View<G> {
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
    footer(class="footer bg-dark-footer relative text-gray-200 dark:text-gray-200") {
      div(class="py-[30px] px-0 border-t border-slate-800") {
        div(class="container text-center") {
          div(class="grid md:grid-cols-3 grid-cols-1 items-center") {
            div(class="md:text-left text-center") {
              a(class="text-[22px] focus:outline-none", href="#") {
                img(class="mx-auto md:mr-auto md:ml-0", alt="", src="assets/images/logo-wide.png") {
                }
              }
            }

            ul(class="list-none md:text-right text-center mt-6 md:mt-0") {
              li(class="inline") {
                a(href="", target="_blank", class="btn btn-icon btn-sm border border-gray-800 rounded-md hover:border-indigo-600 dark:hover:border-indigo-600 hover:bg-indigo-600 dark:hover:bg-indigo-600") {
                  i(title="Buy Now", class="uil uil-shopping-cart align-middle") {
                  }
                }
              }

              li(class="inline") {
                a(href="", target="_blank", class="btn btn-icon btn-sm border border-gray-800 rounded-md hover:border-indigo-600 dark:hover:border-indigo-600 hover:bg-indigo-600 dark:hover:bg-indigo-600") {
                  i(title="Linkedin", class="uil uil-linkedin") {
                  }
                }
              }
              li(class="inline") {
                a(href="https://www.facebook.com/shreethemes", target="_blank", class="btn btn-icon btn-sm border border-gray-800 rounded-md hover:border-indigo-600 dark:hover:border-indigo-600 hover:bg-indigo-600 dark:hover:bg-indigo-600") {
                  i(class="uil uil-facebook-f align-middle", title="facebook") {
                  }
                }
              }

              li(class="inline") {
                a(href="/", target="_blank", class="btn btn-icon btn-sm border border-gray-800 rounded-md hover:border-indigo-600 dark:hover:border-indigo-600 hover:bg-indigo-600 dark:hover:bg-indigo-600") {
                  i(title="twitter", class="uil uil-twitter align-middle") {
                  }
                }
              }

            }
          }
        }
      }
    }

    }
}
