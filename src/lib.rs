#![feature(result_expect)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate semver;
extern crate hyper;
extern crate iron;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
mod service;

#[cfg(test)]
mod tests {
    #[test]
    fn external_service_macro_works_correctly() {
        external_services! {
            service DoSomethingService {
                properties {
                    version = "1.0.0"; 
                    location = "http://192.168.1.1";
                }
                
                paths {
                    get "dosomething" => fn do_something(first: String, second: String) -> String;
                }
            }
        }

        assert_eq!(
            DoSomethingService::new().do_something("first".to_string(), "second".to_string()).unwrap(),
            "http://192.168.1.1/dosomething with args RequestBody { first: \"first\", second: \"second\" }".to_string()
        );
    }

    #[test]
    fn service_macro_works_correctly() {
    }
}
