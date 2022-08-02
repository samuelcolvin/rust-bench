#![feature(test)]

extern crate core;
extern crate test;

use std::collections::{BTreeSet, HashSet};
use std::hash::BuildHasherDefault;

use test::{black_box, Bencher};

use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::type_object::PyTypeObject;
use pyo3::types::{PyBool, PyDict, PyList, PySet, PyString, PyTuple};
use pyo3::{intern, AsPyPointer, ToBorrowedObject};

use ahash::AHashSet;
use nohash_hasher::NoHashHasher;

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

    let keys: Vec<&PyString> = (0..100)
        .map(|i| PyString::new(py, &i.to_string()))
        .collect();

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
            error_on_minusone(
                py,
                ffi::_PyDict_SetItem_KnownHash(dict.as_ptr(), key_ptr, value, hash),
            )
        })?
    }
    Ok(dict.into_py(py))
}

#[bench]
fn dict_reuse_known_hash(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let keys: Vec<(&PyString, isize)> = (0..100)
        .map(|i| {
            let s = PyString::new(py, &i.to_string());
            let hash = s.hash().unwrap();
            (s, hash)
        })
        .collect();

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
            error_on_minusone(
                py,
                ffi::_PyDict_SetItem_KnownHash(dict.as_ptr(), key_ptr, value, hash),
            )
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
    let keys: Vec<&PyString> = (0..100)
        .map(|i| PyString::intern(py, &i.to_string()))
        .collect();

    bench.iter(|| {
        black_box(run_set_vec(py, &keys).unwrap());
    });
}

fn run_set_vec2(py: Python, keys: &Vec<Py<PyString>>) -> PyResult<PyObject> {
    let mut vec: Vec<Py<PyString>> = Vec::with_capacity(100);
    for i in 0..100 {
        let key = &keys[i];
        vec.push(key.clone_ref(py));
    }
    let set = PySet::new(py, &vec)?;
    Ok(set.into_py(py))
}

#[bench]
fn set_vec2(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let keys: Vec<Py<PyString>> = (0..100)
        .map(|i| PyString::intern(py, &i.to_string()).into_py(py))
        .collect();

    bench.iter(|| {
        black_box(run_set_vec2(py, &keys).unwrap());
    });
}

fn run_hashset_simple(checks: &[String]) -> i32 {
    let mut set: HashSet<String> = HashSet::with_capacity(100);
    for i in 0..100 {
        set.insert(i.to_string());
    }
    let mut count = 0;
    for i in checks {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn hashset_simple(bench: &mut Bencher) {
    let checks: Vec<String> = vec![
        "1".to_string(),
        "50".to_string(),
        "51".to_string(),
        "99".to_string(),
        "100".to_string(),
    ];
    bench.iter(|| {
        black_box(run_hashset_simple(&checks));
    });
}

fn run_hashset_vec(checks: &[String]) -> i32 {
    let mut vec: Vec<String> = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i.to_string());
    }
    let set: HashSet<String> = HashSet::from_iter(vec);
    let mut count = 0;
    for i in checks {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn hashset_vec(bench: &mut Bencher) {
    let checks: Vec<String> = vec![
        "1".to_string(),
        "50".to_string(),
        "51".to_string(),
        "99".to_string(),
        "100".to_string(),
    ];
    bench.iter(|| {
        black_box(run_hashset_vec(&checks));
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
    let items: Vec<PyObject> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                i.to_string().to_object(py)
            } else {
                true.to_object(py)
            }
        })
        .collect();
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
    let items: Vec<PyObject> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                i.to_string().to_object(py)
            } else {
                true.to_object(py)
            }
        })
        .collect();
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
    let items: Vec<PyObject> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                i.to_string().to_object(py)
            } else {
                true.to_object(py)
            }
        })
        .collect();
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
        let startswith_func = item.cast_as::<PyString>()?.getattr(startswith_pys)?;
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
    let items: Vec<PyObject> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                i.to_string().to_object(py)
            } else {
                format!("_{}", i).to_object(py)
            }
        })
        .collect();
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
    let items: Vec<PyObject> = (0..100)
        .map(|i| {
            if i % 2 == 0 {
                i.to_string().to_object(py)
            } else {
                format!("_{}", i).to_object(py)
            }
        })
        .collect();
    let py_list = PyList::new(py, &items);
    assert_eq!(run_startswith_rust(py_list).unwrap(), 50);

    bench.iter(|| {
        black_box(run_startswith_rust(py_list).unwrap());
    });
}

fn run_rust_set_hash_set(to_check: &[i32]) -> i32 {
    let mut set: HashSet<i32> = HashSet::with_capacity(100);
    for i in 0..100 {
        set.insert(i);
    }
    let mut count = 0;
    for i in to_check {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn rust_set_hash_set(bench: &mut Bencher) {
    let primes: Vec<i32> = vec![
        1, 3, 5, 7, 11, 13, 1779, 83, 89, 97, 101, 103, 107, 109, 111, 199,
    ];
    assert_eq!(run_rust_set_hash_set(&primes), 9);

    bench.iter(|| {
        black_box(run_rust_set_hash_set(black_box(&primes)));
    });
}

fn run_rust_set_btree_set(to_check: &[i32]) -> i32 {
    let mut set: BTreeSet<i32> = BTreeSet::new();
    for i in 0..100 {
        set.insert(i);
    }
    let mut count = 0;
    for i in to_check {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn rust_set_btree_set(bench: &mut Bencher) {
    let primes: Vec<i32> = vec![
        1, 3, 5, 7, 11, 13, 1779, 83, 89, 97, 101, 103, 107, 109, 111, 199,
    ];
    assert_eq!(run_rust_set_btree_set(&primes), 9);

    bench.iter(|| {
        black_box(run_rust_set_btree_set(black_box(&primes)));
    });
}

fn run_rust_set_a_hash_set(to_check: &[i32]) -> i32 {
    let mut set: AHashSet<i32> = AHashSet::with_capacity(100);
    for i in 0..100 {
        set.insert(i);
    }
    let mut count = 0;
    for i in to_check {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn rust_set_a_hash_set(bench: &mut Bencher) {
    let primes: Vec<i32> = vec![
        1, 3, 5, 7, 11, 13, 1779, 83, 89, 97, 101, 103, 107, 109, 111, 199,
    ];
    assert_eq!(run_rust_set_a_hash_set(&primes), 9);

    bench.iter(|| {
        black_box(run_rust_set_a_hash_set(black_box(&primes)));
    });
}

fn run_rust_set_no_hash_set(to_check: &[i32]) -> i32 {
    let mut set: HashSet<i32, BuildHasherDefault<NoHashHasher<i32>>> =
        HashSet::with_capacity_and_hasher(100, BuildHasherDefault::default());
    for i in 0..100 {
        set.insert(i);
    }
    let mut count = 0;
    for i in to_check {
        if set.contains(i) {
            count += 1;
        }
    }
    count
}

#[bench]
fn rust_set_no_hash_set(bench: &mut Bencher) {
    let primes: Vec<i32> = vec![
        1, 3, 5, 7, 11, 13, 1779, 83, 89, 97, 101, 103, 107, 109, 111, 199,
    ];
    assert_eq!(run_rust_set_no_hash_set(&primes), 9);

    bench.iter(|| {
        black_box(run_rust_set_no_hash_set(black_box(&primes)));
    });
}

fn run_extract_string(py_any: &PyAny) -> bool {
    let str: String = py_any.extract().unwrap();
    return str == "foobar"
}


#[bench]
fn extract_string(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let py_any: &PyAny = PyString::new(py, "foobar");
    bench.iter(|| {
        black_box(run_extract_string(black_box(py_any)));
    });
}

fn run_to_string_lossy(py_any: &PyAny) -> bool {
    let py_str: &PyString = py_any.cast_as().unwrap();
    let str = py_str.to_string_lossy();
    return str.as_ref() == "foobar"
}

#[bench]
fn to_string_lossy(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let py_any: &PyAny = PyString::new(py, "foobar");
    bench.iter(|| {
        black_box(run_to_string_lossy(black_box(py_any)));
    });
}

fn run_to_str(py_any: &PyAny) -> bool {
    let py_str: &PyString = py_any.cast_as().unwrap();
    let str = py_str.to_str().unwrap();
    return str == "foobar"
}

#[bench]
fn to_str(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let py_any: &PyAny = PyString::new(py, "foobar");
    bench.iter(|| {
        black_box(run_to_str(black_box(py_any)));
    });
}

fn run_is_str_cast_as(py_any: &PyAny) -> Option<String> {
    if let Ok(py_str) = py_any.cast_as::<PyString>() {
        Some(py_str.to_str().unwrap().to_string())
    } else {
        None
    }
}

#[bench]
fn is_str_cast_as(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let py_any_str: &PyAny = PyString::new(py, "foobar");
    let py_int = 123.to_object(py);
    let py_any_int: &PyAny = py_int.extract(py).unwrap();
    bench.iter(|| {
        black_box(run_is_str_cast_as(black_box(py_any_str)));
        black_box(run_is_str_cast_as(black_box(py_any_int)));
    });
}

fn run_is_str_extract(py_any: &PyAny) -> Option<String> {
    if let Ok(str) = py_any.extract::<String>() {
        Some(str)
    } else {
        None
    }
}

#[bench]
fn is_str_extract(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let py_any_str: &PyAny = PyString::new(py, "foobar");
    let py_int = 123.to_object(py);
    let py_any_int: &PyAny = py_int.extract(py).unwrap();
    bench.iter(|| {
        black_box(run_is_str_extract(black_box(py_any_str)));
        black_box(run_is_str_extract(black_box(py_any_int)));
    });
}

fn run_instantiation_tuple<'py>(py: Python<'py>, things: &[&PyAny]) -> &'py PyTuple {
    PyTuple::new(py, things)
}

#[bench]
fn instantiation_tuple(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let vec: Vec<&PyAny> = (0..100).map(|i| PyString::new(py, &i.to_string()) as &PyAny).collect();

    for _ in 0..100 {
        black_box(run_instantiation_tuple(black_box(py), black_box(&vec)));
    }

    bench.iter(|| {
        black_box(run_instantiation_tuple(black_box(py), black_box(&vec)));
    });
}


fn run_instantiation_list<'py>(py: Python<'py>, things: &[&PyAny]) -> &'py PyList {
    PyList::new(py, things)
}

#[bench]
fn instantiation_list(bench: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let vec: Vec<&PyAny> = (0..100).map(|i| PyString::new(py, &i.to_string()) as &PyAny).collect();

    for _ in 0..100 {
        black_box(run_instantiation_list(black_box(py), black_box(&vec)));
    }

    bench.iter(|| {
        black_box(run_instantiation_list(black_box(py), black_box(&vec)));
    });
}


