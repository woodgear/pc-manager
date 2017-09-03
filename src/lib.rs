#[macro_use]
extern crate derive_new;
#[macro_use]
mod util;

pub mod network_adapter;
#[test]
fn lib_test_network_adapter() {
    let _=network_adapter::network_adapters();
}
