extern crate capnp;
extern crate futures;

#[macro_use]
extern crate capnp_rpc;

pub mod proxy_capnp {
    include!(concat!(env!("OUT_DIR"), "/proxy_capnp.rs"));
}

struct Proxy {}

use proxy_capnp::proxy::{HelloParams, HelloResults, Server};

impl Server for Proxy {
    fn hello(
        &mut self,
        params: HelloParams,
        mut results: HelloResults,
    ) -> ::capnp::capability::Promise<(), ::capnp::Error> {
        results.get().set_greetings(&format!(
            "Hello {}",
            pry!(params.get()).get_name().unwrap()
        ));
        ::capnp::capability::Promise::ok(())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn proxy_server_hello() {
        use Proxy;
        use futures::future::Future;

        let client = ::proxy_capnp::proxy::ToClient::new(Proxy {})
            .from_server::<::capnp_rpc::Server>();
        let mut request = client.hello_request();
        request.get().set_name("Proxy");
        let response = request.send().promise.wait().unwrap();
        assert_eq!(
            response.get().unwrap().get_greetings().unwrap(),
            "Hello Proxy"
        );
    }
}
