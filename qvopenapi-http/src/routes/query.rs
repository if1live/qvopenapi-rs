use std::{convert::Infallible, sync::Arc};

use qvopenapi_async::{models::*, QvOpenApiAsyncClient};
use warp::{
    filters::{body, method::post, BoxedFilter},
    http::StatusCode,
    reply::{self, Reply},
    Filter,
};

use crate::error;

pub fn filter_c8201(client: Arc<QvOpenApiAsyncClient>) -> BoxedFilter<(impl Reply,)> {
    let cloned = client.clone();
    let handler = move |req: C8201Request| query_c8201(cloned.clone(), req);
    post()
        .and(warp::path!("query" / "c8201"))
        .and(body::json())
        .and_then(handler)
        .boxed()
}

async fn query_c8201(
    client: Arc<QvOpenApiAsyncClient>,
    request: C8201Request,
) -> Result<impl Reply, Infallible> {
    let ret = client.query(request.into_raw()).await;

    if ret.is_err() {
        return error::convert_error(ret.err().unwrap());
    }

    let result = ret.unwrap();

    let error = result.get("error");
    if error.is_some() && !error.unwrap().is_null() {
        return Ok(reply::with_status(
            reply::json(&result),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub fn filter_c8010(client: Arc<QvOpenApiAsyncClient>) -> BoxedFilter<(impl Reply,)> {
    let cloned = client.clone();
    let handler = move |req: C8010Request| query_c8010(cloned.clone(), req);
    post()
        .and(warp::path!("query" / "c8010"))
        .and(body::json())
        .and_then(handler)
        .boxed()
}

async fn query_c8010(
    client: Arc<QvOpenApiAsyncClient>,
    request: C8010Request,
) -> Result<impl Reply, Infallible> {
    let ret = client.query(request.into_raw()).await;

    if ret.is_err() {
        return error::convert_error(ret.err().unwrap());
    }

    let result = ret.unwrap();

    let error = result.get("error");
    if error.is_some() && !error.unwrap().is_null() {
        return Ok(reply::with_status(
            reply::json(&result),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub fn filter_s8064(client: Arc<QvOpenApiAsyncClient>) -> BoxedFilter<(impl Reply,)> {
    let cloned = client.clone();
    let handler = move |req: S8064Request| query_s8064(cloned.clone(), req);
    post()
        .and(warp::path!("query" / "s8064"))
        .and(body::json())
        .and_then(handler)
        .boxed()
}

async fn query_s8064(
    client: Arc<QvOpenApiAsyncClient>,
    request: S8064Request,
) -> Result<impl Reply, Infallible> {
    let ret = client.query(request.into_raw()).await;

    if ret.is_err() {
        return error::convert_error(ret.err().unwrap());
    }

    let result = ret.unwrap();

    let error = result.get("error");
    if error.is_some() && !error.unwrap().is_null() {
        return Ok(reply::with_status(
            reply::json(&result),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}
