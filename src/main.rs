extern crate temp_bot;
use temp_bot::db::ReadingConn;

fn main() {
  let mut conn = ReadingConn::new("localhost", 27017);
  conn.connect();

  if conn.connected {
    println!("connected!");

    let reading = conn.latest_reading().unwrap();
    println!("temp: {}", reading.temp.unwrap());
  }
}
