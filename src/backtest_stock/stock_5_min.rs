#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stock5Min {
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: i32,
    pub is_green_candle: bool,
    pub is_hammer: bool,
    pub is_shooting_star: bool,
    pub body_size_ratio: f32,
}
impl Stock5Min {
    pub fn new(date: String, open: f32, high: f32, low: f32, close: f32, volume: i32, is_green_candle: bool, is_hammer: bool, is_shooting_star: bool, body_size_ratio: f32) -> Stock5Min {
        Stock5Min {
            date,
            open,
            high,
            low,
            close,
            volume,
            is_green_candle,
            is_hammer,
            is_shooting_star,
            body_size_ratio
        }
    }

    pub fn update_hammer(&mut self, is_hammer: bool) {
        self.is_hammer = is_hammer;
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

