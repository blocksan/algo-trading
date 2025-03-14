use chrono::prelude::*;

// pub fn date_util() {
pub const DESIRED_STOCK_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%Z";
const FILE_STOCK_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";
const _DESIRED_STOCK_ORDER_CACHE_DATETIME_FORMAT: &str = "%Y_%m_%d_%H:%M:%S";

#[allow(dead_code, unused_variables)]
pub fn parse_date_in_stock_format(date: &str) -> Result<String, String> {
    let date_time = DateTime::parse_from_str(date, FILE_STOCK_DATETIME_FORMAT);
    match date_time {
        Ok(_) => Ok(date.to_string()),
        Err(_) => Err(format!("Error parsing date: {}", date)),
    }
    // Ok(date_time.format(DESIRED_STOCK_DATETIME_FORMAT).to_string())
}
#[allow(dead_code, unused_variables)]
pub fn new_current_date_time_in_desired_stock_datetime_format() -> String {
    let now_time: DateTime<Local> = Local::now();
    now_time.format(DESIRED_STOCK_DATETIME_FORMAT).to_string()
}

#[allow(dead_code, unused_variables)]
pub fn is_date1_greater_than_date2(date1: &str, date2: &str) -> bool {
    let date_time1 = DateTime::parse_from_str(date1, FILE_STOCK_DATETIME_FORMAT).unwrap();
    let date_time2 = DateTime::parse_from_str(date2, FILE_STOCK_DATETIME_FORMAT).unwrap();
    date_time1 > date_time2
}

#[allow(dead_code, unused_variables)]
pub fn date_time_difference_in_seconds(date1: &str, date2: &str) -> i64 {
    let date_time1 = DateTime::parse_from_str(date1, FILE_STOCK_DATETIME_FORMAT).unwrap();
    let date_time2 = DateTime::parse_from_str(date2, FILE_STOCK_DATETIME_FORMAT).unwrap();
    let difference = date_time1.signed_duration_since(date_time2);
    difference.num_seconds()
}

#[allow(dead_code, unused_variables)]
pub fn return_only_date_from_datetime(date: &str) -> String {
    let date_time = DateTime::parse_from_str(date, FILE_STOCK_DATETIME_FORMAT).unwrap();
    date_time.format("%Y_%m_%d").to_string()
}

#[allow(dead_code, unused_variables)]
pub fn return_only_time_from_datetime(date_option: Option<String>) -> NaiveTime {
    let date = if date_option.is_none() {
        new_current_date_time_in_desired_stock_datetime_format()
    } else {
        date_option.unwrap()
    };
    let date_time = 
    match DateTime::parse_from_str(date.as_str(), DESIRED_STOCK_DATETIME_FORMAT){
        Ok(date_time) => date_time,
        Err(e) => {
            DateTime::parse_from_str(date.as_str(), FILE_STOCK_DATETIME_FORMAT).unwrap()
        }
    };
    date_time.time()
}

#[allow(dead_code, unused_variables)]
pub fn if_first_time_greater_than_second_time(
    first_time: Option<NaiveTime>,
    second_time: Option<NaiveTime>,
) -> bool {
    
    if first_time.is_none() || second_time.is_none() {
        return false;
    }

    if first_time.unwrap() > second_time.unwrap() {
        return true;
    }

    false
}

pub fn return_days_between_dates(date1: &str, date2: &str, returns_days: bool) -> (i64, Option<Vec<NaiveDate>>) {
    let date_time1 = DateTime::parse_from_str(date1, FILE_STOCK_DATETIME_FORMAT).unwrap();
    let date_time2 = DateTime::parse_from_str(date2, FILE_STOCK_DATETIME_FORMAT).unwrap();
    let difference = date_time1.signed_duration_since(date_time2);
    if returns_days {
        let mut dates: Vec<NaiveDate> = Vec::new();
        let mut date = date_time1;
        while date <= date_time2 {
            dates.push(date.date_naive());
            date = date + chrono::Duration::days(1);
        }

        let filtered_dates = dates
            .into_iter()
            .filter(|date| {
                let day = date.weekday();
                day != Weekday::Sat && day != Weekday::Sun
            })
            .collect::<Vec<NaiveDate>>();

        return (filtered_dates.len() as i64, Some(filtered_dates));
    }
    (difference.num_days(), None)
}


pub fn return_trading_start_end_time() -> (NaiveTime, NaiveTime) {
    let start_time = NaiveTime::from_hms(9, 15, 0);
    let end_time = NaiveTime::from_hms(15, 25, 0);
    (start_time, end_time)
}

// }
