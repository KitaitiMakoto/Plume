#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

extern crate activitypub;
extern crate ammonia;
extern crate bcrypt;
extern crate canapi;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate heck;
#[macro_use]
extern crate lazy_static;
extern crate openssl;
extern crate plume_api;
extern crate plume_common;
extern crate reqwest;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate webfinger;

use std::env;

macro_rules! find_by {
    ($table:ident, $fn:ident, $($col:ident as $type:ident),+) => {
        /// Try to find a $table with a given $col
        pub fn $fn(conn: &PgConnection, $($col: $type),+) -> Option<Self> {
            $table::table
                $(.filter($table::$col.eq($col)))+
                .limit(1)
                .load::<Self>(conn)
                .expect("Error loading $table by $col")
                .into_iter().nth(0)
        }
    };
}

macro_rules! list_by {
    ($table:ident, $fn:ident, $($col:ident as $type:ident),+) => {
        /// Try to find a $table with a given $col
        pub fn $fn(conn: &PgConnection, $($col: $type),+) -> Vec<Self> {
            $table::table
                $(.filter($table::$col.eq($col)))+
                .load::<Self>(conn)
                .expect("Error loading $table by $col")
        }
    };
}

macro_rules! get {
    ($table:ident) => {
        pub fn get(conn: &PgConnection, id: i32) -> Option<Self> {
            $table::table.filter($table::id.eq(id))
                .limit(1)
                .load::<Self>(conn)
                .expect("Error loading $table by id")
                .into_iter().nth(0)
        }
    };
}

macro_rules! insert {
    ($table:ident, $from:ident) => {
        pub fn insert(conn: &PgConnection, new: $from) -> Self {
            diesel::insert_into($table::table)
                .values(new)
                .get_result(conn)
                .expect("Error saving new $table")
        }
    };
}

macro_rules! update {
    ($table:ident) => {
        pub fn update(&self, conn: &PgConnection) -> Self {
            diesel::update(self)
                .set(self)
                .get_result(conn)
                .expect(concat!("Error updating ", stringify!($table)))
        }
    };
}

lazy_static! {
    pub static ref BASE_URL: String = env::var("BASE_URL")
        .unwrap_or(format!("127.0.0.1:{}", env::var("ROCKET_PORT").unwrap_or(String::from("8000"))));

    pub static ref DB_URL: String = env::var("DB_URL")
        .unwrap_or(format!("postgres://plume:plume@localhost/{}", env::var("DB_NAME").unwrap_or(String::from("plume"))));

    pub static ref USE_HTTPS: bool = env::var("USE_HTTPS").map(|val| val == "1").unwrap_or(true);
}

pub fn ap_url(url: String) -> String {
    let scheme = if *USE_HTTPS {
        "https"
    } else {
        "http"
    };
    format!("{}://{}", scheme, url)
}

pub mod admin;
pub mod blog_authors;
pub mod blogs;
pub mod comments;
pub mod db_conn;
pub mod follows;
pub mod instance;
pub mod likes;
pub mod medias;
pub mod mentions;
pub mod notifications;
pub mod post_authors;
pub mod posts;
pub mod reshares;
pub mod safe_string;
pub mod schema;
pub mod tags;
pub mod users;
