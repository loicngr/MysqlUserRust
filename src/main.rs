extern crate mysql;
extern crate dotenv;

use mysql as my;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut mysql_db_name = String::new();
    let mut mysql_host = String::from("localhost:3306");
    let mut mysql_login = String::new();
    let mut mysql_pass = String::new();
    let mut mysql_table = String::new();

    for (key, value) in env::vars() {
        if key == "MYSQL_DB_NAME" { mysql_db_name = String::from(value) }
        else if key == "MYSQL_HOST" { mysql_host = String::from(value) }
        else if key == "MYSQL_LOGIN" { mysql_login = String::from(value) }
        else if key == "MYSQL_PASS" { mysql_pass = String::from(value) }
        else if key == "MYSQL_TABLE" { mysql_table = String::from(value) }
    }

    let pool = my::create_pool_conn( mysql_db_name, mysql_host, mysql_login, mysql_pass );

    let users: Vec<my::User> = my::read( pool, mysql_table );

    println!("{:?}", users);
}