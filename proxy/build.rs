extern crate capnpc;

#[cfg(not(windows))]
fn main() {
    match ::capnpc::CompilerCommand::new()
        .src_prefix("src")
        .file("src/proxy.capnp")
        .run() {
        Ok(r) => println!("Got result {:?}", r),
        Err(e) => panic!(e),
    }
}
