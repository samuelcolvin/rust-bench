#![feature(test)]

extern crate test;
extern crate core;

use test::{black_box, Bencher};
use pyo3::{AsPyPointer, intern, ToBorrowedObject};
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList, PySet, PyString};
use pyo3::type_object::PyTypeObject;

fn run_dict_simple(py: Python) -> PyResult<PyObject> {
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
        black_box(run_dict_simple(py).unwrap());
    });
}

fn run_dict_list(py: Python) -> PyResult<PyObject> {
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
        black_box(run_dict_list(py).unwrap());
    });
}

pub fn error_on_minusone(py: Python<'_>, result: std::os::raw::c_int) -> PyResult<()> {
    if result != -1 {
        Ok(())
    } else {
        Err(PyErr::fetch(py))
    }
}

fn run_dict_reuse_str(py: Python, keys: &Vec<&PyString>) -> PyResult<PyObject> {
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
        black_box(run_dict_reuse_str(py, &keys).unwrap());
    });
}

fn run_dict_reuse_known_hash(py: Python, keys: &Vec<(&PyString, isize)>) -> PyResult<PyObject> {
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
        black_box(run_dict_reuse_known_hash(py, &keys).unwrap());
    });
}

fn run_dict_known_hash(py: Python) -> PyResult<PyObject> {
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

    // check this works with `cargo bench -- --nocapture`
    // let dict = run_dict_known_hash(py).unwrap();
    // let dict: &PyDict = dict.extract(py).unwrap();
    // for (key, value) in dict.iter() {
    //     println!("{:?} -> {:?}", key, value);
    // }

    bench.iter(|| {
        black_box(run_dict_known_hash(py).unwrap());
    });
}

fn run_set_simple(py: Python) -> PyResult<PyObject> {
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
        black_box(run_set_simple(py).unwrap());
    });
}


fn run_set_vec(py: Python, keys: &Vec<&PyString>) -> PyResult<PyObject> {
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
        black_box(run_set_vec(py, &keys).unwrap());
    });
}

fn run_isinstance_bool_extract(items: &PyList) -> i32 {
    let mut count = 0;
    for item in items.iter() {
        if item.cast_as::<PyBool>().is_ok() {
            count += 1;
        }
    }
    count
}

#[bench]
fn isinstance_bool_extract(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let items: Vec<PyObject> = (0..100).map(|i| {
        if i % 2 == 0 {
            i.to_string().to_object(py)
        } else {
            true.to_object(py)
        }
    }).collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_isinstance_bool_extract(py_list), 50);

    bench.iter(|| {
        black_box(run_isinstance_bool_extract(py_list));
    });
}

fn run_isinstance_bool_isinstance(items: &PyList) -> i32 {
    let mut count = 0;
    for item in items.iter() {
        if matches!(item.is_instance_of::<PyBool>(), Ok(true)) {
            count += 1;
        }
    }
    count
}

#[bench]
fn isinstance_bool_isinstance(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let items: Vec<PyObject> = (0..100).map(|i| {
        if i % 2 == 0 {
            i.to_string().to_object(py)
        } else {
            true.to_object(py)
        }
    }).collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_isinstance_bool_isinstance(py_list), 50);

    bench.iter(|| {
        black_box(run_isinstance_bool_isinstance(py_list));
    });
}

fn run_isinstance_bool_type_is(items: &PyList) -> i32 {
    let mut count = 0;
    let bool_type = PyBool::type_object(items.py());
    for item in items.iter() {
        let t = item.get_type();
        if t.is(bool_type) {
            count += 1;
        }
    }
    count
}

#[bench]
fn isinstance_bool_type_is(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let items: Vec<PyObject> = (0..100).map(|i| {
        if i % 2 == 0 {
            i.to_string().to_object(py)
        } else {
            true.to_object(py)
        }
    }).collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_isinstance_bool_type_is(py_list), 50);

    bench.iter(|| {
        black_box(run_isinstance_bool_type_is(py_list));
    });
}

fn run_startswith_py(items: &PyList) -> PyResult<i32> {
    let mut count = 0;
    let startswith_pys = intern!(items.py(), "startswith");
    let underscore_pys = intern!(items.py(), "_");
    for item in items.iter() {
        let startswith_func =  item.cast_as::<PyString>()?.getattr(startswith_pys)?;
        if startswith_func.call1((underscore_pys,))?.is_true()? {
            count += 1;
        }
    }
    Ok(count)
}

#[bench]
fn startswith_py(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let items: Vec<PyObject> = (0..100).map(|i| {
        if i % 2 == 0 {
            i.to_string().to_object(py)
        } else {
            format!("_{}", i).to_object(py)
        }
    }).collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_startswith_py(py_list).unwrap(), 50);

    bench.iter(|| {
        black_box(run_startswith_py(py_list).unwrap());
    });
}

fn run_startswith_rust(items: &PyList) -> PyResult<i32> {
    let mut count = 0;
    for item in items.iter() {
        let item_cow = item.cast_as::<PyString>()?.to_string_lossy();
        if item_cow.as_ref().starts_with('_') {
            count += 1;
        }
    }
    Ok(count)
}

#[bench]
fn startswith_rust(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let items: Vec<PyObject> = (0..100).map(|i| {
        if i % 2 == 0 {
            i.to_string().to_object(py)
        } else {
            format!("_{}", i).to_object(py)
        }
    }).collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_startswith_rust(py_list).unwrap(), 50);

    bench.iter(|| {
        black_box(run_startswith_rust(py_list).unwrap());
    });
}
