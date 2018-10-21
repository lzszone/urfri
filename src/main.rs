extern crate actix_web;
extern crate urfri;
extern crate diesel;

use self::urfri::*;
use utils::crypto::Secret;
use self::models::*;
use self::diesel::prelude::*;
use actix_web::{server, App, HttpRequest};

// fn index(_req: &HttpRequest) -> String {
//     use urfri::schema::posts::dsl::*;

//     let connection = establish_connection();
//     let results = posts
//         .filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(&connection)
//         .expect("Error loading posts");

//     results[0].body.to_owned()
// }

fn main() {
    

    // server::new(|| App::new().resource("/", |r| r.f(index)))
    //     .bind("127.0.0.1:8088")
    //     .unwrap()
    //     .run();
}

#[test]
fn run() {
    let username = "lzszone";
    let wp = "wrong password";
    let rp = "right password";
    let secret = Secret::new(&username, &rp).expect("err");

    assert!(secret.verify(&wp).is_err());

    assert!(secret.verify(&rp).is_ok());
}