use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use std::collections::HashSet;
use rocksdb::{DB};

#[pyclass]
struct MyClass {
    db: DB,
}

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<HashSet<String>> {
//     let mut out = HashSet::new();
//     out.insert((a + b).to_string());
//     Ok(out)
// }

#[pyfunction]
fn open(path: String) -> MyClass {
    MyClass { db: DB::open_default(path).unwrap() }
}

#[pyfunction]
fn get(s: &MyClass, k: &[u8]) -> Option<Vec<u8>> {
	   // let db = DB::open_default(path).unwrap();
	   // db.put(b"my key", b"my valuefux").unwrap();
// Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
    match s.db.get(k) {
        Ok(Some(v)) => Some(v),
        Ok(None) => None,
        Err(_e) => panic!("fux!!"),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_class::<MyClass>()?;

    Ok(())
}
