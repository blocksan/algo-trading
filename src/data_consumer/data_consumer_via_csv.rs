#[path = "../common/mod.rs"] mod common;
use common::read_csv;
use crate::common::raw_stock::RawStock;

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
pub fn read_5_min_data(file_path: &str) -> Result<Vec<RawStock>, Box<dyn std::error::Error>> {
        let mut file = read_csv::read_csv_file(file_path.to_owned()).expect("Not able to read");
    
        let mut stock_array: Vec<f32> = Vec::new();
        let mut stock_5_min_keeper = Vec::new(); 
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
                stock_array[4],
            );
            stock_5_min_keeper.push(stock);
            stock_array = Vec::new();
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
            stock_array[4],
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
            stock_array[4],
        );
        stock_15_min_keeper.push(stock);
        stock_array = Vec::new();
    }
    Ok(stock_15_min_keeper)
}

