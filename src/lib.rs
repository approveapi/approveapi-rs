extern crate approveapi_openapi;
extern crate tokio;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use approveapi_openapi::hyper::Client;
use approveapi_openapi::hyper_tls::HttpsConnector;
use approveapi_openapi::apis;
use approveapi_openapi::apis::Error;

pub use approveapi_openapi::apis::ApproveApiClient;
pub use approveapi_openapi::models::*;
pub use approveapi_openapi::apis::ApproveApi;


use futures::prelude::*;

lazy_static! {
    //  Credit to rusoto_core for this technique
    static ref FALLBACK_RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

pub trait ApproveApiSync {
    type F: IntoFuture;
    fn sync(self) -> Result<<<Self as ApproveApiSync>::F as IntoFuture>::Item, <<Self as ApproveApiSync>::F as IntoFuture>::Error>;
}

impl <I: Send, T: 'static + IntoFuture<Item = I, Error = Error<serde_json::value::Value>> + Send> ApproveApiSync for T 
where 
    <T as IntoFuture>::Future : Send 
{
    type F = T;
    fn sync(self) -> Result<<T as IntoFuture>::Item, <T as IntoFuture>::Error> {
        futures::sync::oneshot::spawn(self.into_future(), &FALLBACK_RUNTIME.executor()).wait()
    }
}

pub fn create_client<S: Into<String>>(api_key: S) -> ApproveApiClient<approveapi_openapi::hyper_tls::HttpsConnector<approveapi_openapi::hyper::client::HttpConnector>> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let hyper_client = Client::builder().build::<_, hyper::Body>(https);

    let mut config = apis::configuration::Configuration::new(hyper_client);
    config.basic_auth = Some((api_key.into(), None));
    ApproveApiClient::new(std::sync::Arc::new(config))
}

#[test]
fn sync_api_compiles() {
    println!("{:?}", create_client("123").get_prompt("123", false).sync());
}
