#[macro_use]
extern crate derive_new;

extern crate encoding;

#[macro_use]
mod util;
#[macro_use]
extern crate cpp;

pub mod network_adapter;
pub mod process;
mod c_util;
#[test]
fn lib_test_network_adapter() {
    let _=network_adapter::network_adapters();
}
