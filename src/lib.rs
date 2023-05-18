#![allow(non_snake_case)]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[no_mangle]
pub extern "C" fn getNowMicroseconds() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::default());
    let nanos = since_the_epoch.subsec_nanos() as u64;
    let micros = (1000*1000*1000 * since_the_epoch.as_secs() + nanos)/(1000);
    return micros;
}

#[test]
fn it_works() {
    for i in 0..100 {
        println!("{} microsec({})", getNowMicroseconds(), i);
    }
}
