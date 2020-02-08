#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug)]
pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    active: bool
}

pub fn create_pool_conn( mysql_db_name: String, mysql_host: String, mysql_login: String, mysql_pass: String ) -> my::Pool {
    let url = format!("mysql://{}:{}@{}/{}", mysql_login, mysql_pass, mysql_host, mysql_db_name);
    my::Pool::new(url).unwrap()
}

/// Just read Data form specified table
pub fn read( pool: my::Pool, mysql_table: String ) -> Vec<User> {
    let sql = format!("SELECT first_name, last_name, email, active from {}", mysql_table);
    pool.prep_exec(sql, ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (first_name, last_name, email, active) = my::from_row(row);
            User {
                first_name: first_name,
                last_name: last_name,
                email: email,
                active: active,
            }
        }).collect()
    }).unwrap()
}