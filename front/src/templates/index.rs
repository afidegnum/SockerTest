use crate::httpreq;
use crate::httpreq::model::ContentViews;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::*;
// #[derive(Serialize, Deserialize, ReactiveState, Clone)]
// #[rx(alias = "IndexPageStateRx")]
// struct IndexPageState {
//     greeting: String,
// }

#[cfg(engine)]
use async_std::future;

#[cfg(client)]
use gloo_timers::future::sleep;
// #[cfg(client)]
use std::time::Duration;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    #[rx(suspense = "index_handler")]
    contents: Result<Vec<ContentViews>, SerdeInfallible>,
}

#[browser_only_fn]
async fn index_handler<'a>(
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

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, page: &'a IndexPageStateRx) -> View<G> {
    // let widget = create_ref(cx, page.contents);

    let widget = create_memo(cx, || match page.contents.get().as_ref().clone() {
        Ok(page) => page.clone(),
        Err(_) => unreachable!(),
    });

    // let widget = create_memo(cx, || match Rc::try_unwrap(page.contents.get().clone()) {
    //     Ok(Ok(page)) => page,
    //     Ok(Err(_)) | Err(_) => unreachable!(),
    // });

    let widg = widget;
    // let widget = create_rc_signal(*widg.clone().get());
    let widget_clone = create_signal(cx, widg);
    view! { cx,
    // code { (format!("API: {:?}", widget)) }
      section(class="relative table w-full py-36 lg:py-64 bg-[url('../../assets/images/it/bg.jpg')] bg-no-repeat sm:bg-left bg-right overflow-hidden") {
        div(class="container relative z-1") {
          div(class="grid md:grid-cols-12 grid-cols-1 items-center mt-10 gap-[30px]") {
            div(class="md:col-span-7") {
              div(class="md:mr-6") {
                span(class="text-xl font-semibold") {
                  "ISODEC"
                }
                h3(class="font-bold lg:leading-normal leading-normal text-4xl lg:text-6xl mb-5 text-black dark:text-white") {
                  span(class="after:absolute after:right-0 after:left-0 after:bottom-2 after:h-1.5 after:w-auto after:bg-gradient-to-l after:to-blue-900 after:from-green-600 relative") {
                    "Promoting"
                  }
                  br() {
                  }
                  "Community Development " br{}
                  span(class="after:absolute after:right-0 after:left-0 after:bottom-2 after:h-1.5 after:w-auto after:bg-gradient-to-l after:to-blue-900 after:from-green-600 relative") {
                    "& Social Justice"
                  }
                }
                p(class="text-slate-400 text-lg max-w-xl") {
                  "The Integrated Social Devlopment Center (Isodec) is .... "
                }
                div(class="mt-6") {
                  a(class="btn bg-blue-900 hover:bg-indigo-700 border-blue-900 hover:border-indigo-700 text-white rounded-md", href="") {
                    "More about us"
                  }
                  // a(href="#!", data-id="yba7hPeTSjk", class="btn btn-icon rounded-full bg-blue-900 hover:bg-indigo-700 border-blue-900 hover:border-indigo-700 text-white m-1 lightbox") {
                  //   i(data-feather="video", class="h-4 w-4") {
                  //   }
                  // }
                  span(class="font-semibold ml-1 align-middle") {
                  }
                }
              }
            }
          }
        }
        div(class="absolute md:w-3/4 w-full top-1/2 -translate-y-1/2") {
          div(class="absolute w-full h-[5000px] bg-slate-50 dark:bg-slate-800 md:opacity-100 opacity-90 top-1/2 -translate-y-1/2 md:-left-[10%] -left-[25%] rotate-12") {
          }
          div(class="absolute md:w-48 w-20 h-[5000px] bg-blue-900 top-1/2 -translate-y-1/2 md:right-[10%] -right-[1%] rotate-12") {
          }
        }
      }
      section(class="relative md:pb-16 pb-12") {
        div(class="container md:mt-20 mt-12") {
          div(class="grid md:grid-cols-12 grid-cols-1 items-center") {
            div(class="md:col-span-6") {
              h6(class="text-blue-900 text-sm font-bold uppercase mb-2") {
              }
              h3(class="mb-4 md:text-3xl md:leading-normal text-2xl leading-normal font-semibold") {
                "News & Blog"
              }
            }
            div(class="md:col-span-6") {
              p(class="text-slate-400 max-w-xl") {
                "Latest Community development and social justice updates from isodec."
              }
            }
          }
                  div {(
                    view! { cx,
                          // ul() { //
                          ul(class="grid grid-cols-1 lg:grid-cols-3 md:grid-cols-2 mt-8 gap-[30px]") {
                                  Keyed(
                                      iterable=&widget_clone.get(),
                                      view= move |cx, x|
                                      {

                                          /*
                                          let desc = match x.title.get("title") {
                                              Some(x) => x.to_string(),
                                              None => "".to_string(),
                                          };

                                          let path = x.url.get("url").unwrap().as_str().to_owned();
                                          */
                                          let title = x.title;
                                          let path = x.url;
                                          let img = x.media_id;
                                          view! { cx,
                                                    // li (class="nav-item") { a (class="nav-link active", aria-current="page", href=path) {(title)} }

                                                        li(class="blog relative rounded-md shadow dark:shadow-gray-800 overflow-hidden") {
                                                          img(alt="", src=(format!("assets/images/iso/{}", img)) ) {
                                                          }
                                                          div(class="content p-6") {
                                                            a(class="title h5 text-lg font-medium hover:text-blue-900 duration-500 ease-in-out", href="blog-detail.html") {
                                                            (title)
                                                            }
                                                            p(class="text-slate-400 mt-3") {
                                                              "The phrasal sequence of the is now so that many campaign and benefit"
                                                            }
                                                            div(class="mt-4") {
                                                              a(class="btn btn-link font-normal hover:text-blue-900 after:bg-blue-900 duration-500 ease-in-out", href="blog-detail.html") {
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
        }
      }
      section(class="relative md:py-18 py-12") {
        div(class="container md:mt-22 mt-12") {
          div(class="grid md:grid-cols-12 grid-cols-1 items-center gap-[30px]") {
            div(class="md:col-span-6") {
              div(class="lg:mr-8") {
                div(class="relative") {
                  img(alt="", class="rounded-full shadow-lg dark:shadow-gray-800", src="assets/images/it/about.jpg") {
                  }
                  div(class="absolute top-1/2 -translate-y-1/2 right-0 left-0 mx-auto lg:w-64 w-52 lg:h-64 h-52 flex justify-center items-center bg-white dark:bg-slate-900 rounded-full shadow-lg dark:shadow-gray-800") {
                    div(class="text-center") {
                      span(class="text-blue-900 text-2xl font-bold mb-0 block") {
                        span(data-target="15", class="counter-value text-6xl font-bold") {
                          "35"
                        }
                        "+"
                      }
                      span(class="font-semibold block mt-2") {
                        "Years"
                        br() {
                        }
                        "Experience"
                      }
                    }
                  }
                }
              }
            }
            div(class="md:col-span-6") {
              div(class="lg:ml-8") {
                h6(class="text-blue-900 text-sm font-bold uppercase mb-2") {
                  "Unmatched track record"
                }
                h3(class="mb-4 md:text-3xl md:leading-normal text-2xl leading-normal font-semibold") {
                  "Isodec achieved ... "
                  br() {
                  }
                  "partnered with various"
                  br() {
                  }
                  "in communities."
                }
                p(class="text-slate-400 max-w-xl mb-6") {
                  "and much more with just a few taps. commodo consequat. Duis aute irure."
                }
                a(class="btn bg-blue-900 hover:bg-indigo-700 border-blue-900 hover:border-indigo-700 text-white rounded-md", href="page-services.html") {
                  i(class="uil uil-notes") {
                  }
                  "Get Started"
                }
              }
            }
          }
        }
      }


      }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Isodec" }
    }
}

// // #[engine_only_fn]
// async fn get_request_state(
//     _info: StateGeneratorInfo<()>,
//     req: Request,
// ) -> Result<IndexPageState, BlamedError<std::convert::Infallible>> {
//     use crate::SOCKET_ADDR;
//     println!("{:#?}", req);
//     Ok(IndexPageState {
//         // ip: format!(
//         //     "{:?}",
//         //     req.headers()
//         //         // NOTE: This header can be trivially spoofed, and may well not be the user's actual
//         //         // IP address
//         //         .get("X-Forwarded-For")
//         //         .unwrap_or(&perseus::http::HeaderValue::from_str("hidden from view!").unwrap())
//         // ),
//         contents: todo!(),
//     })
// }

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<httpreq::MyError>> {
    use crate::httpreq;
    let body = perseus::utils::cache_fallible_res(
        "index",
        || async {
            let path = "/views/news/3";
            // let res = httpreq::request_client::<Vec<ContentViews>>(path.to_string()).await?;
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

    Ok(IndexPageState { contents: Ok(body) })
}

/*
#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: "---".to_string(),
    }
}
*/
pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        //.request_state_fn(get_request_state)
        .head_with_state(head)
        .build()
}
