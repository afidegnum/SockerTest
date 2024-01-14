use crate::httpreq;
use crate::httpreq::model::BooksViews;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
// #[derive(Serialize, Deserialize, ReactiveState, Clone)]
// #[rx(alias = "BooksPageStateRx")]
// struct BooksPageState {
//     greeting: String,
// }

//#[cfg(engine)]
// use async_std::future;

// #[cfg(engine)]
// use std::time::Duration;
#[cfg(engine)]
use async_std::future;

#[cfg(client)]
use gloo_timers::future::sleep;
// #[cfg(client)]
use std::time::Duration;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "BooksPageStateRx")]
struct BooksPageState {
    #[rx(suspense = "book_handler")]
    contents: Result<Vec<BooksViews>, SerdeInfallible>,
}

#[browser_only_fn]
async fn book_handler<'a>(
    _cx: Scope<'a>,
    contents: &'a RcSignal<Result<Vec<BooksViews>, SerdeInfallible>>,
) -> Result<(), SerdeInfallible> {
    sleep(Duration::from_secs(1)).await;

    // Create a vector of BooksViews
    let books_views = vec![
        BooksViews {
            Book_Title: "Book Title 1".to_string(),
            ISBN: "ISBN 1".to_string(),
            Author_Name: "Author Name 1".to_string(),
            Country: "Country 1".to_string(),
        },
        // Add more BooksViews as needed
    ];

    contents.set(Ok(books_views));
    Ok(())
}

// #[browser_only_fn]
// async fn index_handler<'a>(
//     _cx: Scope<'a>,
//     contents: &'a RcSignal<Result<Vec<BooksViews>, SerdeInfallible>>,
// ) -> Result<(), SerdeInfallible> {
//     sleep(Duration::from_secs(1)).await;

//     // Create a vector of ContentViews
//     let content_views = vec![
//         BooksViews {
//             title: "Title 1".to_string(),
//             url: "URL 1".to_string(),
//             media_id: "Media ID 1".to_string(),
//             type_name: "Type Name 1".to_string(),
//         },
//         // Add more ContentViews as needed
//     ];

//     contents.set(Ok(content_views));
//     Ok(())
// }

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, page: &'a BooksPageStateRx) -> View<G> {
    // let widget = &page.contents;

    let widget = create_memo(cx, || match page.contents.get().as_ref().clone() {
        Ok(page) => page.clone(),
        Err(_) => unreachable!(),
    });

    let widg = widget;
    // let widget = create_rc_signal(*widg.clone().get());
    let widget_clone = create_signal(cx, widg);
    view! { cx,
    // code { (format!("API: {:?}", widget)) }
    div(class="container-fluid") {
      div(class="profile-banner relative text-transparent") {
        input(id="pro-banner", name="profile-banner", type="file", class="hidden") {
        }
        div(class="relative shrink-0") {
          img(alt="", class="h-96 w-full object-cover", src="assets/images/blog/bg.jpg", id="profile-banner") {
          }
          div(class="absolute inset-0 bg-black/70") {
          }
          label(class="absolute inset-0 cursor-pointer", for="pro-banner") {
          }
        }
      }
    }
    section(class="relative md:py-8 py-6") {
      div(class="container mt-6") {
        div(class="relative bg-white dark:bg-slate-900 lg:px-8 px-6 py-10 rounded-xl shadow dark:shadow-gray-800 overflow-hidden") {
          div(class="grid md:grid-cols-2 grid-cols-1 items-center gap-[30px]") {
            div(class="md:text-left text-center z-1") {
              h3(class="md:text-3xl text-2xl md:leading-normal leading-normal font-semibold") {
                "Subscribe the ISODEC CLOUD LIBRARY!"
              }
              p(class="text-slate-400 max-w-xl mx-auto mt-2") {
                "Arrays of books to enlighten yourself."
              }
            }
            div(class="subcribe-form z-1") {
              form(class="relative max-w-xl") {
                input(type="email", class="pt-4 pr-40 pb-4 pl-6 w-full h-[50px] outline-none text-black dark:text-white rounded-full bg-white dark:bg-slate-900 shadow dark:shadow-gray-800", placeholder="Your Email Address :", name="email", id="subcribe") {
                }
                button(type="submit", class="btn absolute top-[2px] right-[3px] h-[46px] bg-indigo-600 hover:bg-indigo-700 border-indigo-600 hover:border-indigo-700 text-white rounded-full") {
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
      div(class="container mt-16") {

        div(class="grid md:grid-cols-12 grid-cols-1 mt-8 gap-[30px]") {


          div(class="lg:col-span-8 md:col-span-6") {
            div(class="grid grid-cols-1 gap-[30px]") {(
                    view! { cx,

                                 Keyed(
                                      iterable=&widget_clone.get(),
                                      view= move |cx, x|

                                      {

                                         view! { cx,
                                            // (x.Author_Name) // (x.ISBN)  (x.Country)
              div(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
                div(class="lg:flex relative") {
                  div(class="relative md:shrink-0") {
                    img(class="h-full w-full object-cover lg:w-52 lg:h-56", src="assets/images/blog/02.jpg", alt="") {
                    }
                  }
                  div(class="p-6 flex flex-col lg:h-56 justify-center") {
                    a(href="blog-detail.html", class="title h5 text-lg font-medium hover:text-indigo-600 duration-500 ease-in-out") {
                        (x.Book_Title)
                    }
                    div(class="my-auto") {
                      p(class="text-slate-400 mt-3") {
                                            // (x.Author_Name) - (x.ISBN)  | (x.Country)
                                            "Author:"  span{(x.Author_Name)} "ISBN:  " span{(x.ISBN)} "Country:   "  span{(x.Country)}
                      }
                    }
                    div(class="mt-4") {
                      a(class="btn btn-link font-normal hover:text-indigo-600 after:bg-indigo-600 duration-500 ease-in-out", href="#") {
                        "Read More"
                        i(class="uil uil-arrow-right") {
                        }
                      }
                    }
                  }
                }
              }

                                          }
                                      },
                                      key= move |x| x.ISBN.as_str().to_owned(),
                                     )

                  }



      )}
          }



          div(class="lg:col-span-4 md:col-span-6") {
            div(class="sticky top-20") {
              h5(class="text-lg font-semibold bg-gray-50 dark:bg-slate-800 shadow dark:shadow-gray-800 rounded-md p-2 text-center") {
                "Recent post"
              }
              div(class="flex items-center mt-8") {
                img(class="h-16 rounded-md shadow dark:shadow-gray-800", alt="", src="assets/images/blog/06.jpg") {
                }
                div(class="ml-3") {
                  a(class="font-semibold hover:text-indigo-600", href="") {
                    "Consultant Business"
                  }
                  p(class="text-sm text-slate-400") {
                    "1st May 2022"
                  }
                }
              }
              div(class="flex items-center mt-4") {
                img(alt="", class="h-16 rounded-md shadow dark:shadow-gray-800", src="assets/images/blog/07.jpg") {
                }
                div(class="ml-3") {
                  a(href="", class="font-semibold hover:text-indigo-600") {
                    "Grow Your Business"
                  }
                  p(class="text-sm text-slate-400") {
                    "1st May 2022"
                  }
                }
              }
              div(class="flex items-center mt-4") {
                img(class="h-16 rounded-md shadow dark:shadow-gray-800", alt="", src="assets/images/blog/08.jpg") {
                }
                div(class="ml-3") {
                  a(class="font-semibold hover:text-indigo-600", href="") {
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
                  a(href="", class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600") {
                    i(data-feather="facebook", class="h-4 w-4") {
                    }
                  }
                }
                li(class="inline") {
                  a(class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600", href="") {
                    i(class="h-4 w-4", data-feather="instagram") {
                    }
                  }
                }
                li(class="inline") {
                  a(class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600", href="") {
                    i(data-feather="twitter", class="h-4 w-4") {
                    }
                  }
                }
                li(class="inline") {
                  a(class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600", href="") {
                    i(data-feather="linkedin", class="h-4 w-4") {
                    }
                  }
                }
                li(class="inline") {
                  a(class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600", href="") {
                    i(class="h-4 w-4", data-feather="github") {
                    }
                  }
                }
                li(class="inline") {
                  a(href="", class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600") {
                    i(data-feather="youtube", class="h-4 w-4") {
                    }
                  }
                }
                li(class="inline") {
                  a(href="", class="btn btn-icon btn-sm border border-gray-100 dark:border-gray-800 rounded-md text-slate-400 hover:border-indigo-600 hover:text-white hover:bg-indigo-600") {
                    i(data-feather="gitlab", class="h-4 w-4") {
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
                  a(class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out", href="") {
                    "Finance"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Marketing"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Fashion"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Bride"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Lifestyle"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Travel"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Beauty"
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
}

#[engine_only_fn]
fn head(cx: Scope, _props: BooksPageState) -> View<SsrNode> {
    view! { cx,
        title { "Library | Isodec" }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<BooksPageState, BlamedError<httpreq::MyError>> {
    use crate::httpreq;
    let body = perseus::utils::cache_fallible_res(
        "library",
        || async {
            let path = "/views/books";
            let res = httpreq::request_client::<Vec<BooksViews>>(path.to_string()).await?;
            let res = future::timeout(
                Duration::from_secs(15),
                httpreq::request_client::<Vec<BooksViews>>(path.to_string()),
            )
            .await?;
            Ok::<Vec<BooksViews>, httpreq::MyError>(res.map_err(|e| e.to_string())?)
        },
        true,
    )
    .await?;

    Ok(BooksPageState { contents: Ok(body) })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("libraries")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
