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

    div(class="container mt-16", d-comp="featured") {
      div(class="grid md:grid-cols-12 grid-cols-1 pb-8 items-end") {
        div(class="lg:col-span-8 md:col-span-6 md:text-left text-center") {
          h3(class="mb-4 md:text-3xl md:leading-normal text-2xl leading-normal font-semibold") {
            "Our Featured News Post"
          }
        }
      }
      div(class="grid md:grid-cols-12 grid-cols-1 mt-8 gap-[30px]") {
        div(class="lg:col-span-8 md:col-span-6") {
          div(class="grid grid-cols-1 gap-[30px]") {
            div(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
              div(class="lg:flex relative") {
                div(class="relative md:shrink-0") {
                  img(class="h-full w-full object-cover lg:w-52 lg:h-56", src="assets/images/blog/08.jpg", alt="") {
                  }
                }
                div(class="p-6 flex flex-col lg:h-56 justify-center") {
                  a(href="blog-detail.html", class="title h5 text-lg font-medium hover:text-sky-600 duration-500 ease-in-out") {
                    "This mountain will make you reborn"
                  }
                  div(class="my-auto") {
                    p(class="text-slate-400 mt-3") {
                      "The phrasal sequence of the is now so that many campaign and benefit"
                    }
                  }
                  div(class="mt-4") {
                    a(href="blog-detail.html", class="btn btn-link font-normal hover:text-sky-600 after:bg-indigo-600 duration-500 ease-in-out") {
                      "Read More"
                      i(class="uil uil-arrow-right") {
                      }
                    }
                  }
                }
              }
            }
          }
        }
        div(class="lg:col-span-4 md:col-span-6") {
          div(class="sticky top-20") {
            h5(class="text-lg font-semibold bg-gray-50 dark:bg-slate-800 shadow dark:shadow-gray-800 rounded-md p-2 text-center") {
              "Recent post"
            }
            div(class="flex items-center mt-4") {
              img(src="assets/images/blog/08.jpg", class="h-16 rounded-md shadow dark:shadow-gray-800", alt="") {
              }
              div(class="ml-3") {
                a(href="", class="font-semibold hover:text-sky-600") {
                  "Look On The Glorious Balance"
                }
                p(class="text-sm text-slate-400") {
                  "1st May 2022"
                }
              }
            }
            h5(class="text-lg font-semibold bg-gray-50 dark:bg-slate-800 shadow dark:shadow-gray-800 rounded-md p-2 text-center mt-8") {
              "Social sites"
            }
            ul(class="list-none text-center mt-8") {
              li(class="inline") {
                a(class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600", href="") {
                  i(data-feather="facebook", class="h-4 w-4") {
                  }
                }
              }
              li(class="inline") {
                a(href="", class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600") {
                  i(class="h-4 w-4", data-feather="gitlab") {
                  }
                }
              }
            }
            h5(class="text-lg font-semibold bg-gray-50 dark:bg-slate-800 shadow dark:shadow-gray-800 rounded-md p-2 text-center mt-8") {
              "Tagscloud"
            }
            ul(class="list-none text-center mt-8") {
              li(class="inline-block m-2") {
                a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                  "Business"
                }
              }
              li(class="inline-block m-2") {
                a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                  "Video"
                }
              }
              li(class="inline-block m-2") {
                a(class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out", href="") {
                  "Audio"
                }
              }
            }
          }
        }
      }
    }

      }
}
