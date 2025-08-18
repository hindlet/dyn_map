use std::fs;

use sqlite::Connection;

pub mod tile_funcs;
pub mod player_funcs;






pub fn init_database() -> Connection {
    let init_query = fs::read_to_string("assets/init.sql").expect("can load unit query");
    let db_con = sqlite::open("assets/test.db").expect("can create sqlite database");
    // db_con
    //     .execute(init_query)
    //     .expect("can initialise sqlite db");
    db_con
}