use chrono::prelude::*;

pub const BITS_IN_BYTE: u32 = 8;
pub const MILLS_IN_ONE_SECOND: u32 = 1000;
pub const INTERVAL_OF_BROADCAST_IN_MILLS: u32 = 50;// I have set it as 50 mills; can by any number from 1 

pub fn get_bitrate(bytes_len: u64, mills: i64) -> f64 {
	let bits_in_byte = BITS_IN_BYTE as u64;
	let mills_in_one_second = MILLS_IN_ONE_SECOND as u64;

	(bytes_len * bits_in_byte * mills_in_one_second) as f64 / mills as f64
	// casts to f64
}

pub fn get_current_time_in_mills() -> i64  {
    let now = Utc::now();
    let seconds: i64 = Utc::now().timestamp();
    let nanoseconds: i64 = now.nanosecond() as i64;
    (seconds * 1000) + (nanoseconds / 1000 / 1000)
}

pub fn get_current_time_in_seconds() -> i64 {
	Utc::now().timestamp() as i64
}