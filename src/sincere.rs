use num_cpus;
use sincere::App;

mod common;
use common::Result;

macro_rules! page_num {
    ($context: expr) => {
        match $context.request.query("page") {
            Some(p) => match p.parse::<i64>() {
                Ok(p) => p,
                Err(_) => {
                    let result = Result {
                        message: "parse page number error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => 1,
        }
    };
}

macro_rules! page_size {
    ($context: expr) => {
        match $context.request.query("per_page") {
            Some(p) => match p.parse::<i64>() {
                Ok(p) => p,
                Err(_) => {
                    let result = Result {
                        message: "parse page size error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => 20,
        }
    };
}

macro_rules! province_code {
    ($context: expr) => {
        match $context.request.query("province") {
            Some(p) => match p.parse::<i64>() {
                Ok(p) => p,
                Err(_) => {
                    let result = Result {
                        message: "parse province code error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => {
                let result = Result {
                    message: "province code must not be empty!",
                    data: vec![],
                };
                $context.response.from_json(result).unwrap();
                return;
            }
        }
    };
}

macro_rules! city_code {
    ($context: expr) => {
        match $context.request.query("city") {
            Some(c) => match c.parse::<i64>() {
                Ok(c) => c,
                Err(_) => {
                    let result = Result {
                        message: "parse city code error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => {
                let result = Result {
                    message: "city code must not be empty!",
                    data: vec![],
                };
                $context.response.from_json(result).unwrap();
                return;
            }
        }
    };
}

macro_rules! area_code {
    ($context: expr) => {
        match $context.request.query("area") {
            Some(a) => match a.parse::<i64>() {
                Ok(a) => a,
                Err(_) => {
                    let result = Result {
                        message: "parse area code error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => {
                let result = Result {
                    message: "area code must not be empty!",
                    data: vec![],
                };
                $context.response.from_json(result).unwrap();
                return;
            }
        }
    };
}

macro_rules! street_code {
    ($context: expr) => {
        match $context.request.query("street") {
            Some(s) => match s.parse::<i64>() {
                Ok(s) => s,
                Err(_) => {
                    let result = Result {
                        message: "parse street code error!",
                        data: vec![],
                    };
                    $context.response.from_json(result).unwrap();
                    return;
                }
            },
            None => {
                let result = Result {
                    message: "street code must not be empty!",
                    data: vec![],
                };
                $context.response.from_json(result).unwrap();
                return;
            }
        }
    };
}

fn main() {
    let mut app = App::new();

    // 省级
    app.get("/province_list", |context| {
        let province_list = common::province_list();
        context.response.from_json(province_list).unwrap();
    });

    // 地级
    app.get("/city_list", |context| {
        let province = province_code!(context);
        let page = page_num!(context);
        let per_page = page_size!(context);

        let city_list = common::city_list(province, page, per_page);
        context.response.from_json(city_list).unwrap();
    });

    // 县级
    app.get("/area_list", |context| {
        let province = province_code!(context);
        let city = city_code!(context);
        let page = page_num!(context);
        let per_page = page_size!(context);

        let area_list = common::area_list(province, city, page, per_page);
        context.response.from_json(area_list).unwrap();
    });

    // 乡级
    app.get("/street_list", |context| {
        let province = province_code!(context);
        let city = city_code!(context);
        let area = area_code!(context);
        let page = page_num!(context);
        let per_page = page_size!(context);

        let street_list = common::street_list(province, city, area, page, per_page);
        context.response.from_json(street_list).unwrap();
    });

    // 村级
    app.get("/village_list", |context| {
        let province = province_code!(context);
        let city = city_code!(context);
        let area = area_code!(context);
        let street = street_code!(context);
        let page = page_num!(context);
        let per_page = page_size!(context);

        let village_list = common::village_list(province, city, area, street, page, per_page);
        context.response.from_json(village_list).unwrap();
    });

    app.run("127.0.0.1:10000", num_cpus::get() * 2).unwrap();
}
