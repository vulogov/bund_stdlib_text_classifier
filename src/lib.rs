use easy_error::{Error};

use bundcore::bundcore::Bund;

pub mod classifier_filter;
pub mod topic_classifier;

pub fn init_lib(vm: &Bund) -> Result<&Bund, Error> {
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
