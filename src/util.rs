pub type SimpleResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn ceil(value: f64, scale: u8) -> f64 {
    let multiplier = 10i64.pow(u32::from(scale)) as f64;
    (value * multiplier).ceil() / multiplier
}
