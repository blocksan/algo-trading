#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShootingStarPattern {
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
    pub is_green_candle: bool,
    pub is_shooting_star: bool,
    pub body_size_ratio: f32,
    pub identified_at: String,
}
impl ShootingStarPattern {
    pub fn new(date: String, open: f32, high: f32, low: f32, close: f32, volume: f32, is_green_candle: bool, is_shooting_star: bool, body_size_ratio: f32) -> ShootingStarPattern {
        ShootingStarPattern {
            date,
            open,
            high,
            low,
            close,
            volume,
            is_green_candle,
            is_shooting_star,
            body_size_ratio
        }
    }

    pub fn update_shooting_star(&mut self, is_shooting_star: bool) {
        self.is_shooting_star = is_shooting_star;
    }

    pub fn update_body_size_ratio(&mut self, body_size_ratio: f32) {
        self.body_size_ratio = body_size_ratio;
    }

    pub fn update_is_green_candle(&mut self, is_green_candle: bool) {
        self.is_green_candle = is_green_candle;
    }

}
pub fn tick_receiver() {

    let shooting_star_pattern_ledger = Vec::new();

    let (is_shooting_star_candle, calculated_body_size_ratio, is_green_candle) = calculate_candle_metadata(stock.open, stock.high, stock.low, stock.close);

    if is_shooting_star_candle {
        let mut shooting_star_pattern = ShootingStarPattern::new(stock.date, stock.open, stock.high, stock.low, stock.close, stock.volume, is_green_candle, is_shooting_star_candle, calculated_body_size_ratio);
        // shooting_star_pattern.update_shooting_star(is_shooting_star_candle);
        // shooting_star_pattern.update_body_size_ratio(calculated_body_size_ratio);
        // shooting_star_pattern.update_is_green_candle(is_green_candle);
        // stock_5_min_keeper.push(shooting_star_pattern);
    }

    fn add_into_shooting_star_pattern_ledger(candle: ShootingStarPattern) -> () {
        shooting_star_pattern_ledger.push(candle);
    }

    pub fn fetch_shooting_star_pattern_ledger() -> Vec<ShootingStarPattern> {
        shooting_star_pattern_ledger
    }

    fn calculate_candle_metadata(open: f32, high: f32, low: f32, close: f32) -> (bool, bool, f32, bool) {
        let calculated_body_size_ratio:f32 = candle_body_size(open, close);
        let is_shooting_star_candle = calculate_shooting_star_candle(calculated_body_size_ratio, open, high, low, close);
        let is_green_candle = calculate_if_green_candle(open, close);

        (is_shooting_star_candle, calculated_body_size_ratio, is_green_candle)
        
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
}
