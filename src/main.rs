extern crate temp_bot;
use temp_bot::db::ReadingConn;

fn main() {
  let mut conn = ReadingConn::new("localhost", 27017);
  conn.connect();

  if conn.connected {
    println!("connected!");

    let reading = conn.latest_reading().unwrap();
    println!("The temperature is {}, humidity is {}, taken at {}, {}", reading.temp.unwrap(), reading.humid.unwrap(), reading.lat.unwrap(), reading.long.unwrap());
  }
}
