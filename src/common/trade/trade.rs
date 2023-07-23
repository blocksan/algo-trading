enum TradeType {
    Long,
    Short,
}

enum TradeEntryReason {
    Hammer,
    ShootingStar
}

pub struct Trade {
    trade_type: TradeType,
    trade_entry_reason: TradeEntryReason,
    entry_price: f32,
    exit_price: f32,
    profit: f32,
    is_trade_open: bool,
    qty: i32,
    total_price: f32,
    trade_taken_at: String,
    trade_closed_at: String,
}

impl Trade {
    fn new(trade_type: TradeType, trade_entry_reason: TradeEntryReason, entry_price: f32, exit_price: f32, profit: f32, is_trade_open: bool, qty:i32, total_price:f32, trade_taken_at: String) -> Trade {
        Trade {
            trade_type,
            trade_entry_reason,
            entry_price,
            exit_price,
            profit,
            is_trade_open,
            qty,
            total_price,
            trade_taken_at,
            trade_closed_at: "".to_string(),
        }
    }

    fn update_trade_metadata(&mut self, exit_price: f32, profit: f32, is_trade_open: bool, trade_closed_at: String) {
        self.exit_price = exit_price;
        self.profit = profit;
        self.is_trade_open = is_trade_open;
        self.trade_closed_at = trade_closed_at;
    }
}

