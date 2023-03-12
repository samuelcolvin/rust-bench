use std::cmp::Ordering;

use pyo3::{AsPyPointer};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::ffi;
use pyo3::types::{PyList, PyTuple};


pub struct PyListBuilder {
    len: ffi::Py_ssize_t,
    list: Py<PyList>,
    counter: ffi::Py_ssize_t,
}

impl PyListBuilder {
    pub fn with_capacity(py: Python, capacity: usize) -> PyResult<Self> {
        let len: ffi::Py_ssize_t = capacity
            .try_into()
            .map_err(|_| PyValueError::new_err("list len out of range"))?;
        unsafe {
            let ptr = ffi::PyList_New(len);
            let list: Py<PyList> = Py::from_owned_ptr(py, ptr);
            Ok(Self { len, list, counter: 0 })
        }
    }

    pub fn push(&mut self, py: Python, item: impl ToPyObject) -> PyResult<()> {
        let ptr = self.list.as_ptr();
        unsafe {
            match ffi::PyList_SetItem(ptr, self.counter, item.to_object(py).into_ptr()) {
                0 => {
                    self.counter += 1;
                    Ok(())
                }
                _ => Err(PyValueError::new_err("push() exceeded list capacity")),
            }
        }
    }

    pub fn push_alt(&mut self, py: Python, item: impl ToPyObject) -> PyResult<()> {
        if self.counter == self.len {
            Err(PyValueError::new_err("push() exceeded list capacity"))
        } else {
            unsafe {
                let obj_ptr = item.to_object(py).into_ptr();

                #[cfg(not(Py_LIMITED_API))]
                ffi::PyList_SET_ITEM(self.list.as_ptr(), self.counter, obj_ptr);
                #[cfg(Py_LIMITED_API)]
                ffi::PyList_SetItem(self.list.as_ptr(), self.counter, obj_ptr);
            }
            self.counter += 1;
            Ok(())
        }
    }

    pub fn get(self, py: Python) -> PyResult<&PyList> {
        match self.counter.cmp(&self.len) {
            // we've filled the list, return it
            Ordering::Equal => Ok(self.list.into_ref(py)),
            // we haven't yet filled the list, return a slice
            Ordering::Less => Err(PyValueError::new_err("list not yet complete")),
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded list capacity"),
        }
    }

    pub fn get_incomplete(self, py: Python) -> &PyList {
        match self.counter.cmp(&self.len) {
            // we've filled the list, return it
            Ordering::Equal => self.list.into_ref(py),
            // we haven't yet filled the list, return a slice
            Ordering::Less => unsafe {
                let ptr = self.list.as_ptr();
                let slice_ptr = ffi::PyList_GetSlice(ptr, 0, self.counter);
                let py_list: Py<PyList> = Py::from_owned_ptr(py, slice_ptr);
                py_list.into_ref(py)
            },
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded list capacity"),
        }
    }
}

pub fn list_as_tuple<'py>(py: Python<'py>, list: &'py PyList) -> &'py PyTuple {
    let py_tuple: Py<PyTuple> = unsafe {
        let ptr = list.as_ptr();
        let tuple_ptr = ffi::PyList_AsTuple(ptr);
        Py::from_owned_ptr(py, tuple_ptr)
    };
    py_tuple.into_ref(py)
}

pub struct PyTupleBuilder {
    len: ffi::Py_ssize_t,
    tuple: Py<PyTuple>,
    counter: ffi::Py_ssize_t,
}

impl PyTupleBuilder {
    pub fn with_capacity(py: Python, capacity: usize) -> PyResult<Self> {
        let len: ffi::Py_ssize_t = capacity
            .try_into()
            .map_err(|_| PyValueError::new_err("tuple len out of range"))?;
        unsafe {
            let ptr = ffi::PyTuple_New(len);
            let tuple: Py<PyTuple> = Py::from_owned_ptr(py, ptr);
            Ok(Self { len, tuple, counter: 0 })
        }
    }

    pub fn push(&mut self, py: Python, item: impl ToPyObject) -> PyResult<()> {
        let ptr = self.tuple.as_ptr();
        unsafe {
            match ffi::PyTuple_SetItem(ptr, self.counter, item.to_object(py).into_ptr()) {
                0 => {
                    self.counter += 1;
                    Ok(())
                }
                _ => Err(PyValueError::new_err("push() exceeded tuple capacity")),
            }
        }
    }
    pub fn get(self, py: Python) -> PyResult<&PyTuple> {
        match self.counter.cmp(&self.len) {
            // we've filled the tuple, return it
            Ordering::Equal => Ok(self.tuple.into_ref(py)),
            // we haven't yet filled the tuple, error
            Ordering::Less => Err(PyValueError::new_err("tuple not yet filled")),
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded tuple capacity"),
        }
    }

    pub fn get_incomplete(self, py: Python) -> &PyTuple {
        match self.counter.cmp(&self.len) {
            // we've filled the tuple, return it
            Ordering::Equal => self.tuple.into_ref(py),
            // we haven't yet filled the tuple, return a slice
            Ordering::Less => unsafe {
                let ptr = self.tuple.as_ptr();
                let slice_ptr = ffi::PyTuple_GetSlice(ptr, 0, self.counter);
                let py_tuple: Py<PyTuple> = Py::from_owned_ptr(py, slice_ptr);
                py_tuple.into_ref(py)
            },
            // shouldn't happen
            Ordering::Greater => unreachable!("complete() exceeded tuple capacity"),
        }
    }
}
