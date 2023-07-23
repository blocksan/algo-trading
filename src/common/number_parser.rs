#[allow(dead_code, unused_variables)]
pub fn return_2_precision_for_float(number: f32) -> f32 {
    (number * 100.0).round()/100.0
}