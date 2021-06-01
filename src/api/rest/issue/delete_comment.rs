use github_derive::github_rest_api;
use ruma::UInt;

use crate::api::{
    rest::{ApplicationV3Json, Type},
    IncomingComment,
};

github_rest_api! {
    metadata: {
        description: "Delete a comment by comment Id",
        method: DELETE,
        path: "/repos/:owner/:repo/issues/comments/:comment_id",
        name: "delete_issue_comment",
        authentication: true,
    }

    request: {
        /// Optional accept header to enable preview features.
        #[github(header = ACCEPT)]
        pub accept: Option<ApplicationV3Json>,

        /// The owner of this repository.
        #[github(path)]
        pub owner: &'a str,

        /// The name of this repository.
        #[github(path)]
        pub repo: &'a str,

        /// The title of the new issue.
        #[github(path)]
        pub comment_id: UInt,
    }

    /// An empty response with a status of 204.
    response: { }
}
