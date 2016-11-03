use bson::{Document, Bson};

pub struct Reading {
  pub temp:  Option<f64>,
  pub humid: Option<f64>,
  pub lat:   Option<f64>,
  pub long:  Option<f64>
}

impl Reading {
  pub fn from_result(result: &Document) -> Reading {
    let temp  = match result.get_f64("temp") { Ok(val) => { Some(val) }, Err(_) => None };
    let humid = match result.get_f64("humid") { Ok(val) => { Some(val) }, Err(_) => None };
    let lat   = match result.get_f64("lat") { Ok(val) => { Some(val) }, Err(_) => None };
    let long  = match result.get_f64("long") { Ok(val) => { Some(val) }, Err(_) => None };

    Reading { temp: temp, humid: humid, lat: lat, long: long }
  }
}
