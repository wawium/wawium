extern crate capnp;
extern crate futures;

#[macro_use]
extern crate capnp_rpc;

pub mod proxy_capnp {
    include!(concat!(env!("OUT_DIR"), "/proxy_capnp.rs"));
}

struct Proxy {}
use proxy_capnp::proxy::Server;

use proxy_capnp::proxy::{HelloParams, HelloResults};
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

    #[test]
    fn proxy_server_listInvoices() {
        use Proxy;
        use futures::future::Future;
        use proxy_capnp::proxy::ListInvoicesResults;

        let client = ::proxy_capnp::proxy::ToClient::new(Proxy {})
            .from_server::<::capnp_rpc::Server>();
        let mut request = client.list_invoices_request();
        request.get();
        let response = request.send().promise.wait().unwrap();
        assert_eq!(
            response.get().unwrap().get_invoice_list().unwrap().len() , 0
        );
    }
}
