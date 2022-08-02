use pyo3::prelude::*;
use pyo3::types::{PyList, PySet};

fn main() {
    println!("benchmarks");
    let primes: Vec<i32> = vec![
        1, 3, 5, 7, 11, 13, 1779, 83, 89, 97, 101, 103, 107, 109, 111, 199,
    ];
    let gil = Python::acquire_gil();
    let py = gil.python();
    let list = PyList::new(py, &primes);
    let set: &PySet = list.cast_as().unwrap();
    dbg!(set);
}
