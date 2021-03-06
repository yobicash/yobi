#![recursion_limit="1024"]

#![feature(custom_attribute)]
extern crate libyobicash;
#[macro_use]
extern crate error_chain;
extern crate bytes;
extern crate futures;
extern crate futures_await;
extern crate unqlite;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod errors;
pub mod version;
pub mod store;
pub mod models;
pub mod config;
pub mod info;
pub mod api;
pub mod network;
pub mod cli;

pub const VERSION: &str = "0.1.0";
