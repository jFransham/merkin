use semver::{ Version, VersionReq };
use iron::prelude::*;
use iron::error::HttpResult;
use iron::status;
use hyper::{ Client, Url };
use hyper::server::Listening;
use hyper::method::Method;
use serde::ser::Serialize;
use serde::de::Deserialize;
use serde_json;
use std::io::Read;
use std::mem;

pub type ServiceResult<T> = Result<T, status::Status>;

pub trait ExternalService {
    fn is_compatible(&self, require: &VersionReq) -> bool {
        require.matches(self.get_version())
    }

    fn client(&self) -> &Client;
    fn get_version(&self) -> &Version;
    fn location(&self) -> &Url;
    fn request<In, Out>(&self, method: Method, path: &str, input: In) -> Out
        where In: Serialize, Out: Deserialize {
        let mut add_path_parts = vec![path.to_string()];

        let mut location = self.location().clone();

        match location.path_mut() {
            Some(v) => { v.push(path.to_string()); },
            mut x => { mem::replace(&mut x, Some(&mut add_path_parts)); },
        };

        let body = serde_json::to_string(&input).unwrap();

        let mut response = self.client().
            request(method, location).
            body(&body).
            send().
            unwrap();

        let mut response_body = String::new();

        response.read_to_string(&mut response_body).unwrap();

        serde_json::from_str(&response_body).unwrap()
    }
}

pub trait Service {
    fn client(&self) -> &Client;
    fn listen(&self) -> HttpResult<Listening>;
}
