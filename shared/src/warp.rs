use warp::{http::Response as HttpResponse, Filter, Rejection, Reply, reject};

pub fn enable(is_enabled: bool) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::any()
        .and_then(move || async move {
            if is_enabled {
                Ok(())
            } else {
                // or warp::reject::custom if something besides 404
                Err(warp::reject::not_found())
            }
        },)
        // this weirdo deals with the Ok(())
        .untuple_one()
}
