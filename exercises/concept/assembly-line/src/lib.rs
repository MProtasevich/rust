// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let produced_cars = speed as u64 * 221u64;

    let success_rate = match speed {
        0..=4  => 1.0,
        5..=8  => 0.9,
        9..=10 => 0.77,
        _ => unreachable!("Speed can't be higher than 10. But speed = {}", speed)
    };

    produced_cars as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
