#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use rust_bench::{PyListBuilder, PyTupleBuilder, list_as_tuple};

fn get_value(i: &usize) -> String {
    format!("value_{}", i)
    // *i
}

fn run_py_list_builder<'py>(py: Python<'py>, input: &[usize]) -> PyResult<&'py PyList> {
    let mut list_builder = PyListBuilder::with_capacity(py, input.len())?;
    for i in input {
        list_builder.push(py , get_value(i))?;
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
