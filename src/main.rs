extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate yup_oauth2;
extern crate google_testing1;
use google_testing1::TestMatrix;
use google_testing1::{Result, Error};
use std::default::Default;
use hyper::rt;
use hyper::Client;
use hyper::rt::Future;
use yup_oauth2::{
    Authenticator, DefaultAuthenticatorDelegate,
    ApplicationSecret, MemoryStorage
};
use google_testing1::Testing;

fn doit() -> impl Future<Item=(), Error=()> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things.
    let secret: ApplicationSecret = Default::default();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let auth = Authenticator::new(
        &secret, DefaultAuthenticatorDelegate,
        client, <MemoryStorage as Default>::default(), None);
    let https2 = hyper_tls::HttpsConnector::new(4).unwrap();
    let client2 = Client::builder().build::<_, hyper::Body>(https2);
    let hub = Testing::new(client2, auth);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let req = TestMatrix::default();
    
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let fut = hub.projects().test_matrices_create(req, "projectId")
        .request_id("sed")
        .doit().map(|result| {
            println!("Got result! {:?}", result);
        }).map_err(|err| {
            println!("Got err! {:?}", err);
        });
    rt::spawn(fut);
    futures::future::ok(())
}

fn main() {
    rt::run(doit());
    /*
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
    */
}