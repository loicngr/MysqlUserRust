#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub active: bool
}

pub struct Mysql {
    db_name: String,
    host_name: String,
    login: String,
    password: String,
    table: String
}

impl Mysql {
    pub fn new( db_name: String, host_name: String, login: String, password: String, table: String ) -> Mysql {
        Mysql {
            db_name,
            host_name,
            login,
            password,
            table
        }
    }

    pub fn create_pool_conn( &self ) -> Result<my::Pool, my::Error> {
        let url = format!("mysql://{}:{}@{}/{}", self.login, self.password, self.host_name, self.db_name);
        let pool = match my::Pool::new(url) {
            Ok(i)  => i,
            Err(e) => return Err(e),
        };
        Ok(pool)
    }

    pub fn read( &self, pool: &my::Pool ) -> Vec<User> {
        let sql = format!("SELECT first_name, last_name, email, active from {}", self.table);
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

    pub fn create( &self, user: &Vec<User>, pool: &my::Pool ) {
        let sql = format!(r"INSERT INTO {} (first_name, last_name, email, active) VALUES (:first_name, :last_name, :email, :active)", self.table);
        for mut stmp in pool.prepare(sql).into_iter() {
            for p in user.iter() {
                stmp.execute(params!{
                    "first_name"    => &p.first_name,
                    "last_name"     => &p.last_name,
                    "email"         => &p.email,
                    "active"        => &p.active
                }).unwrap();
            }
        }
    }

    // pub fn drop_table( &self ) -> my::Result
}