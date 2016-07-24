extern crate iron;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

fn main() {
   Iron::new(|_:&mut Request|{

	let content_type="text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, get_answer_text())))

   }).http("localhost:3000").unwrap();
}

fn get_answer_text() -> String
{
    String::from("Hail Torvalds!\n")
}
