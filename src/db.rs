use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{FindOptions, CursorType};
use mongodb::cursor;

use reading::Reading;

const ORDER_DESC: i32 = -1;

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
    let opts = FindOptions {
      sort: Some(doc! { "_id" => ORDER_DESC }),
      allow_partial_results: false,
      no_cursor_timeout: false,
      op_log_replay: false,
      skip: 0,
      limit: 0,
      cursor_type: CursorType::NonTailable,
      batch_size: cursor::DEFAULT_BATCH_SIZE,
      comment: None,
      max_time_ms: None,
      modifiers: None,
      projection: None,
      read_preference: None
    };

    let resp = coll.find_one(None, Some(opts)).ok().expect("No cursor");

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
