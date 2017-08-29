pub mod network_adapter;
mod util;
#[test]
fn lib_test_network_adapter() {
    let res=network_adapter::network_adapters();
    println!("{:#?}",res)
}