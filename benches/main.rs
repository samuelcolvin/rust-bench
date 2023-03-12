#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use rust_bench::{PyListBuilder, PyTupleBuilder, list_as_tuple};

fn run_py_list_builder(py: Python, capacity: usize, to_push: usize) -> PyResult<Py<PyList>> {
    let mut list_builder = PyListBuilder::with_capacity(py, capacity)?;
    for i in 0..to_push {
        list_builder.push(i.into_py(py))?;
    }
    Ok(list_builder.complete(py))
}

fn run_py_list_vec(py: Python, capacity: usize, to_push: usize) -> Py<PyList> {
    let mut vec = Vec::with_capacity(capacity);
    for i in 0..to_push {
        vec.push(i.into_py(py));
    }
    PyList::new(py, vec).into_py(py)
}

#[bench]
fn py_list_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let list_5 = run_py_list_builder(py, 5, 5)?;
        let list_5_expected = run_py_list_vec(py, 5, 5);
        assert!(list_5.as_ref(py).eq(list_5_expected.as_ref(py))?);

        bench.iter(|| {
            let list_50 = run_py_list_builder(py, 50, 50).unwrap();
            black_box(list_50);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_list_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        bench.iter(|| {
            let py_list = run_py_list_vec(py, 50, 50);
            black_box(py_list);
        });
    });
}


#[bench]
fn py_list_builder_incomplete(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let list_5 = run_py_list_builder(py, 10, 5)?;
        let list_5_expected = run_py_list_vec(py, 5, 5);
        assert!(list_5.as_ref(py).eq(list_5_expected.as_ref(py))?);

        bench.iter(|| {
            let list_40 = run_py_list_builder(py, 50, 40).unwrap();
            black_box(list_40);
        });
        Ok(())
    }).unwrap();
}


#[bench]
fn py_list_vec_incomplete(bench: &mut Bencher) {
    Python::with_gil(|py| {
        bench.iter(|| {
            let py_list = run_py_list_vec(py, 50, 40);
            black_box(py_list);
        });
    });
}

////////////////////////////

fn run_py_tuple_builder(py: Python, capacity: usize, to_push: usize) -> PyResult<Py<PyTuple>> {
    let mut tuple_builder = PyTupleBuilder::with_capacity(py, capacity)?;
    for i in 0..to_push {
        tuple_builder.push(i.into_py(py))?;
    }
    Ok(tuple_builder.complete(py))
}

fn run_py_tuple_vec(py: Python, capacity: usize, to_push: usize) -> Py<PyTuple> {
    let mut vec = Vec::with_capacity(capacity);
    for i in 0..to_push {
        vec.push(i.into_py(py));
    }
    PyTuple::new(py, vec).into_py(py)
}

#[bench]
fn py_tuple_builder(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let tuple_5 = run_py_tuple_builder(py, 5, 5)?;
        let tuple_5_expected = run_py_tuple_vec(py, 5, 5);
        assert!(tuple_5.as_ref(py).eq(tuple_5_expected.as_ref(py))?);

        bench.iter(|| {
            let tuple_50 = run_py_tuple_builder(py, 50, 50).unwrap();
            black_box(tuple_50);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_tuple_vec(bench: &mut Bencher) {
    Python::with_gil(|py| {
        bench.iter(|| {
            let py_tuple = run_py_tuple_vec(py, 50, 50);
            black_box(py_tuple);
        });
    });
}


#[bench]
fn py_tuple_builder_incomplete(bench: &mut Bencher) {
    Python::with_gil(|py| -> PyResult<()> {
        let tuple_5 = run_py_tuple_builder(py, 10, 5)?;
        let tuple_5_expected = run_py_tuple_vec(py, 5, 5);
        assert!(tuple_5.as_ref(py).eq(tuple_5_expected.as_ref(py))?);

        bench.iter(|| {
            let tuple_40 = run_py_tuple_builder(py, 50, 40).unwrap();
            black_box(tuple_40);
        });
        Ok(())
    }).unwrap();
}

#[bench]
fn py_tuple_vec_incomplete(bench: &mut Bencher) {
    Python::with_gil(|py| {
        bench.iter(|| {
            let py_tuple = run_py_tuple_vec(py, 50, 40);
            black_box(py_tuple);
        });
    });
}

#[bench]
fn list_as_tuple_direct(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_list_5 = run_py_list_vec(py, 5, 5);
        let py_tuple_expected = run_py_tuple_vec(py, 5, 5);
        let py_tuple_5 = list_as_tuple(py, py_list_5.as_ref(py));
        assert!(py_tuple_5.eq(py_tuple_expected.as_ref(py)).unwrap());

        let py_list_50 = run_py_list_vec(py, 50, 50);
        let py_list= py_list_50.as_ref(py);

        bench.iter(|| {
            let py_tuple = list_as_tuple(py, black_box(py_list));
            black_box(py_tuple);
        });
    });
}

#[bench]
fn list_as_tuple_iterate(bench: &mut Bencher) {
    Python::with_gil(|py| {
        let py_list_50 = run_py_list_vec(py, 50, 50);
        let py_list= py_list_50.as_ref(py);

        bench.iter(|| {
            let py_tuple = PyTuple::new(py, py_list);
            black_box(py_tuple);
        });
    });
}
