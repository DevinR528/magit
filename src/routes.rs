use rocket::{post, State};

use crate::{api::GitHubEvent, Store};

#[post("/", data = "<event>")]
pub fn index(event: GitHubEvent, _to_matrix: &State<Store>) -> Result<(), ()> {
    println!("{:?}", event);
    Ok(())
}
