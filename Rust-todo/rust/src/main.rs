use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;
use chrono::{Utc, DateTime};


fn main(){
    println!("Hello, world!");
}