use chrono::{
    DateTime,
    offset::{Utc, Local},
};

pub fn get_utc_time() -> DateTime<Utc> {
    Utc::now()
}

pub fn get_local_time() -> DateTime<Local> {
    Local::now()
}