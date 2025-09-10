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


pub fn textclassifier_train_from_file(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 3 {
        bail!("Stack is too shallow for inline TEXTCLASSIFIER.TRAIN.FROM_FILE");
    }

    let fname_value = match vm.stack.pull() {
        Some(fname_value) => fname_value,
        None => bail!("TEXTCLASSIFIER: error getting file name"),
    };

    let fname = match fname_value.cast_string() {
        Ok(fname) => fname,
        Err(err) => {
            bail!("textclassifier.train.from_file returned for #1: {}", err);
        }
    };

    let tname_value = match vm.stack.pull() {
        Some(tname_value) => tname_value,
        None => bail!("TEXTCLASSIFIER: error getting topic name"),
    };

    let tname = match tname_value.cast_string() {
        Ok(tname) => tname,
        Err(err) => {
            bail!("textclassifier.train.from_file returned for #2: {}", err);
        }
    };

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
            bail!("textclassifier.train.from_file can not obtain the lock: {}", err);
        }
    };
    let res =  c.classifier(cname.clone()).train_from_file(tname, fname);
    drop(c);
    vm.stack.push(Value::from_string(cname.clone()));
    vm.stack.push(Value::from_int(res as i64));
    Ok(vm)
}

pub fn textclassifier_train_finish(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
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
    if ! c.classifier_exists(cname.clone()) {
        drop(c);
        return Ok(vm);
    }
    c.classifier(cname.clone()).finish();
    drop(c);
    vm.stack.push(Value::from_string(cname.clone()));
    Ok(vm)
}

pub fn textclassifier_classify(vm: &mut VM) -> std::result::Result<&mut VM, Error> {
    if vm.stack.current_stack_len() < 2 {
        bail!("Stack is too shallow for inline TEXTCLASSIFIER.CLASSIFY");
    }

    let data_value = match vm.stack.pull() {
        Some(data_value) => data_value,
        None => bail!("TEXTCLASSIFIER: error getting data"),
    };

    let data = match data_value.cast_string() {
        Ok(data) => data,
        Err(err) => {
            bail!("textclassifier.classify returned for #1: {}", err);
        }
    };

    let name_value = match vm.stack.pull() {
        Some(name_value) => name_value,
        None => bail!("TEXTCLASSIFIER: error getting classifier name"),
    };

    let cname = match name_value.cast_string() {
        Ok(cname) => cname,
        Err(err) => {
            bail!("textclassifier.classify returned for #1: {}", err);
        }
    };

    let mut c = match TEXTCLASSIFIERS.lock() {
        Ok(c) => c,
        Err(err) => {
            bail!("textclassifier.classify can not obtain the lock: {}", err);
        }
    };
    let res =  c.classifier(cname.clone()).classify(data);
    let mut ret = Value::dict();
    drop(c);
    for (key, val) in res.iter() {
        ret = ret.set(key, Value::from_float(*val as f64));
    }
    vm.stack.push(Value::from_string(cname.clone()));
    vm.stack.push(ret);
    Ok(vm)
}
