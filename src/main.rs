extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate yup_oauth2;
extern crate google_testing1;
extern crate env_logger;

use std::path::Path;
use std::default::Default;

use google_testing1::TestMatrix;
use futures::lazy;
use hyper::rt;
use hyper::Client;
use hyper::rt::Future;
use yup_oauth2::{
    Authenticator, DefaultAuthenticatorDelegate,
    ApplicationSecret, MemoryStorage, ServiceAccountAccess,
};
use yup_oauth2::{service_account_key_from_file, read_application_secret};
use google_testing1::Testing;

fn doit() -> impl Future<Item=(), Error=()> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things.
    println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    let secret: ApplicationSecret = Default::default();
    let secret = service_account_key_from_file(Path::new("/path/to/service-account.json")).unwrap();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut access = ServiceAccountAccess::new(secret, client);
    println!("!AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    use yup_oauth2::GetToken;
    println!("{:?}",
access.token(&vec!["https://www.googleapis.com/auth/spanner.data"]).unwrap());
    /*
    let auth = Authenticator::new(
        &secret, DefaultAuthenticatorDelegate,
        client, <MemoryStorage as Default>::default(), None);
    */
    let https2 = hyper_tls::HttpsConnector::new(4).unwrap();
    let client2 = Client::builder().build::<_, hyper::Body>(https2);
    //let hub = Testing::new(client2, auth);
    println!("!!AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    let hub = Testing::new(client2, access);
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let req = TestMatrix::default();
    
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    println!("!!!AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    hub.projects().test_matrices_create(req, "projectId")
        .request_id("sed")
        .doit().map(|result| {
            println!("Got result! {:?}", result);
        }).map_err(|err| {
            println!("Got err! {:?}", err);
        })
}

fn main() {
    env_logger::init();
    let f = lazy(|| doit());
    rt::run(f);
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
