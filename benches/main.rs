#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

use pyo3::prelude::*;
use pyo3::types::{PyInt, PyIterator, PyList, PyString, PyTuple};

use rust_bench::{PyListBuilder, PyTupleBuilder, list_as_tuple};

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
