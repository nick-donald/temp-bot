use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use reading::Reading;

pub struct ReadingConn {
  host: &'static str,
  port: u16,
  client: Option<Client>,
  pub connected: bool
}

impl ReadingConn {
  pub fn new(host: &'static str, port: u16) -> ReadingConn {
    ReadingConn { host: host, port: port, connected: false, client: None }
  }

  pub fn connect(&mut self) {
    let client = Client::connect(self.host, self.port);

    match client {
      Ok(cl) => {
        self.client = Some(cl);
        self.connected = true;
      },
      Err(e) => panic!("Could not connect: {}", e)
    }
  }

  pub fn latest_reading(self) -> Option<Reading> {
    let coll = self.client.unwrap().db("test").collection("readings");
    let resp = coll.find_one(None, None).ok().expect("No cursor");

    match resp {
      Some(result) => {
        let reading = Reading::from_result(&result);
        Some(reading)
      },
      None => {
        None
      }
    }
  }
}
