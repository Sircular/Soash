#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate tantivy;
extern crate base64;
extern crate pickledb;
extern crate rocket_contrib;
extern crate serde;
extern crate uuid;

mod auth;
mod cache;
mod constants;
mod endpoints;
mod search;

use crate::{
    auth::{AuthStore, AuthenticatedUser},
    cache::TtlCache,
    search::NoteStore,
};
use rocket::fairing::AdHoc;
use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub index_dir: String,
    pub auth_store: String,
}

fn main() {
    rocket::ignite()
        .mount("/api/auth", endpoints::auth::routes())
        .mount("/api/note", endpoints::note::routes())
        .attach(AdHoc::on_attach("Config Loader", |rocket| {
            let config = rocket.config();

            let index_dir = config.get_str("index_dir").unwrap_or("./index").to_string();
            let auth_store_path = config
                .get_str("auth_store")
                .unwrap_or("./auth.db")
                .to_string();

            let auth_cache: TtlCache<AuthenticatedUser> =
                TtlCache::new(Duration::new(constants::INDEX_CACHE_EXPIRY, 0));
            let auth_store = AuthStore::new(&auth_store_path);
            let note_store = match NoteStore::new(index_dir) {
                Ok(store) => store,
                Err(e) => {
                    println!("{:?}", e);
                    return Err(rocket)
                },
            };

            Ok(rocket
                .manage(auth_cache)
                .manage(auth_store)
                .manage(note_store))
        }))
        .attach(auth::TokenRefreshFairing {})
        .launch();
}
