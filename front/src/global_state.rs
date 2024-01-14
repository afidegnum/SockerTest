use perseus::{prelude::*, state::GlobalStateCreator};
use serde::{Deserialize, Serialize};

pub fn get_global_state_creator() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
    //.request_state_fn(get_request_state)
    //.amalgamate_states_fn(amalgamate_states)
}

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    contents: Vec<ContentViews>,
}

#[engine_only_fn]
async fn get_build_state(
    _info: StateGeneratorInfo<()>,
) -> Result<IndexPageState, BlamedError<hyper::Error>> {
    use crate::httpreq;
    let body = perseus::utils::cache_fallible_res(
        "index",
        || async {
            let path = "/views/news/3";
            let res = httpreq::client_request::<Vec<ContentViews>>(path.to_string()).await?;
            Ok::<Vec<ContentViews>, hyper::Error>(res)
        },
        false,
    )
    .await?;

    Ok(IndexPageState { contents: body })
}
