#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

pub mod db;
pub mod reading;
