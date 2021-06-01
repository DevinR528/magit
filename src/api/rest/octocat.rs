use std::fmt;

use github_derive::github_rest_api;
use ruma::UInt;
use serde::{Deserialize, Deserializer};

use crate::api::rest::ApplicationV3Json;

github_rest_api! {
    metadata: {
        description: "ASCII art Octocat with speech bubble",
        method: GET,
        path: "/octocat",
        name: "octocat",
        authentication: false,
    }

    request: {
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        #[github(query)]
        pub s: &'a str,
    }

    // Uggg this is dumb!
    // TODO: use serde somehow and remove the DeserAttr::ForwardToBody hack
    // github returns a string, not valid JSON
    #[github(forward_to_body = cat)]
    response: {
        pub cat: String,
    }
}
