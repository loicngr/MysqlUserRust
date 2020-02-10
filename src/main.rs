extern crate crud;
extern crate dotenv;

use std::io::Result;
use dotenv::dotenv;
use std::env;
use crud as mysql;

fn main() -> Result<()> {
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

    let mysql_controller = mysql::Mysql::new( mysql_db_name, mysql_host, mysql_login, mysql_pass, mysql_table );
    let pool = match mysql_controller.create_pool_conn() {
        Ok(i) => i,
        Err(err) => panic!("{}", err),
    };

    match mysql_controller.trunc_table( &pool ) {
        Ok(_) => println!("Table was truncated !"),
        Err(_) => {
            match mysql_controller.create_table( &pool ) {
                Ok(_) => println!("Table Created !"),
                Err(err) => panic!("{}", err),
            };
        },
    };

    let new_users = vec![
        mysql::User { first_name: String::from("loic"), last_name: String::from("NOGIER"), email: String::from("test@email.fr"), active: false },
        mysql::User { first_name: String::from("paul"), last_name: String::from("BONS"), email: String::from("test2@email.fr"), active: false }
    ];
    mysql_controller.create( &new_users, &pool );

    let users: Vec<mysql::User> = mysql_controller.read( &pool );
    println!("{:?}", users);

    Ok(())
}