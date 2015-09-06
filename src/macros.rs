#[macro_export]
macro_rules! external_services {
    ( $(
            service $srv_name:ident { 
                properties {
                    $($prop:ident = $val:expr;)*
                }

                paths {
                    $( $verb:ident $path:expr => fn $fname:ident( $( $argname:ident : $argtype:ty ),* ) $( -> $ret:ty )*; )*
                }
            }
    )+ ) => {
            use ::service::{ ExternalService, Service };
            use semver::{ Version, VersionReq };
            use hyper::{ Client, Url };
            use hyper::client::IntoUrl;
            use hyper::method::Method;

            $(
                struct $srv_name {
                    version: VersionReq,
                    location: Url,
                    client: Client,
                    concrete_version: Version,
                }

                impl ExternalService for $srv_name {
                    fn get_version(&self) -> &Version {
                        &self.concrete_version
                    }
                    
                    fn location(&self) -> &Url {
                        &self.location
                    }

                    fn client(&self) -> &Client {
                        &self.client
                    }
                }

                impl $srv_name {
                     fn new() -> $srv_name {
                        struct OParams<'a, T: 'a + IntoUrl> {
                            version: &'a str,
                            location: T,
                        }

                        let mut params = OParams {
                            version: "*",
                            location: "http://localhost",
                        };

                        $(
                            params.$prop = $val;
                        )*

                        let loc = params.location.into_url().unwrap();
                        
                        $srv_name {
                            version: VersionReq::parse(params.version).unwrap(),
                            location: loc,
                            concrete_version: Version::parse("0.0.1").unwrap(),
                            client: Client::new(),
                        }
                    }

                    $(
                        fn $fname( &self, $( $argname:$argtype ),* ) $( -> Result<$ret, ()> )* {
                            #[derive(Debug, Serialize, Deserialize)]
                            struct RequestBody {
                                $( $argname:$argtype ),*
                            }

                            let request_body: RequestBody = RequestBody {
                                $( $argname: $argname ),*
                            };

                            #[allow(non_camel_case_types)]
                            enum Verb {
                                put,
                                post,
                                get,
                                delete,
                            }

                            fn verb_to_method(v: Verb) -> Method {
                                match v {
                                    Verb::put => Method::Put,
                                    Verb::post => Method::Post,
                                    Verb::get => Method::Get,
                                    Verb::delete => Method::Delete,
                                }
                            }
            
                            let method = verb_to_method(Verb::$verb);
                            
                            // Ok(self.request(method, $path, request_body))
                            Ok(self.location.to_string() + $path + " with args " + &format!( "{:?}",  request_body ))
                        }
                    )*
                }
            )*
    }
}

// Creates a trait $srv_name with the Iron routing already handled -- just need to implement the
// methods in "paths".
macro_rules! service {
    { service $srv_name:ident { 
        $($prop:ident = $val:expr;)*

        paths {
            $( $verb:ident $path:expr => fn $fname:ident( $( $argname:ident : $argtype:ty ),* ) $( -> $ret:ty )*; )*
        }
    } } => {
        use iron::middleware::Handler;
        use router::Router;

        trait $srv_name {
            $( fn $fname:ident( $( $argname:ident : $argtype:ty ),* ) $( -> $ret:ty )*; )*
        }

        impl Handler for T where T: $srv_name {
            fn handle(&self, req: Request) -> Response {
                let 
            }
        }
    }
}
