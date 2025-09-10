use easy_error::{Error};
use lazy_static::lazy_static;
use std::sync::Mutex;

use bundcore::bundcore::Bund;
use crate::classifiers::Classifiers;

pub mod classifier_filter;
pub mod topic_classifier;
pub mod classifiers;
pub mod bund_interface;

use crate::bund_interface::*;

lazy_static! {
    pub static ref TEXTCLASSIFIERS: Mutex<Classifiers> = {
        let e: Mutex<Classifiers> = Mutex::new(Classifiers::new());
        e
    };
}

pub fn init_lib(vm: &mut Bund) -> Result<&Bund, Error> {
    let _ = vm.vm.register_inline("textclassifier.new".to_string(), textclassifier_new);
    let _ = vm.vm.register_inline("textclassifier.exists".to_string(), textclassifier_exists);
    let _ = vm.vm.register_inline("textclassifier.train.from_file".to_string(), textclassifier_train_from_file);
    let _ = vm.vm.register_inline("textclassifier.train.finish".to_string(), textclassifier_train_finish);
    let _ = vm.vm.register_inline("textclassifier.classify".to_string(), textclassifier_classify);
    Ok(vm)
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
