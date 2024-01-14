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
