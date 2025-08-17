use std::fs;

mod tile;
mod db_funcs;


fn load_init_sql() -> std::io::Result<String> {
    fs::read_to_string("assets/init.sql")
}

fn main() {
    env_logger::init();

    let init_query = load_init_sql().expect("can load unit query");
    let db_con = sqlite::open(":memory:").expect("can create sqlite database");
    db_con
        .execute(init_query)
        .expect("can initialise sqlite db");
}
