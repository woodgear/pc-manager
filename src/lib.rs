#[macro_use]
extern crate derive_new;

extern crate encoding;

#[macro_use]
mod util;

pub mod network_adapter;
pub mod process;

#[test]
fn lib_test_network_adapter() {
    let _=network_adapter::network_adapters();
}
