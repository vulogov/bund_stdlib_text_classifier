extern crate log;
use easy_error::{bail, Error};

use rust_dynamic::value::Value;
use rust_multistackvm::multistackvm::{VM};

use crate::TEXTCLASSIFIERS;

pub fn textclassifier_new(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline TEXTCLASSIFIER.NEW");
    }

    let name_value = match vm.stack.pull() {
        Some(name_value) => name_value,
        None => bail!("TEXTCLASSIFIER: error getting classifier name"),
    };

    let cname = match name_value.cast_string() {
        Ok(cname) => cname,
        Err(err) => {
            bail!("textclassifier.new returned for #1: {}", err);
        }
    };

    let mut c = match TEXTCLASSIFIERS.lock() {
        Ok(c) => c,
        Err(err) => {
            bail!("textclassifier.new can not obtain the lock: {}", err);
        }
    };
    if c.classifier_exists(cname.clone()) {
        drop(c);
        return Ok(vm);
    }
    let _ = c.classifier(cname.clone());
    drop(c);
    vm.stack.push(Value::from_string(cname.clone()));
    Ok(vm)
}

pub fn textclassifier_exists(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 1 {
        bail!("Stack is too shallow for inline TEXTCLASSIFIER.EXIST");
    }

    let name_value = match vm.stack.pull() {
        Some(name_value) => name_value,
        None => bail!("TEXTCLASSIFIER: error getting classifier name"),
    };

    let cname = match name_value.cast_string() {
        Ok(cname) => cname,
        Err(err) => {
            bail!("textclassifier.exists returned for #1: {}", err);
        }
    };

    let mut c = match TEXTCLASSIFIERS.lock() {
        Ok(c) => c,
        Err(err) => {
            bail!("textclassifier.exist can not obtain the lock: {}", err);
        }
    };
    let res =  c.classifier_exists(cname.clone());
    drop(c);
    vm.stack.push(Value::from_bool(res));
    Ok(vm)
}
