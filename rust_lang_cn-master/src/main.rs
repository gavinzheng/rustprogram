extern crate hyper;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbsi;
extern crate rustc_serialize;
extern crate persistent;
extern crate urlencoded;
extern crate traitobject;
#[macro_use]
extern crate mime;
extern crate mysql;
extern crate crypto;
extern crate rand;
extern crate iron_login;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate time;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate pulldown_cmark;
extern crate regex;
extern crate ammonia;
extern crate rss;
extern crate cookie;
extern crate oven;
extern crate url;
extern crate mount;
extern crate staticfile;
extern crate form_checker;

mod base;
mod handlers;
mod route;

use iron::Chain;
use hbsi::{HandlebarsEngine, DirectorySource};
use persistent::Read;
use base::config::Config;
use base::db::MyPool;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

fn main() {
    // init logging
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let mut chain = Chain::new(route::gen_router());
    let config = Config::new();
    chain.link_before(Read::<Config>::one(config.clone()));

    let my_pool = MyPool::new(&config);
    chain.link_before(Read::<MyPool>::one(my_pool));

    let cookie_sign_key = config.get("cookie_sign_key").as_str().unwrap().as_bytes().to_owned();
    chain.link_around(iron_login::LoginManager::new(cookie_sign_key));

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    hbse.reload().unwrap();
    chain.link_after(hbse);

    let mut mount = Mount::new();
    mount.mount("/", chain);
    mount.mount("/static/", Static::new(Path::new("static")));

    let listen = config.get("listen").as_str().unwrap();
    iron::Iron::new(mount).http(listen).unwrap();
}
