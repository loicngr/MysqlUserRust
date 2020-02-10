# MysqlUserRust
Just a simple CRUD rust app

### Env vars (.env)
    MYSQL_DB_NAME=
    MYSQL_HOST=localhost:3306
    MYSQL_LOGIN=
    MYSQL_PASS=
    MYSQL_TABLE=root

### Main file 
- src/main.rs

### Mysql CRUD lib file
- mysql/src/lib.rs

### Database
    id:         type(INT)       AUTO-INCREMENTE
    first_name: type(VARCHAR)   VALUE(20)
    last_name:  type(VARCHAR)   VALUE(20)
    email:      type(VARCHAR)   VALUE(30)
    active:     boolean