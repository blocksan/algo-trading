use std::{thread, sync::{Arc, mpsc}};

use crate::common::raw_stock::RawStock;


pub fn find_support_resistance(stocks: &Vec<RawStock>, pivot_depth: usize) -> (Vec<f32>, Vec<f32>){
    let mut support = vec![];
    let mut resistance = vec![];
    let arc_stocks = Arc::new(stocks.clone());
    for i in 0..stocks.len() {
        let local_minima_stocks_shared = Arc::clone(&arc_stocks);
        let local_maxima_stocks_shared = Arc::clone(&arc_stocks);

        if i < pivot_depth*2-1 {
            continue;
        }
        let current_pivot_index = i-pivot_depth;
        let current_stock_1 = stocks[current_pivot_index].clone();
        let current_stock_2 = stocks[current_pivot_index].clone();
        let (minima_sender, minima_receiver) = mpsc::channel();
        let (maxima_sender, maxima_receiver) = mpsc::channel();
        
        let local_minima_thread = thread::spawn(move || {
            let result = is_local_minima(pivot_depth, current_pivot_index,  local_minima_stocks_shared, current_stock_1);
            minima_sender.send(result).unwrap();
            
        });

        let local_maxima_thread = thread::spawn(move || {
            let result = is_local_maxima(pivot_depth,current_pivot_index, local_maxima_stocks_shared,current_stock_2);
            maxima_sender.send(result).unwrap();
        });
        local_minima_thread.join().unwrap();
        local_maxima_thread.join().unwrap();

        let current_stock = stocks[current_pivot_index].clone();
        if minima_receiver.recv().unwrap() {
            println!("minima stock => {:?} value {:?}",current_stock.low, current_stock.date);
            support.push(current_stock.low);
        }
        if maxima_receiver.recv().unwrap() {
            println!("maxima stock => {:?} value {:?}",current_stock.high, current_stock.date);
            resistance.push(current_stock.high);
        }
        // if is_local_minima(pivot_depth, current_pivot_index,  &stocks, &current_stock) {
        //     println!("minima stock => {:?} value {:?}",current_stock.low, current_stock.date);

        //     support.push(current_stock.low);
        // }
        // if is_local_maxima(pivot_depth,current_pivot_index, &stocks, &current_stock) {
        //     println!("maxima stock => {:?} value {:?}",current_stock.high, current_stock.date);

        //     resistance.push(current_stock.high);
        // }
    }
    
    (support, resistance)
}

fn is_local_minima(pivot_depth: usize, pivot_index: usize, stocks: Arc<Vec<RawStock>>, current_stock: RawStock) -> bool {
    let mut i = 1;
    let mut is_minima = true;
    while i < pivot_depth {
        if current_stock.low > stocks[pivot_index-i].low || current_stock.low > stocks[pivot_index+i].low {
            is_minima = false;
            break;
        }
        i += 1;
    }
    is_minima
}

fn is_local_maxima(pivot_depth: usize, pivot_index: usize, stocks:Arc<Vec<RawStock>>, current_stock: RawStock) -> bool {
    let mut i = 1;
    let mut is_maxima = true;
    while i < pivot_depth {
        if current_stock.high < stocks[pivot_index+i].high || current_stock.high < stocks[pivot_index-i].high {
            is_maxima = false;
            break;
        }
        i += 1;
    }
    is_maxima
}