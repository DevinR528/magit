use rocket::{post, State};

use crate::{api::GitHubEvent, Store};

#[post("/webhook", data = "<event>")]
pub fn receive(event: GitHubEvent, to_matrix: &State<Store>) -> Result<(), ()> {
    println!("{:?}", event);
    Ok(())
}

#[post("/", data = "<event>")]
pub fn test(event: GitHubEvent, to_matrix: &State<Store>) -> Result<(), ()> {
    println!("{:?}", event);
    Ok(())
}
