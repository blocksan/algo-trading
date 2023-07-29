use chrono::prelude::*;

// pub fn date_util() {
    const DESIRED_STOCK_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%Z";
    const FILE_STOCK_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";
    
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

// }
