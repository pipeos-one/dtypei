// type SimpleCollection = HashMap<Vec<u8>, Vec<u8>>;
// type Records = Arc<RwLock<SimpleCollection>>;

type Records = Vec<u8>;

#[derive(Debug)]
pub struct SimpleDB {
  pub records: Records
}

impl SimpleDB {
    pub fn new() -> SimpleDB {
        SimpleDB {
            records: Records::new(),
        }
    }
}

// fn main() {
//     println!("Hello, world!");
//     let mut db = SimpleDB::new();
//     db.records.push(8);
//     println!("Records: {:?}", db.records);
// }
