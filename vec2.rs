use std::ops::BitXor;
use std::time::{SystemTime, UNIX_EPOCH};

fn rn4() -> u8 {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let p = *(&t).to_string().as_bytes().last().unwrap();
    (t.bitxor(p as u128) as u8) % 4u8
}

pub fn random_trajectory() -> i8 {
    let mut sign = 1i8;
    if rn4() >= 2 {
        sign = -1;
    }
    sign
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub(crate) x: i32,
    pub(crate) y: i32
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
    pub fn random() -> Self {
        Vec2 {
            x: random_trajectory() as i32,
            y: random_trajectory() as i32,
        }
    }
}