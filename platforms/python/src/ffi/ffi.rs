use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyUnicode};
use pyo3::{wrap_pyfunction, wrap_pymodule};
use std::thread;
use syft::message::SyftMessage;
use syft::worker::start_on_runtime;

// the module will be syft but with a mixed python project it becomes syft.syft
// so this needs to be re-exported from a __init__.py file with: from .syft import *
#[pymodule]
fn syft(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(message))?;
    m.add_wrapped(wrap_pymodule!(node))?;
    Ok(())
}

#[pymodule]
fn message(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(run_class_method_message))?;
    Ok(())
}

#[pymodule]
fn node(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(start))?;
    Ok(())
}

#[pyfunction]
fn start() -> PyResult<()> {
    thread::spawn(move || {
        let result = start_on_runtime();
        match result {
            Ok(_) => println!("gRPC Server thread finished"),
            Err(err) => println!("gRPC Server thread failed with error. {}", err),
        }
    });

    Ok(())
}

#[pyfunction]
fn run_class_method_message(
    target_addr: &PyUnicode,
    capability_name: &PyUnicode,
    py_bytes: &PyBytes,
) -> PyResult<std::vec::Vec<u8>> {
    println!(
        "Rust got Python Request {:?} {:?} {:?}",
        target_addr, capability_name, py_bytes
    );

    // deserialize
    let request: SyftMessage;
    request = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");
    println!("Rust deserialized message {:?}", request);

    // serialize
    let mut response_bytes = vec![];
    to_bytes(&request, &mut response_bytes).expect("Rust Failed to encode message");

    println!("Rust sending back message as bytes {:?}", request);

    return Ok(response_bytes);
}

/// Encodes the message to a `Vec<u8>`.
pub fn to_bytes<M: prost::Message>(
    message: &M,
    buf: &mut Vec<u8>,
) -> Result<(), prost::EncodeError> {
    buf.reserve(message.encoded_len());
    return message.encode(buf);
}

// Decodes an message from the buffer.
pub fn from_bytes<M: prost::Message + std::default::Default>(
    buf: &[u8],
) -> Result<M, prost::DecodeError> {
    let msg = prost::Message::decode(buf);
    return msg;
}
