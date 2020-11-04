use serde::Serialize;
use sqlite;
use std::path::Path;

#[derive(Serialize)]
pub struct Item {
    pub code: u32,
    pub name: String,
}

#[derive(Serialize)]
pub struct Result {
    pub message: &'static str,
    pub data: Vec<Item>,
}

pub fn new_connection() -> sqlite::Connection {
    let parent = Path::new(file!()).parent().unwrap();
    let dbfile = parent.join("data.sqlite");
    sqlite::open(dbfile).unwrap()
}

pub fn province_list() -> Result {
    let connection = new_connection();
    let mut statement = connection
        .prepare("select code, name from province order by code")
        .unwrap();

    let mut province_list = Result {
        message: "ok",
        data: vec![],
    };
    while let sqlite::State::Row = statement.next().unwrap() {
        let province = Item {
            code: statement.read::<i64>(0).unwrap() as u32,
            name: statement.read::<String>(1).unwrap(),
        };
        province_list.data.push(province);
    }
    province_list
}

pub fn city_list(province: i64, page: i64, per_page: i64) -> Result {
    let connection = new_connection();
    let mut statement = connection
        .prepare(
            "select code, name from city where provinceCode = ? order by code limit ? offset ?",
        )
        .unwrap();
    statement.bind(1, province).unwrap();
    statement.bind(2, per_page).unwrap();
    statement.bind(3, (page - 1) * per_page).unwrap();

    let mut city_list = Result {
        message: "ok",
        data: vec![],
    };
    while let sqlite::State::Row = statement.next().unwrap() {
        let city = Item {
            code: statement.read::<i64>(0).unwrap() as u32,
            name: statement.read::<String>(1).unwrap(),
        };
        city_list.data.push(city);
    }
    city_list
}

pub fn area_list(province: i64, city: i64, page: i64, per_page: i64) -> Result {
    let connection = new_connection();
    let mut statement = connection
            .prepare(
                "select code, name from area where provinceCode = ? and cityCode = ? order by code limit ? offset ?",
            )
            .unwrap();
    statement.bind(1, province).unwrap();
    statement.bind(2, city).unwrap();
    statement.bind(3, per_page).unwrap();
    statement.bind(4, (page - 1) * per_page).unwrap();

    let mut area_list = Result {
        message: "ok",
        data: vec![],
    };
    while let sqlite::State::Row = statement.next().unwrap() {
        let area = Item {
            code: statement.read::<i64>(0).unwrap() as u32,
            name: statement.read::<String>(1).unwrap(),
        };
        area_list.data.push(area);
    }
    area_list
}

pub fn street_list(province: i64, city: i64, area: i64, page: i64, per_page: i64) -> Result {
    let connection = new_connection();
    let mut statement = connection
            .prepare(
                "select code, name from street where provinceCode = ? and cityCode = ? and areaCode = ? order by code limit ? offset ?",
            )
            .unwrap();
    statement.bind(1, province).unwrap();
    statement.bind(2, city).unwrap();
    statement.bind(3, area).unwrap();
    statement.bind(4, per_page).unwrap();
    statement.bind(5, (page - 1) * per_page).unwrap();

    let mut street_list = Result {
        message: "ok",
        data: vec![],
    };
    while let sqlite::State::Row = statement.next().unwrap() {
        let street = Item {
            code: statement.read::<i64>(0).unwrap() as u32,
            name: statement.read::<String>(1).unwrap(),
        };
        street_list.data.push(street);
    }
    street_list
}

pub fn village_list(
    province: i64,
    city: i64,
    area: i64,
    street: i64,
    page: i64,
    per_page: i64,
) -> Result {
    let connection = new_connection();
    let mut statement = connection
            .prepare(
                "select code, name from village where provinceCode = ? and cityCode = ? and areaCode = ? and streetCode = ? order by code limit ? offset ?",
            )
            .unwrap();
    statement.bind(1, province).unwrap();
    statement.bind(2, city).unwrap();
    statement.bind(3, area).unwrap();
    statement.bind(4, street).unwrap();
    statement.bind(5, per_page).unwrap();
    statement.bind(6, (page - 1) * per_page).unwrap();

    let mut village_list = Result {
        message: "ok",
        data: vec![],
    };
    while let sqlite::State::Row = statement.next().unwrap() {
        let village = Item {
            code: statement.read::<i64>(0).unwrap() as u32,
            name: statement.read::<String>(1).unwrap(),
        };
        village_list.data.push(village);
    }
    village_list
}
