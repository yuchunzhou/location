use serde_json;
use std::collections::HashMap;
use tokio;
use warp::http::Response;
use warp::Filter;

mod common;
use common::Result;

macro_rules! build_response {
    ($body: expr) => {
        Response::builder()
            .header("Content-Type", "application/json; charset=UTF-8")
            .body(serde_json::to_string($body).unwrap())
    };
}

macro_rules! province_code {
    ($query: expr) => {
        match $query.get("province") {
            Some(p) => p,
            None => {
                return build_response!(&Result {
                    message: "province code must not be empty!",
                    data: vec![],
                });
            }
        }
    };
}

macro_rules! city_code {
    ($query: expr) => {
        match $query.get("city") {
            Some(c) => c,
            None => {
                return build_response!(&Result {
                    message: "city code must not be empty!",
                    data: vec![],
                });
            }
        }
    };
}

macro_rules! area_code {
    ($query: expr) => {
        match $query.get("area") {
            Some(a) => a,
            None => {
                return build_response!(&Result {
                    message: "area code must not be empty!",
                    data: vec![],
                });
            }
        }
    };
}

macro_rules! street_code {
    ($query: expr) => {
        match $query.get("street") {
            Some(s) => s,
            None => {
                return build_response!(&Result {
                    message: "street code must not be empty!",
                    data: vec![],
                });
            }
        }
    };
}

#[tokio::main]
async fn main() {
    // 省级
    let province_list = warp::get()
        .and(warp::path("province_list"))
        .map(|| build_response!(&common::province_list()));

    // 地级
    let city_list = warp::get()
        .and(warp::path("city_list"))
        .and(warp::query::<HashMap<String, i64>>())
        .map(|query: HashMap<String, i64>| {
            let page = query.get("page").unwrap_or(&1);
            let per_page = query.get("per_page").unwrap_or(&20);
            let province = province_code!(query);

            build_response!(&common::city_list(*province, *page, *per_page))
        });

    // 县级
    let area_list = warp::get()
        .and(warp::path("area_list"))
        .and(warp::query::<HashMap<String, i64>>())
        .map(|query: HashMap<String, i64>| {
            let page = query.get("page").unwrap_or(&1);
            let per_page = query.get("per_page").unwrap_or(&20);
            let province = province_code!(query);
            let city = city_code!(query);

            build_response!(&common::area_list(*province, *city, *page, *per_page))
        });

    // 乡级
    let street_list = warp::get()
        .and(warp::path("street_list"))
        .and(warp::query::<HashMap<String, i64>>())
        .map(|query: HashMap<String, i64>| {
            let page = query.get("page").unwrap_or(&1);
            let per_page = query.get("per_page").unwrap_or(&20);
            let province = province_code!(query);
            let city = city_code!(query);
            let area = area_code!(query);

            build_response!(&common::street_list(
                *province, *city, *area, *page, *per_page
            ))
        });

    // 村级
    let village_list = warp::get()
        .and(warp::path("village_list"))
        .and(warp::query::<HashMap<String, i64>>())
        .map(|query: HashMap<String, i64>| {
            let page = query.get("page").unwrap_or(&1);
            let per_page = query.get("per_page").unwrap_or(&20);
            let province = province_code!(query);
            let city = city_code!(query);
            let area = area_code!(query);
            let street = street_code!(query);

            build_response!(&common::village_list(
                *province, *city, *area, *street, *page, *per_page
            ))
        });

    warp::serve(
        province_list
            .or(city_list)
            .or(area_list)
            .or(street_list)
            .or(village_list),
    )
    .run(([127, 0, 0, 1], 20000))
    .await;
}
