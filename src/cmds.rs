use ruma::{
    api::{
        self as ruma_api,
        client::Error,
        error,
        error::MatrixError,
        exports::{bytes, http, percent_encoding},
        ruma_api, AuthScheme, Metadata, SendAccessToken,
    },
    serde::{json_to_buf, Raw},
    EventId, Outgoing,
};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata: {
        description: "Send a state event to a room associated with a given state key.",
        method: PUT,
        name: "send_state_event",
        path: "users/:user/repos",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        pub field: &'a str,
        pub option: Option<&'a str>,
    }

    response: {
        /// A unique identifier for the event.
        pub event_id: EventId,
    }

    error: Error
}

// #[derive(Clone, Debug, Serialize)]
// pub struct Request<'a> {
//     pub field: &'a str,
//     pub option: Option<&'a str>,
// }

// impl<'a> ruma_api::OutgoingRequest for Request<'a> {
//     type EndpointError = Error;
//     type IncomingResponse = Response;

//     const METADATA: Metadata = METADATA;

//     fn try_into_http_request<T: Default + bytes::BufMut>(
//         self,
//         base_url: &str,
//         access_token: SendAccessToken<'_>,
//     ) -> Result<http::Request<T>, error::IntoHttpError> {
//         use std::borrow::Cow;

//         use http::header::{self, HeaderValue};
//         use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

//         let mut url = format!(
//             "{}/path/{}",
//             base_url.strip_suffix('/').unwrap_or(base_url),
//             utf8_percent_encode(self.field, NON_ALPHANUMERIC),
//         );

//         // Last URL segment is optional, that is why this trait impl is not generated.
//         if !self.option.is_none() {
//             url.push('/');
//             url.push_str(&Cow::from(utf8_percent_encode(
//                 self.option.unwrap(),
//                 NON_ALPHANUMERIC,
//             )));
//         }

//         let http_request = http::Request::builder()
//             .method(http::Method::PUT)
//             .uri(url)
//             .header(header::CONTENT_TYPE, "application/json")
//             .header(
//                 header::AUTHORIZATION,
//                 HeaderValue::from_str(&format!(
//                     "Bearer {}",
//                     access_token
//                         .get_required_for_endpoint()
//                         .ok_or(error::IntoHttpError::NeedsAuthentication)?
//                 ))?,
//             )
//             .body(json_to_buf("")?)?;

//         Ok(http_request)
//     }
// }

#[tokio::test]
async fn commands() {
    use ruma::{
        api::SendAccessToken,
        client::{http_client::Reqwest, Client, HttpClientExt},
    };

    let homeserver_url = "foo.server";
    // create a new Client with the given homeserver url and config
    let client = Client::<Reqwest>::new(homeserver_url.to_owned(), None);

    client
        .send_matrix_request(
            "different.server",
            SendAccessToken::IfRequired("github token"),
            Request { field: "", option: None },
        )
        .await
        .unwrap();
}
