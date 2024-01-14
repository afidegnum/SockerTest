use crate::httpreq;
use crate::httpreq::model::ContentViews;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// #[cfg(engine)]
// use async_std::future;

#[cfg(engine)]
use async_std::future;

#[cfg(client)]
use gloo_timers::future::sleep;
// #[cfg(client)]
use std::time::Duration;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "LatestPageStateRx")]
struct LatestPageState {
    #[rx(suspense = "latest_handler")]
    contents: Result<Vec<ContentViews>, SerdeInfallible>,
}

#[browser_only_fn]
async fn latest_handler<'a>(
    _cx: Scope<'a>,
    contents: &'a RcSignal<Result<Vec<ContentViews>, SerdeInfallible>>,
) -> Result<(), SerdeInfallible> {
    sleep(Duration::from_secs(1)).await;

    // Create a vector of ContentViews
    let content_views = vec![
        ContentViews {
            title: "Title 1".to_string(),
            url: "URL 1".to_string(),
            media_id: "Media ID 1".to_string(),
            type_name: "Type Name 1".to_string(),
        },
        // Add more ContentViews as needed
    ];

    contents.set(Ok(content_views));
    Ok(())
}

fn latest_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, page: &'a LatestPageStateRx) -> View<G> {
    // let widget = &page.contents;
    let widget = create_memo(cx, || match page.contents.get().as_ref().clone() {
        Ok(page) => page.clone(),
        Err(_) => unreachable!(),
    });

    let widg = widget;
    // let widget = create_rc_signal(*widg.clone().get());
    let widget_clone = create_signal(cx, widg);
    view! { cx,


    div(d-comp="headings", class="container-fluid") {
      div(class="profile-banner relative text-transparent") {
        input(name="profile-banner", onchange="loadFile(event)", class="hidden", type="file", id="pro-banner") {
        }
        div(class="relative shrink-0") {
          img(id="profile-banner", class="h-96 w-full object-cover", src="assets/images/blog/bg.jpg", alt="") {
          }
          div(class="absolute inset-0 bg-black/70") {
          }
          label(class="absolute inset-0 cursor-pointer", for="pro-banner") {
          }
        }
      }
    }
    section(class="relative md:py-24 py-16") {
      div(class="container") {

                  div {(
                    view! { cx,
                          // ul() { //
                           div(d-obj="content", d-comp="grid", class="grid grid-cols-1 lg:grid-cols-3 md:grid-cols-2 gap-[30px] sm:-mt-[220px] -mt-[200px] m-0") {
                                 Keyed(
                                      iterable=&widget_clone.get(),
                                      view= move |cx, x|
                                      {


                                          // let title = x.title;
                                          // let new_title = &title;
                                          let path = x.url;
                                          let img = x.media_id;

                                         view! { cx,

                                          // let title = x.title.clone()


                                        div(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
                                          img(src=(format!("assets/images/iso/{}", img)) , alt="----------") {
                                          }
                                          div(class="content p-6") {
                                            a(class="title h5 text-lg font-medium hover:text-indigo-600 duration-500 ease-in-out", href="blog-detail.html", d-field="url") {
                                                (x.title.clone())
                                            }
                                            p(d-field="summary", class="text-slate-400 mt-3") {
                                                    // (x.title.clone())
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
                                      },
                                      key= move |x| x.url.as_str().to_owned(),
                                  )

                          } //
                  }
               //  )}

          // } //

                )} //end of views


      //   div(d-obj="content", d-comp="grid", class="grid grid-cols-1 lg:grid-cols-3 md:grid-cols-2 gap-[30px] sm:-mt-[220px] -mt-[200px] m-0") {
      // // --
      //     div(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
      //       img(d-field="media_id", src="assets/images/blog/08.jpg", alt="") {
      //       }
      //       div(class="content p-6") {
      //         a(class="title h5 text-lg font-medium hover:text-indigo-600 duration-500 ease-in-out", href="blog-detail.html", d-field="url") {
      //           "On the other hand we provide denounce"
      //         }
      //         p(d-field="summary", class="text-slate-400 mt-3") {
      //           "The phrasal sequence of the is now so that many campaign and benefit"
      //         }
      //         div(class="mt-4") {
      //           a(class="btn btn-link font-normal hover:text-indigo-600 after:bg-indigo-600 duration-500 ease-in-out", href="blog-detail.html") {
      //             "Read More"
      //             i(class="uil uil-arrow-right") {
      //             }
      //           }
      //         }
      //       }
      //     }
      //     //---
      //   }
        div(class="grid md:grid-cols-12 grid-cols-1 mt-8") {
          div(class="md:col-span-12 text-center") {
            nav(aria-label="Page navigation example") {
              ul(class="inline-flex items-center -space-x-px") {
                li() {
                  a(href="#", aria-current="page", class="z-10 w-[40px] h-[40px] inline-flex justify-center items-center text-white bg-indigo-600 border border-indigo-600") {
                    "3"
                  }
                }
                li() {
                  a(class="w-[40px] h-[40px] inline-flex justify-center items-center text-slate-400 bg-white dark:bg-slate-900 rounded-r-lg hover:text-white border border-gray-100 dark:border-gray-700 hover:border-indigo-600 dark:hover:border-indigo-600 hover:bg-indigo-600 dark:hover:bg-indigo-600", href="#") {
                    i(class="uil uil-angle-right text-[20px]") {
                    }
                  }
                }
              }
            }
          }
        }
      }
      div(class="container mt-16") {
        div(class="relative bg-white dark:bg-slate-900 lg:px-8 px-6 py-10 rounded-xl shadow dark:shadow-gray-800 overflow-hidden", d-comp="newsletters") {
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
                input(id="subcribe", name="email", type="email", class="pt-4 pr-40 pb-4 pl-6 w-full h-[50px] outline-none text-black dark:text-white rounded-full bg-white dark:bg-slate-900 shadow dark:shadow-gray-800", placeholder="Your Email Address :") {
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
      div(d-comp="featured", class="container mt-16") {
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
                    img(src="assets/images/blog/08.jpg", alt="", class="h-full w-full object-cover lg:w-52 lg:h-56") {
                    }
                  }
                  div(class="p-6 flex flex-col lg:h-56 justify-center") {
                    a(class="title h5 text-lg font-medium hover:text-sky-600 duration-500 ease-in-out", href="blog-detail.html") {
                      "This mountain will make you reborn"
                    }
                    div(class="my-auto") {
                      p(class="text-slate-400 mt-3") {
                        "The phrasal sequence of the is now so that many campaign and benefit"
                      }
                    }
                    div(class="mt-4") {
                      a(class="btn btn-link font-normal hover:text-sky-600 after:bg-indigo-600 duration-500 ease-in-out", href="blog-detail.html") {
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
                img(alt="", src="assets/images/blog/08.jpg", class="h-16 rounded-md shadow dark:shadow-gray-800") {
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
                    "Video"
                  }
                }
                li(class="inline-block m-2") {
                  a(href="", class="px-3 py-1 text-slate-400 hover:text-white dark:hover:text-white bg-gray-50 dark:bg-slate-800 text-sm hover:bg-indigo-600 dark:hover:bg-indigo-600 rounded-md shadow dark:shadow-gray-800 transition-all duration-500 ease-in-out") {
                    "Audio"
                  }
                }
              }
            }
          }
        }
      }
    }


    a(class="back-to-top fixed hidden text-lg rounded-full z-10 bottom-5 right-5 h-9 w-9 text-center bg-indigo-600 text-white leading-9", onclick="topFunction()", id="back-to-top", href="#") {
      i(class="uil uil-arrow-up") {
      }
    }
    div(class="fixed top-1/4 -right-2 z-50") {
      span(class="relative inline-block rotate-90") {
        input(id="chk", type="checkbox", class="checkbox opacity-0 absolute") {
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

#[engine_only_fn]
fn head(cx: Scope, _props: LatestPageState) -> View<SsrNode> {
    view! { cx,
        title { "Isodec | Latest" }
    }
}

// #[engine_only_fn]
// async fn get_build_state(_info: StateGeneratorInfo<()>) -> LatestPageState {
//     LatestPageState {
//         greeting: "---".to_string(),
//     }
// }

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<LatestPageState, BlamedError<httpreq::MyError>> {
    use crate::httpreq;

    let body = perseus::utils::cache_fallible_res(
        "index",
        || async {
            let path = "/views/news/6";
            let res = future::timeout(
                Duration::from_secs(15),
                httpreq::request_client::<Vec<ContentViews>>(path.to_string()),
            )
            .await?;

            Ok::<Vec<ContentViews>, httpreq::MyError>(res.map_err(|e| e.to_string())?)
        },
        true,
    )
    .await?;

    Ok(LatestPageState { contents: Ok(body) })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("latest")
        .build_state_fn(get_build_state)
        .view_with_state(latest_page)
        .head_with_state(head)
        .build()
}
