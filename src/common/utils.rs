pub fn symbol_algo_type_formatter(symbol: &str, algo_type: &str) -> String {
    format!("{}-{}", symbol, algo_type.to_string())
}