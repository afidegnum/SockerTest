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
