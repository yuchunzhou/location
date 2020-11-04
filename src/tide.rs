use tide::prelude::*;
use tide::Request;

mod common;

#[derive(Deserialize)]
#[serde(default)]
struct Query {
    page: i64,
    per_page: i64,
    province: i64,
    city: i64,
    area: i64,
    street: i64,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
            province: 0,
            city: 0,
            area: 0,
            street: 0,
        }
    }
}

macro_rules! check_province {
    ($query: expr) => {
        if $query.province == 0 {
            return Ok(json!(common::Result {
                message: "province code must not be empty!",
                data: vec![],
            }));
        }
    };
}

macro_rules! check_city {
    ($query: expr) => {
        if $query.city == 0 {
            return Ok(json!(common::Result {
                message: "city code must not be empty!",
                data: vec![],
            }));
        }
    };
}

macro_rules! check_area {
    ($query: expr) => {
        if $query.area == 0 {
            return Ok(json!(common::Result {
                message: "area code must not be empty!",
                data: vec![],
            }));
        }
    };
}

macro_rules! check_street {
    ($query: expr) => {
        if $query.street == 0 {
            return Ok(json!(common::Result {
                message: "street code must not be empty!",
                data: vec![],
            }));
        }
    };
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();

    // 省级
    app.at("/province_list")
        .get(|_| async { Ok(json!(common::province_list())) });

    // 地级
    app.at("/city_list").get(|req: Request<_>| async move {
        let query: Query = req.query()?;
        check_province!(query);
        Ok(json!(common::city_list(
            query.province,
            query.page,
            query.per_page
        )))
    });

    // 县级
    app.at("/area_list").get(|req: Request<_>| async move {
        let query: Query = req.query()?;
        check_province!(query);
        check_city!(query);
        Ok(json!(common::area_list(
            query.province,
            query.city,
            query.page,
            query.per_page
        )))
    });

    // 乡级
    app.at("/street_list").get(|req: Request<_>| async move {
        let query: Query = req.query()?;
        check_province!(query);
        check_city!(query);
        check_area!(query);
        Ok(json!(common::street_list(
            query.province,
            query.city,
            query.area,
            query.page,
            query.per_page
        )))
    });

    // 村级
    app.at("/village_list").get(|req: Request<_>| async move {
        let query: Query = req.query()?;
        check_province!(query);
        check_city!(query);
        check_area!(query);
        check_street!(query);
        Ok(json!(common::village_list(
            query.province,
            query.city,
            query.area,
            query.street,
            query.page,
            query.per_page
        )))
    });

    app.listen("127.0.0.1:30000").await?;
    Ok(())
}
