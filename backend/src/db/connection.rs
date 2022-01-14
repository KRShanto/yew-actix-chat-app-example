use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url_not_set_err_msg = "The environment variable `DATABASE_URL` have to save in your .env file. Make sure you have a env variable like this -> \nDATABASE_URL=postgres://username:password@localhost:port/database_name
";

    let database_url = env::var("DATABASE_URL").expect(database_url_not_set_err_msg);

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
