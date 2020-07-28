use std::thread;
use syft::capabilities::message::SyftMessage;
use syft::worker::{add_capability, start_on_runtime, Callable, Callback};

// #[pymodule]
// fn syft(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pymodule!(message))?;
//     m.add_wrapped(wrap_pymodule!(node))?;
//     Ok(())
// }

// #[pymodule]
// fn message(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(run_class_method_message))?;
//     m.add_wrapped(wrap_pyfunction!(say_hello))?;
//     Ok(())
// }

// #[pymodule]
// fn node(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(start))?;
//     m.add_wrapped(wrap_pyfunction!(register))?;
//     m.add_wrapped(wrap_pyfunction!(connect))?;
//     m.add_wrapped(wrap_pyfunction!(request_capabilities))?;
//     Ok(())
// }

// #[no_mangle]
// pub extern "C" fn start(address: *const c_char) -> *mut c_char {
//     // Create the initial map and store it in `GLOBAL_REGISTRY`.
//     //let initial_map = HashMap::new();
//     //GLOBAL_REGISTRY.set(Mutex::new(initial_map));

//     println!("Rust got address for binding {:?}", address);

//     let c_str = unsafe { CStr::from_ptr(address) };
//     let ipaddr = match c_str.to_str() {
//         Err(_) => "error",
//         Ok(string) => string,
//     };

//     println!("Rust extracted address for binding {}", ipaddr);

//     let str_copy: std::string::String = ipaddr.to_owned();

//     thread::spawn(move || {
//         let _ = listen(&str_copy);
//     });

//     return CString::new("Start ".to_owned()).unwrap().into_raw();
// }

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start(c_str_iface: *const c_char, port: u32) {
    println!(
        "Rust got interface and port for binding {:?}:{:?}",
        c_str_iface, port
    );
    let c_str = unsafe { CStr::from_ptr(c_str_iface) };

    // [::] means all ipv6 interfaces like 0.0.0.0 in ipv4
    let mut default_iface = "[::]".to_owned();
    if let Ok(iface) = c_str.to_str() {
        println!("Rust extracted iface {:?}", iface);
        default_iface = iface.to_owned();
    }

    thread::spawn(move || {
        let result = start_on_runtime(default_iface.clone(), port);
        match result {
            Ok(_) => println!("gRPC Server thread finished"),
            Err(err) => println!("gRPC Server thread failed with error. {}", err),
        }
    });

    println!("Rust finished running start");
}

#[no_mangle]
pub extern "C" fn rust_hello(to: *const c_char) -> *mut c_char {
    println!("Hello from Rust!");
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

// #[pyfunction]
// fn start(local_iface: &PyUnicode, port: u32) -> PyResult<()> {
//     let iface: String = local_iface.extract()?;
//     thread::spawn(move || {
//         let result = start_on_runtime(iface.clone(), port);
//         match result {
//             Ok(_) => println!("gRPC Server thread finished"),
//             Err(err) => println!("gRPC Server thread failed with error. {}", err),
//         }
//     });

//     Ok(())
// }

//struct SwiftCallback(SwiftObject);

// impl Callable for PyCallback {
//     fn execute(&self, message: SyftMessage) -> Result<SyftMessage, Box<dyn std::error::Error>> {
//         let mut message_bytes = vec![];
//         to_bytes(&message, &mut message_bytes).expect("Rust Failed to encode message");

//         let gil = Python::acquire_gil();
//         let py = gil.python();
//         let py_bytes = PyBytes::new(py, message_bytes.as_slice());

//         let py_result: PyResult<PyObject> = self.0.call1(py, (py_bytes,));

//         // lets get the result of the function back into py_bytes
//         let py_bytes: &PyBytes;
//         let response: SyftMessage;

//         if let Ok(result) = py_result {
//             if let Ok(bytes) = result.extract(py) {
//                 py_bytes = bytes;
//                 response = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");
//                 return Ok(response);
//             }
//         };

//         Err(format!(
//             "Failed to execute capability: {} in python",
//             message.capability
//         ))?
//     }
// }

// #[allow(dead_code)]
// #[pyfunction]
// fn register(py_capability_name: &PyUnicode, py_callback: &PyAny) -> PyResult<()> {
//     // bring the function over to the dark side
//     let name: String = py_capability_name.extract()?;
//     let callback: PyObject = py_callback.into();

//     let cb1 = Box::new(PyCallback(callback));
//     let cb = Callback { callable: cb1 };

//     let result = add_capability(name.clone(), cb);
//     match result {
//         Ok(_) => println!("Capability registered: {}", name),
//         Err(err) => println!("Failed to register capability {}. {}", name, err),
//     }

//     Ok(())
// }

// #[pyfunction]
// pub fn connect(target_addr: &PyUnicode) -> PyResult<()> {
//     let addr: String = target_addr.extract()?;
//     syft::client::connect(addr);
//     Ok(())
// }

// #[pyfunction]
// pub fn request_capabilities(target_addr: &PyUnicode) -> PyResult<Vec<String>> {
//     let addr: String = target_addr.extract()?;
//     let response = syft::client::request_capabilities(addr);
//     match response {
//         Ok(caps) => Ok(caps),
//         Err(_err) => Err(PyErr::new::<pyo3::exceptions::Exception, _>(
//             "unable to run request_capabilities",
//         )),
//     }
// }

// #[pyfunction]
// pub fn say_hello(target_addr: &PyUnicode, name: &PyUnicode) -> PyResult<String> {
//     let addr: String = target_addr.extract()?;
//     let name: String = name.extract()?;
//     let result = syft::client::say_hello(addr, name).into();
//     Ok(result)
// }

// #[pyfunction]
// fn run_class_method_message(
//     target_addr: &PyUnicode,
//     py_bytes: &PyBytes,
// ) -> PyResult<std::vec::Vec<u8>> {
//     // deserialize
//     let request: SyftMessage;
//     request = from_bytes(py_bytes.as_bytes()).expect("Rust Failed to decode message");

//     let addr: String = target_addr.extract()?;
//     let response = syft::client::run_class_method_message(addr, request);

//     // serialize
//     match response {
//         Ok(message) => {
//             let mut response_bytes = vec![];
//             to_bytes(&message, &mut response_bytes).expect("Rust Failed to encode message");
//             return Ok(response_bytes);
//         }
//         Err(_err) => Err(PyErr::new::<pyo3::exceptions::Exception, _>(
//             "unable to run run_class_method_message",
//         )),
//     }
// }

/// Encodes the message to a `Vec<u8>`.
pub fn to_bytes<M: prost::Message>(
    message: &M,
    buf: &mut Vec<u8>,
) -> Result<(), prost::EncodeError> {
    buf.reserve(message.encoded_len());
    return message.encode(buf);
}

// Decodes an message from the buffer.
pub fn from_bytes<M: prost::Message + Default>(buf: &[u8]) -> Result<M, prost::DecodeError> {
    let msg = prost::Message::decode(buf);
    return msg;
}
