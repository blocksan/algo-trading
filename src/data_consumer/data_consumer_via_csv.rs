#[path = "../common/mod.rs"] mod common;
use common::read_csv;
use crate::common::{raw_stock::RawStock, enums::TimeFrame};

    // let temp_stock = Stock5Min {
    //     date: "2021-01-01".to_string(),
    //     open: 402.5,
    //     high: 403.3,
    //     low: 401.6,
    //     close: 403.3,
    //     volume: 100000.0,
    //     is_hammer: false,
    //     is_shooting_star: false,
    // };
    //402.5,403.3,401.6,403.3,128738
    // stock_5_min_keeper.push(temp_stock);
pub fn read_5_min_data(file_path: &str, start_date: &str, end_date: &str, stock_symbol: &str) -> Result<Vec<RawStock>, Box<dyn std::error::Error>> {
        let mut file = read_csv::read_csv_file(file_path.to_owned()).expect("Not able to read");
    
        let mut stock_array: Vec<f32> = Vec::new();
        let mut stock_5_min_keeper = Vec::new(); 
        let file_name_parts = file_path.split("_");
        // for result in file.records() {
        //     let record = result.expect("Not able to read");
        //     let date: String = record[1].parse().unwrap();
        //     // Print the values of each column in the record
        //     for (index, column) in record.iter().enumerate() {
        //         if index == 0 || index == 1 {
        //             continue;
        //         }
        //         stock_array.push(column.parse::<f32>().unwrap());
        //     }
        //     let stock = RawStock::new(
        //         file_name_parts.clone().collect::<Vec<&str>>()[0].to_string(),
        //         date,
        //         stock_array[0],
        //         stock_array[1],
        //         stock_array[2],
        //         stock_array[3],
        //         stock_array[4] as i32,
        //         TimeFrame::FiveMinutes
        //     );
        //     stock_5_min_keeper.push(stock);
        //     stock_array = Vec::new();
        // }

        //current format is 
            // 1) Date - Date of observation
            // 2) Open - Open price of the index on a particular day
            // 3) High - High price of the index on a particular day
            // 4) Low - Low price of the index on a particular day
            // 5) Close - Close price of the index on a particular day
        //@TODO: Change the format to the format used in the previous code
        //write the code to change the format
        for result in file.records() {
            let record = result.expect("Not able to read");
            let date: String = record[0].parse().unwrap();
            if date < start_date.to_string() || date > end_date.to_string() {
                //skip the date
                // println!("skipping the date: {}", date);
                continue;
            }
            let open: f32 = record[1].parse().unwrap();
            let high: f32 = record[2].parse().unwrap();
            let low: f32 = record[3].parse().unwrap();
            let close: f32 = record[4].parse().unwrap();
            let volume: i32 = record[5].parse().unwrap();
            let stock = RawStock::new(
                stock_symbol.to_string(),
                date,
                close,
                high,
                low,
                open,
                volume,
                TimeFrame::FiveMinutes
            );
            stock_5_min_keeper.push(stock);
            // stock_array = Vec::new();
            // println!("stock: {:?}", stock);

        }

        Ok(stock_5_min_keeper)
}

pub fn read_1_min_data(file_path: &str) -> Result<Vec<RawStock>, Box<dyn std::error::Error>> {
    let mut file = read_csv::read_csv_file(file_path.to_owned()).expect("Not able to read");

    let mut stock_array: Vec<f32> = Vec::new();
    let mut stock_1_min_keeper = Vec::new(); 

    let file_name_parts = file_path.split("_");

    for result in file.records() {
        let record = result.expect("Not able to read");
        let date: String = record[1].parse().unwrap();
        // Print the values of each column in the record
        for (index, column) in record.iter().enumerate() {
            if index == 0 || index == 1 {
                continue;
            }
            stock_array.push(column.parse::<f32>().unwrap());
        }
        let stock = RawStock::new(
            file_name_parts.clone().collect::<Vec<&str>>()[0].to_string(),
            date,
            stock_array[0],
            stock_array[1],
            stock_array[2],
            stock_array[3],
            stock_array[4] as i32,
            TimeFrame::OneMinute
        );
        stock_1_min_keeper.push(stock);
        stock_array = Vec::new();
    }
    Ok(stock_1_min_keeper)
}

pub fn read_15_min_data(file_path: &str) -> Result<Vec<RawStock>, Box<dyn std::error::Error>> {
    let mut file = read_csv::read_csv_file(file_path.to_owned()).expect("Not able to read");

    let mut stock_array: Vec<f32> = Vec::new();
    let mut stock_15_min_keeper = Vec::new(); 

    let file_name_parts = file_path.split("_");

    for result in file.records() {
        let record = result.expect("Not able to read");
        let date: String = record[0].parse().unwrap();
        // Print the values of each column in the record
        for (index, column) in record.iter().enumerate() {
            if index == 0 || index == 1 {
                continue;
            }
            stock_array.push(column.parse::<f32>().unwrap());
        }
        let stock = RawStock::new(
            file_name_parts.clone().collect::<Vec<&str>>()[0].to_string(),
            date,
            stock_array[0],
            stock_array[1],
            stock_array[2],
            stock_array[3],
            stock_array[4] as i32,
            TimeFrame::FifteenMinutes,
        );
        stock_15_min_keeper.push(stock);
        stock_array = Vec::new();
    }
    Ok(stock_15_min_keeper)
}

