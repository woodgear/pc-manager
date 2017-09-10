#[macro_use]
extern crate derive_new;

extern crate winreg;

extern crate encoding;

#[macro_use]
mod util;
#[macro_use]
extern crate cpp;

extern crate csv;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod network_adapter;
pub mod process;
pub mod software;
mod c_util;
