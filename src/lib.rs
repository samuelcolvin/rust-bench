use std::cmp::Ordering;

use pyo3::AsPyPointer;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::ffi::{Py_ssize_t, PyList_AsTuple, PyList_GetSlice, PyList_New, PyList_SetItem, PyTuple_GetSlice, PyTuple_New, PyTuple_SetItem};
use pyo3::types::{PyList, PyTuple};


pub struct PyListBuilder {
    len: Py_ssize_t,
    list: Py<PyList>,
    counter: Py_ssize_t,
}

impl PyListBuilder {
    pub fn with_capacity(py: Python, capacity: usize) -> PyResult<Self> {
        let len: Py_ssize_t = capacity
            .try_into()
            .map_err(|_| PyValueError::new_err("list len out of range"))?;
        unsafe {
            let ptr = PyList_New(len);
            let list: Py<PyList> = Py::from_owned_ptr(py, ptr);
            Ok(Self { len, list, counter: 0 })
        }
    }

    pub fn push(&mut self, item: PyObject) -> PyResult<()> {
        let ptr = self.list.as_ptr();
        unsafe {
            match PyList_SetItem(ptr, self.counter, item.into_ptr()) {
                0 => {
                    self.counter += 1;
                    Ok(())
                }
                _ => Err(PyValueError::new_err("push() exceeded list capacity")),
            }
        }
    }

    pub fn complete(self, py: Python) -> Py<PyList> {
        match self.counter.cmp(&self.len) {
            // we've filled the list, return it
            Ordering::Equal => self.list,
            // we haven't yet filled the list, return a slice
            Ordering::Less => unsafe {
                let ptr = self.list.as_ptr();
                let slice_ptr = PyList_GetSlice(ptr, 0, self.counter);
                Py::from_owned_ptr(py, slice_ptr)
            },
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded list capacity"),
        }
    }
}

pub fn list_as_tuple<'py>(py: Python<'py>, list: &'py PyList) -> &'py PyTuple {
    let py_tuple: Py<PyTuple> = unsafe {
        let ptr = list.as_ptr();
        let tuple_ptr = PyList_AsTuple(ptr);
        Py::from_owned_ptr(py, tuple_ptr)
    };
    py_tuple.into_ref(py)
}

pub struct PyTupleBuilder {
    len: Py_ssize_t,
    tuple: Py<PyTuple>,
    counter: Py_ssize_t,
}

impl PyTupleBuilder {
    pub fn with_capacity(py: Python, capacity: usize) -> PyResult<Self> {
        let len: Py_ssize_t = capacity
            .try_into()
            .map_err(|_| PyValueError::new_err("tuple len out of range"))?;
        unsafe {
            let ptr = PyTuple_New(len);
            let tuple: Py<PyTuple> = Py::from_owned_ptr(py, ptr);
            Ok(Self { len, tuple, counter: 0 })
        }
    }

    pub fn push(&mut self, item: PyObject) -> PyResult<()> {
        let ptr = self.tuple.as_ptr();
        unsafe {
            match PyTuple_SetItem(ptr, self.counter, item.into_ptr()) {
                0 => {
                    self.counter += 1;
                    Ok(())
                }
                _ => Err(PyValueError::new_err("push() exceeded tuple capacity")),
            }
        }
    }

    pub fn complete(self, py: Python) -> Py<PyTuple> {
        match self.counter.cmp(&self.len) {
            // we've filled the tuple, return it
            Ordering::Equal => self.tuple,
            // we haven't yet filled the tuple, return a slice
            Ordering::Less => unsafe {
                let ptr = self.tuple.as_ptr();
                let slice_ptr = PyTuple_GetSlice(ptr, 0, self.counter);
                Py::from_owned_ptr(py, slice_ptr)
            },
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded tuple capacity"),
        }
    }
}
