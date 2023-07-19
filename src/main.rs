mod read_csv;
mod stock;
mod market;
mod trade;

use crate::stock::stock_5_min::Stock5Min;
use crate::market::current_market_state::CurrentMarketState;
use crate::trade::trade::Trade;


fn main() {

    let mut file = read_csv::csv_reader::read_csv_file().expect("Not able to read");

    let mut stock_array: Vec<f32> = Vec::new();
    let mut stock_5_min_keeper = Vec::new(); 

    for result in file.records() {
        let record = result.expect("Not able to read");
        let date: String = record[0].parse().unwrap();
        // Print the values of each column in the record
        for (index, column) in record.iter().enumerate() {
            if index == 0 {
                continue;
            }
            stock_array.push(column.parse::<f32>().unwrap());
        }
        let stock = Stock5Min::new(date,stock_array[0], stock_array[1], stock_array[2], stock_array[3], stock_array[4], false, false, false, 0.0);
        stock_5_min_keeper.push(stock);
        stock_array = Vec::new();
    }
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
    for stock in stock_5_min_keeper.iter_mut() {
        // println!("Date => {:?}", stock.date);
        // println!("Candle Metadata START => {:?}", stock.date);
        let (is_hammer, is_shooting_star, body_size_ratio, is_green_candle) = calculate_candle_metadata(stock.open, stock.high, stock.low, stock.close);
        if is_hammer {
            // println!("Hammer star found");
            stock.update_hammer(is_hammer);
            // println!("{:?}", stock);
        }

        if is_shooting_star {
            // println!("Shooting star found");
            stock.update_shooting_star(is_shooting_star);
            // println!("{:?}", stock);
        }

        stock.update_body_size_ratio(body_size_ratio);
        stock.update_is_green_candle(is_green_candle);

        // println!("Candle Metadata END");
    }

    println!("Stock 5 min keeper => {:?}", stock_5_min_keeper[230]);

    fn calculate_candle_metadata(open: f32, high: f32, low: f32, close: f32) -> (bool, bool, f32, bool) {
        let calculated_body_size_ratio:f32 = candle_body_size(open, close);
        let is_hammer_candle = calculate_hammer_candle(calculated_body_size_ratio, open, high, low, close);
        let is_shooting_star_candle = calculate_shooting_star_candle(calculated_body_size_ratio, open, high, low, close);
        let is_green_candle = calculate_if_green_candle(open, close);

        (is_hammer_candle, is_shooting_star_candle, calculated_body_size_ratio, is_green_candle)
        
    }

    fn calculate_hammer_candle(calculated_body_size_ratio:f32, open: f32, high: f32, low: f32, close: f32) -> bool {
        let lower_wick = if open > close { close - low } else { open - low };
        let upper_wick = if open > close { high - open } else { high - close };
    
        let lower_wick_to_body_ratio = lower_wick / calculated_body_size_ratio;
        let upper_wick_to_body_ratio = upper_wick / calculated_body_size_ratio;
        
        let full_candle_height = high - low;
        let body_to_full_candle_ratio = calculated_body_size_ratio / full_candle_height;

        // print!("lower_wick_to_body_ratio => {:?} ", lower_wick_to_body_ratio);
        // print!("upper_wick_to_body_ratio => {:?} ", upper_wick_to_body_ratio);
        // println!("calculated_body_size_ratio => {:?}", calculated_body_size_ratio);
        // println!("lower_wick => {:?}", lower_wick);
        // println!("upper_wick => {:?}", upper_wick);
        // println!("full_candle_height => {:?}", full_candle_height);
        // println!("body_to_full_candle_ratio => {:?}", body_to_full_candle_ratio);
        if body_to_full_candle_ratio <= 0.25 {
            lower_wick > (2.1 * upper_wick)
        }else {
            lower_wick_to_body_ratio >= 1.75 && upper_wick_to_body_ratio <= 1.5 && (lower_wick > (2.0 * upper_wick))
        }
    }

    fn calculate_shooting_star_candle(calculated_body_size_ratio:f32,open: f32, high: f32, low: f32, close: f32) -> bool {
        let lower_wick = if open > close { close - low } else { open - low };
        let upper_wick = if open > close { high - open } else { high - close };
    
        let lower_wick_to_body_ratio = lower_wick / calculated_body_size_ratio;
        let upper_wick_to_body_ratio = upper_wick / calculated_body_size_ratio;
        
        let full_candle_height = high - low;
        let body_to_full_candle_ratio = calculated_body_size_ratio / full_candle_height;

        // print!("lower_wick_to_body_ratio => {:?} ", lower_wick_to_body_ratio);
        // print!("upper_wick_to_body_ratio => {:?} ", upper_wick_to_body_ratio);
        // println!("calculated_body_size_ratio => {:?}", calculated_body_size_ratio);
        // println!("lower_wick => {:?}", lower_wick);
        // println!("upper_wick => {:?}", upper_wick);
        // println!("full_candle_height => {:?}", full_candle_height);
        // println!("body_to_full_candle_ratio => {:?}", body_to_full_candle_ratio);
        if body_to_full_candle_ratio <= 0.25 {
            upper_wick > (2.1 * lower_wick)
        }else {
            upper_wick_to_body_ratio >= 1.75 && lower_wick_to_body_ratio <= 1.5 && (upper_wick > (2.0 * lower_wick))
        }
    }

    fn candle_body_size(open: f32, close: f32) -> f32 {
        (open - close).abs()
    }

    fn calculate_if_green_candle(open: f32, close: f32) -> bool {
        open < close
    }
    
}   
