#![feature(test)]

extern crate test;

use std::collections::{BTreeSet, HashSet};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use test::{black_box, Bencher};
use ahash::AHashSet;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::{PyBool, PyFloat, PyInt, PyIterator, PyList, PyString, PyTuple};

use rust_bench::{PyListBuilder, PyTupleBuilder, list_as_tuple};


fn run_startswith_rust(items: &PyList) -> PyResult<i32> {
    let mut count = 0;
    for item in items.iter() {
        let item_cow = item.downcast::<PyString>()?.to_string_lossy();
        if item_cow.as_ref().starts_with('_') {
            count += 1;
        }
    }
    Ok(count)
}

#[bench]
fn startswith_rust(bench: &mut Bencher) {
    Python::with_gil(|py| {
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

fn run_extract_string(py_any: &PyAny) -> bool {
    let str: String = py_any.extract().unwrap();
    return str == "foobar"
}

#[bench]
fn extract_string(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_any: &PyAny = PyString::new(py, "foobar");
        bench.iter(|| {
            black_box(run_extract_string(black_box(py_any)));
        });
    });
}

fn run_to_string_lossy(py_any: &PyAny) -> bool {
    let py_str: &PyString = py_any.downcast().unwrap();
    let str = py_str.to_string_lossy();
    return str.as_ref() == "foobar"
}

#[bench]
fn to_string_lossy(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_any: &PyAny = PyString::new(py, "foobar");
        bench.iter(|| {
            black_box(run_to_string_lossy(black_box(py_any)));
        });
    });
}

fn run_to_str(py_any: &PyAny) -> bool {
    let py_str: &PyString = py_any.downcast().unwrap();
    let str = py_str.to_str().unwrap();
    return str == "foobar"
}

#[bench]
fn to_str(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_any: &PyAny = PyString::new(py, "foobar");
        bench.iter(|| {
            black_box(run_to_str(black_box(py_any)));
        });
    });
}

fn run_is_str_cast_as(py_any: &PyAny) -> Option<String> {
    if let Ok(py_str) = py_any.downcast::<PyString>() {
        Some(py_str.to_str().unwrap().to_string())
    } else {
        None
    }
}

#[bench]
fn is_str_cast_as(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_any_str: &PyAny = PyString::new(py, "foobar");
        let py_int = 123.to_object(py);
        let py_any_int: &PyAny = py_int.extract(py).unwrap();
        bench.iter(|| {
            black_box(run_is_str_cast_as(black_box(py_any_str)));
            black_box(run_is_str_cast_as(black_box(py_any_int)));
        });
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
    Python::with_gil(|py| {
        let py_any_str: &PyAny = PyString::new(py, "foobar");
        let py_int = 123.to_object(py);
        let py_any_int: &PyAny = py_int.extract(py).unwrap();
        bench.iter(|| {
            black_box(run_is_str_extract(black_box(py_any_str)));
            black_box(run_is_str_extract(black_box(py_any_int)));
        });
    });
}

fn run_instantiation_tuple<'py>(py: Python<'py>, things: &[&PyAny]) -> &'py PyTuple {
    PyTuple::new(py, things)
}

#[bench]
fn instantiation_tuple(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec: Vec<&PyAny> = (0..100).map(|i| PyString::new(py, &i.to_string()) as &PyAny).collect();

        for _ in 0..100 {
            black_box(run_instantiation_tuple(black_box(py), black_box(&vec)));
        }

        bench.iter(|| {
            black_box(run_instantiation_tuple(black_box(py), black_box(&vec)));
        });
    });
}


fn run_instantiation_list<'py>(py: Python<'py>, things: &[&PyAny]) -> &'py PyList {
    PyList::new(py, things)
}

#[bench]
fn instantiation_list(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec: Vec<&PyAny> = (0..100).map(|i| PyString::new(py, &i.to_string()) as &PyAny).collect();

        for _ in 0..100 {
            black_box(run_instantiation_list(black_box(py), black_box(&vec)));
        }

        bench.iter(|| {
            black_box(run_instantiation_list(black_box(py), black_box(&vec)));
        });
    });
}

fn int_run_vec_contains(vec: &[i64], item: i64) -> bool {
    vec.contains(&item)
}

#[bench]
fn int_vec_contains(bench: &mut Bencher) {
    let vec: Vec<i64> = (0..5).collect();

    assert!(int_run_vec_contains(black_box(&vec), black_box(3)));
    assert!(!int_run_vec_contains(black_box(&vec), black_box(5)));

    bench.iter(|| {
        black_box(int_run_vec_contains(black_box(&vec), black_box(0)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(1)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(2)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(3)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(4)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(5)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(6)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(7)));
        black_box(int_run_vec_contains(black_box(&vec), black_box(8)));
    });
}

fn int_run_aset_contains(set: &AHashSet<i64>, item: i64) -> bool {
    set.contains(&item)
}

#[bench]
fn int_aset_contains(bench: &mut Bencher) {
    let mut set: AHashSet<i64> = AHashSet::with_capacity(5);
    for i in 0..5 {
        set.insert(i);
    }

    assert!(int_run_aset_contains(black_box(&set), black_box(3)));
    assert!(!int_run_aset_contains(black_box(&set), black_box(6)));

    bench.iter(|| {
        black_box(int_run_aset_contains(black_box(&set), black_box(0)));
        black_box(int_run_aset_contains(black_box(&set), black_box(1)));
        black_box(int_run_aset_contains(black_box(&set), black_box(2)));
        black_box(int_run_aset_contains(black_box(&set), black_box(3)));
        black_box(int_run_aset_contains(black_box(&set), black_box(4)));
        black_box(int_run_aset_contains(black_box(&set), black_box(5)));
        black_box(int_run_aset_contains(black_box(&set), black_box(6)));
        black_box(int_run_aset_contains(black_box(&set), black_box(7)));
        black_box(int_run_aset_contains(black_box(&set), black_box(8)));
    });
}


fn str_run_vec_contains(vec: &[String], item: &str) -> bool {
    vec.iter().any(|s| s.as_str() == item)
}

#[bench]
fn str_vec_contains(bench: &mut Bencher) {
    let mut vec: Vec<String> = Vec::with_capacity(5);
    for i in 0..5 {
        vec.push(format!("number {}", i));
    }

    assert!(str_run_vec_contains(black_box(&vec), black_box("number 2")));
    assert!(!str_run_vec_contains(black_box(&vec), black_box("number 5")));

    bench.iter(|| {
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 0")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 1")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 2")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 3")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 4")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 5")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 6")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 7")));
        black_box(str_run_vec_contains(black_box(&vec), black_box("number 8")));
    });
}

fn str_run_set_contains(set: &AHashSet<String>, item: &str) -> bool {
    set.contains(item)
}

#[bench]
fn str_set_contains(bench: &mut Bencher) {
    let mut set: AHashSet<String> = AHashSet::with_capacity(5);
    for i in 0..5 {
        set.insert(format!("number {}", i));
    }

    assert!(str_run_set_contains(black_box(&set), black_box("number 3")));
    assert!(!str_run_set_contains(black_box(&set), black_box("number 6")));

    bench.iter(|| {
        black_box(str_run_set_contains(black_box(&set), black_box("number 0")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 1")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 2")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 3")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 4")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 5")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 6")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 7")));
        black_box(str_run_set_contains(black_box(&set), black_box("number 8")));
    });
}


struct HashVec {
    vec: Vec<u64>,
    hash_builder: RandomState,
}

impl HashVec {
    fn new(capacity: usize) -> HashVec {
        HashVec {
            vec: Vec::with_capacity(capacity),
            hash_builder: RandomState::new(),
        }
    }

    fn push(&mut self, item: &str) {
        self.vec.push(self.hash(item));
    }

    fn contains(&self, item: &str) -> bool {
        self.vec.contains(&self.hash(item))
    }

    fn hash(&self, item: &str) -> u64 {
        // let hash = self.hash_builder.hash_one(item);
        let mut hasher = self.hash_builder.build_hasher();
        item.hash(&mut hasher);
        hasher.finish()
    }
}


fn str_run_hashvec_contains(hashvec: &HashVec, item: &str) -> bool {
    hashvec.contains(item)
}

#[bench]
fn str_hashvec_contains(bench: &mut Bencher) {
    let mut v: HashVec = HashVec::new(5);
    for i in 0..5 {
        v.push(&format!("number {}", i));
    }

    assert!(str_run_hashvec_contains(black_box(&v), black_box("number 3")));
    assert!(!str_run_hashvec_contains(black_box(&v), black_box("number 6")));

    bench.iter(|| {
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 0")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 1")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 2")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 3")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 4")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 5")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 6")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 7")));
        black_box(str_run_hashvec_contains(black_box(&v), black_box("number 8")));
    });
}


fn get_value(i: &usize) -> usize {
    // format!("value_{}", i)
    *i
}

fn run_py_list_builder<'py>(py: Python<'py>, input: &[usize]) -> PyResult<&'py PyList> {
    let mut list_builder = PyListBuilder::with_capacity(py, input.len())?;
    for i in input {
        list_builder.push(py , get_value(i))?;
    }
    list_builder.get(py)
}

fn run_py_list_builder_alt<'py>(py: Python<'py>, input: &[usize]) -> PyResult<&'py PyList> {
    let mut list_builder = PyListBuilder::with_capacity(py, input.len())?;
    for i in input {
        list_builder.push_alt(py , get_value(i))?;
    }
    list_builder.get(py)
}


fn run_py_list_builder_incomplete<'py>(py: Python<'py>, break_at: usize, input: &[usize]) -> PyResult<&'py PyList> {
    let mut list_builder = PyListBuilder::with_capacity(py, input.len())?;
    for i in input {
        list_builder.push(py , get_value(i))?;
        if i >= &break_at {
            break;
        }
    }
    Ok(list_builder.get_incomplete(py))
}

fn run_py_list_vec<'py>(py: Python<'py>, input: &[usize]) -> &'py PyList {
    let mut vec = Vec::with_capacity(input.len());
    for i in input {
        vec.push(get_value(i));
    }
    PyList::new(py, vec)
}

fn run_py_list_vec_incomplete<'py>(py: Python<'py>, break_at: usize, input: &[usize]) -> &'py PyList {
    let mut vec = Vec::with_capacity(input.len());
    for i in input {
        vec.push(get_value(i));
        if i >= &break_at {
            break;
        }
    }
    PyList::new(py, vec)
}

#[bench]
fn py_list_complete_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let list_5 = run_py_list_builder(py, &vec_5)?;
        let list_5_expected = run_py_list_vec(py, &vec_5);
        assert!(list_5.eq(list_5_expected)?);

        let vec_500: Vec<usize> = (0..500).collect();

        bench.iter(|| {
            let list_500 = run_py_list_builder(py, black_box(&vec_500)).unwrap();
            black_box(list_500);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_list_complete_builder_alt(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let list_5 = run_py_list_builder_alt(py, &vec_5)?;
        let list_5_expected = run_py_list_vec(py, &vec_5);
        assert!(list_5.eq(list_5_expected)?);

        let vec_500: Vec<usize> = (0..500).collect();

        bench.iter(|| {
            let list_500 = run_py_list_builder_alt(py, black_box(&vec_500)).unwrap();
            black_box(list_500);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_list_complete_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();

        bench.iter(|| {
            let py_list = run_py_list_vec(py, black_box(&vec_500));
            black_box(py_list);
        });
    });
}


#[bench]
fn py_list_incomplete_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let list_3 = run_py_list_builder_incomplete(py, 3, &vec_5)?;
        let list_3_expected = run_py_list_vec_incomplete(py, 3, &vec_5);
        assert!(list_3.eq(list_3_expected)?);

        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let list_40 = run_py_list_builder_incomplete(py, black_box(400), black_box(&vec_500)).unwrap();
            black_box(list_40);
        });
        Ok(())
    }).unwrap();
}


#[bench]
fn py_list_incomplete_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let py_list = run_py_list_vec_incomplete(py, black_box(400), black_box(&vec_500));
            black_box(py_list);
        });
    });
}

////////////////////////////

fn run_py_tuple_builder<'py>(py: Python<'py>, input: &[usize]) -> PyResult<&'py PyTuple> {
    let mut tuple_builder = PyTupleBuilder::with_capacity(py, input.len())?;
    for i in input {
        tuple_builder.push(py, get_value(i))?;
    }
    tuple_builder.get(py)
}

fn run_py_tuple_builder_incomplete<'py>(py: Python<'py>, break_at: usize, input: &[usize]) -> PyResult<&'py PyTuple> {
    let mut tuple_builder = PyTupleBuilder::with_capacity(py, input.len())?;
    for i in input {
        tuple_builder.push(py, get_value(i))?;
        if i >= &break_at {
            break;
        }
    }
    Ok(tuple_builder.get_incomplete(py))
}

fn run_py_tuple_vec<'py>(py: Python<'py>, input: &[usize]) -> &'py PyTuple {
    let mut vec = Vec::with_capacity(input.len());
    for i in input {
        vec.push(get_value(i));
    }
    PyTuple::new(py, vec)
}

fn run_py_tuple_vec_incomplete<'py>(py: Python<'py>, break_at: usize, input: &[usize]) -> &'py PyTuple {
    let mut vec = Vec::with_capacity(input.len());
    for i in input {
        vec.push(get_value(i));
        if i >= &break_at {
            break;
        }
    }
    PyTuple::new(py, vec)
}

#[bench]
fn py_tuple_complete_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let tuple_5 = run_py_tuple_builder(py, &vec_5)?;
        let tuple_5_expected = run_py_tuple_vec(py, &vec_5);
        assert!(tuple_5.eq(tuple_5_expected)?);

        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let tuple_500 = run_py_tuple_builder(py, black_box(&vec_500)).unwrap();
            black_box(tuple_500);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_tuple_complete_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let py_tuple = run_py_tuple_vec(py, black_box(&vec_500));
            black_box(py_tuple);
        });
    });
}


#[bench]
fn py_tuple_incomplete_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let tuple_3 = run_py_tuple_builder_incomplete(py, 3, &vec_5)?;
        let tuple_3_expected = run_py_tuple_vec_incomplete(py, 3, &vec_5);
        assert!(tuple_3.eq(tuple_3_expected)?);

        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let tuple_40 = run_py_tuple_builder_incomplete(py, black_box(400), black_box(&vec_500)).unwrap();
            black_box(tuple_40);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_tuple_incomplete_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        bench.iter(|| {
            let py_tuple = run_py_tuple_vec_incomplete(py, black_box(400), black_box(&vec_500));
            black_box(py_tuple);
        });
    });
}

#[bench]
fn list_as_tuple_direct(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_5 = vec![0, 1, 2, 3, 4];
        let py_list_5 = run_py_list_vec(py, &vec_5);
        let py_tuple_expected = run_py_tuple_vec(py, &vec_5);
        let py_tuple_5 = list_as_tuple(py, py_list_5);
        assert!(py_tuple_5.eq(py_tuple_expected).unwrap());

        let vec_500: Vec<usize> = (0..500).collect();
        let py_list_500 = run_py_list_vec(py, &vec_500);

        bench.iter(|| {
            let py_tuple = list_as_tuple(py, black_box(py_list_500));
            black_box(py_tuple);
        });
    });
}

#[bench]
fn list_as_tuple_iterate(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        let py_list_500 = run_py_list_vec(py, &vec_500);

        bench.iter(|| {
            let py_tuple = PyTuple::new(py, py_list_500);
            black_box(py_tuple);
        });
    });
}


fn run_list_iter(list: &PyList) -> PyResult<Vec<PyObject>> {
    let mut v = Vec::with_capacity(list.len());
    for item in list.iter() {
        v.push(item.to_object(list.py()));
    }
    Ok(v)
}

#[bench]
fn list_iter(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        let list: &PyList = PyList::new(py, vec_500);

        bench.iter(|| {
            let r = run_list_iter(list).unwrap();
            black_box(r);
        });
    });
}

fn run_any_list_iter(list: &PyAny, len: usize) -> PyResult<Vec<PyObject>> {
    let mut v = Vec::with_capacity(len);
    let py_iterator = list.iter()?;
    for item_result in py_iterator {
        let item = item_result?;
        v.push(item.to_object(list.py()));
    }
    Ok(v)
}

#[bench]
fn any_list_iter(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        let list: &PyList = PyList::new(py, vec_500);
        let list_any = list as &PyAny;

        bench.iter(|| {
            let r = run_any_list_iter(list_any, 500).unwrap();
            black_box(r);
        });
    });
}


fn run_iter_list_iter(py_iter: &PyIterator, len: usize) -> PyResult<Vec<PyObject>> {
    let mut v = Vec::with_capacity(len);
    for item_result in py_iter {
        let item = item_result?;
        v.push(item.to_object(py_iter.py()));
    }
    Ok(v)
}

#[bench]
fn iter_list_iter(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let vec_500: Vec<usize> = (0..500).collect();
        let list: &PyList = PyList::new(py, vec_500);
        let iterator: &PyIterator = PyIterator::from_object(py, list).unwrap();

        bench.iter(|| {
            let r = run_iter_list_iter(iterator, 500).unwrap();
            black_box(r);
        });
    });
}

#[bench]
fn extract_str_extract_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let s = PyString::new(py, "Hello, World!") as &PyAny;

        bench.iter(|| {
            let v = black_box(s).extract::<&str>().unwrap();
            black_box(v);
        });
    });
}

#[bench]
fn extract_str_extract_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).extract::<&str>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_str_downcast_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let s = PyString::new(py, "Hello, World!") as &PyAny;

        bench.iter(|| {
            let py_str = black_box(s).downcast::<PyString>().unwrap();
            let v = py_str.to_str().unwrap();
            black_box(v);
        });
    });
}

#[bench]
fn extract_str_downcast_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).downcast::<PyString>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_int_extract_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let int_obj: PyObject = 123.into_py(py);
        let int = int_obj.as_ref(py);

        bench.iter(|| {
            let v = black_box(int).extract::<i64>().unwrap();
            black_box(v);
        });
    });
}


#[bench]
fn extract_int_extract_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).extract::<i64>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_int_downcast_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let int_obj: PyObject = 123.into_py(py);
        let int = int_obj.as_ref(py);

        bench.iter(|| {
            let py_int = black_box(int).downcast::<PyInt>().unwrap();
            let v = py_int.extract::<i64>().unwrap();
            black_box(v);
        });
    });
}


#[bench]
fn extract_int_downcast_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).downcast::<PyInt>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_int_is_instance_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let int_obj: PyObject = 123.into_py(py);
        let int = int_obj.as_ref(py);

        bench.iter(|| {
            let input = black_box(int);
            let v = match PyInt::is_type_of(input) {
                true => input.extract::<i64>().unwrap(),
                false => panic!("not instance of int {}", input),
            };
            black_box(v);
        });
    });
}


#[bench]
fn extract_int_is_instance_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            let input = black_box(d);
            match PyInt::is_type_of(input) {
                true => panic!("should err {}", input),
                false => black_box(false),
            }
        });
    });
}

///////////////////////////

#[bench]
fn extract_float_extract_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let float_obj: PyObject = 123.0.into_py(py);
        let float = float_obj.as_ref(py);

        bench.iter(|| {
            let v = black_box(float).extract::<f64>().unwrap();
            black_box(v);
        });
    });
}


#[bench]
fn extract_float_extract_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).extract::<f64>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_float_downcast_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let float_obj: PyObject = 123.0.into_py(py);
        let float = float_obj.as_ref(py);

        bench.iter(|| {
            let py_float = black_box(float).downcast::<PyFloat>().unwrap();
            let v = py_float.extract::<f64>().unwrap();
            black_box(v);
        });
    });
}

#[bench]
fn extract_float_downcast_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).downcast::<PyFloat>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_float_isinstance_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let float_obj: PyObject = 123.0.into_py(py);
        let float = float_obj.as_ref(py);

        bench.iter(|| {
            let input = black_box(float);
            let v = match PyFloat::is_type_of(input) {
                true => input.extract::<f64>().unwrap(),
                false => panic!("not instance of float {}", input),
            };
            black_box(v);
        });
    });
}

///////////////////////// bool


#[bench]
fn extract_bool_extract_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let bool_obj: PyObject = true.into_py(py);
        let b = bool_obj.as_ref(py);

        bench.iter(|| {
            let v = black_box(b).extract::<bool>().unwrap();
            black_box(v);
        });
    });
}


#[bench]
fn extract_bool_extract_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).extract::<bool>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_bool_downcast_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let bool_obj: PyObject = true.into_py(py);
        let b = bool_obj.as_ref(py);

        bench.iter(|| {
            let py_bool = black_box(b).downcast::<PyBool>().unwrap();
            let v = py_bool.is_true();
            black_box(v);
        });
    });
}

#[bench]
fn extract_bool_downcast_fail(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let d = py.None().into_ref(py);

        bench.iter(|| {
            match black_box(d).downcast::<PyBool>() {
                Ok(v) => panic!("should err {}", v),
                Err(e) => black_box(e),
            }
        });
    });
}

#[bench]
fn extract_bool_isinstance_success(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let bool_obj: PyObject = true.into_py(py);
        let b = bool_obj.as_ref(py);

        bench.iter(|| {
            let input = black_box(b);
            let v = match PyBool::is_type_of(input) {
                true => input.is_true().unwrap(),
                false => panic!("not instance of bool {}", input),
            };
            black_box(v);
        });
    });
}
