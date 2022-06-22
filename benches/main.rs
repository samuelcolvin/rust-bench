#![feature(test)]

extern crate test;
extern crate core;

use test::{black_box, Bencher};
use pyo3::{AsPyPointer, ToBorrowedObject};
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PySet, PyString};

fn populate_dict_simple(py: Python) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for i in 0..100 {
        dict.set_item(i.to_string(), i)?;
    }
    Ok(dict.into_py(py))
}


#[bench]
fn dict_simple(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    bench.iter(|| {
        black_box(populate_dict_simple(py).unwrap());
    });
}

fn populate_dict_list(py: Python) -> PyResult<PyObject> {
    let mut items: Vec<(String, i64)> = Vec::with_capacity(100);
    for i in 0..100 {
        items.push((i.to_string(), i));
    }
    let dict = PyDict::from_sequence(py, items.into_py(py))?;
    Ok(dict.into_py(py))
}

#[bench]
fn dict_list(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    bench.iter(|| {
        black_box(populate_dict_list(py).unwrap());
    });
}

pub fn error_on_minusone(py: Python<'_>, result: std::os::raw::c_int) -> PyResult<()> {
    if result != -1 {
        Ok(())
    } else {
        Err(PyErr::fetch(py))
    }
}

fn populate_dict_reuse_str(py: Python, keys: &Vec<&PyString>) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for i in 0..100 {
        let key = keys[i];
        let value = i;
        dict.set_item(key, value)?;
    }
    Ok(dict.into_py(py))
}

#[bench]
fn dict_reuse_str(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let keys: Vec<&PyString> = (0..100).map(|i| PyString::new(py, &i.to_string())).collect();

    bench.iter(|| {
        black_box(populate_dict_reuse_str(py, &keys).unwrap());
    });
}

fn populate_dict_reuse_known_hash(py: Python, keys: &Vec<(&PyString, isize)>) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for i in 0..100 {
        let (key, hash) = keys[i];
        let value = i;
        let key_ptr = key.as_ptr();
        value.with_borrowed_ptr(py, |value| unsafe {
            error_on_minusone(py, ffi::_PyDict_SetItem_KnownHash(dict.as_ptr(), key_ptr, value, hash))
        })?
    }
    Ok(dict.into_py(py))
}

#[bench]
fn dict_reuse_known_hash(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let keys: Vec<(&PyString, isize)> = (0..100).map(|i| {
        let s = PyString::new(py, &i.to_string());
        let hash = s.hash().unwrap();
        (s, hash)
    }).collect();

    bench.iter(|| {
        black_box(populate_dict_reuse_known_hash(py, &keys).unwrap());
    });
}

fn populate_dict_known_hash(py: Python) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for i in 0..100 {
        let key = i.to_string();
        let value = i;
        let hash = i as isize;
        let key_ptr = key.into_py(py).as_ptr();
        value.with_borrowed_ptr(py, |value| unsafe {
            error_on_minusone(py, ffi::_PyDict_SetItem_KnownHash(dict.as_ptr(), key_ptr, value, hash))
        })?
    }
    Ok(dict.into_py(py))
}

#[bench]
fn dict_known_hash(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    /// check this works with `cargo bench -- --nocapture`
    // let dict = populate_dict_known_hash(py).unwrap();
    // let dict: &PyDict = dict.extract(py).unwrap();
    // for (key, value) in dict.iter() {
    //     println!("{:?} -> {:?}", key, value);
    // }

    bench.iter(|| {
        black_box(populate_dict_known_hash(py).unwrap());
    });
}

fn populate_set_simple(py: Python) -> PyResult<PyObject> {
    let set = PySet::empty(py)?;
    for i in 0..100 {
        set.add(i.to_string())?;
    }
    Ok(set.into_py(py))
}


#[bench]
fn set_simple(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    bench.iter(|| {
        black_box(populate_set_simple(py).unwrap());
    });
}


fn populate_set_vec(py: Python, keys: &Vec<&PyString>) -> PyResult<PyObject> {
    let mut vec: Vec<&PyString> = Vec::with_capacity(100);
    for i in 0..100 {
        let key = keys[i];
        vec.push(key);
    }
    let set = PySet::new(py, &vec)?;
    Ok(set.into_py(py))
}


#[bench]
fn set_vec(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let keys: Vec<&PyString> = (0..100).map(|i| PyString::intern(py, &i.to_string())).collect();

    bench.iter(|| {
        black_box(populate_set_vec(py, &keys).unwrap());
    });
}
