#![cfg(windows)]

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CODE: Mutex<String> = Mutex::new(String::new());
    static ref SKIP_MUTATIONS: Mutex<bool> = Mutex::new(true);
}

use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict};

use std::error::Error;
use std::slice;
use std::sync::Mutex;

use winapi::shared::minwindef::{self, BOOL, DWORD, HINSTANCE, LPVOID};

const ENV_SCRIPT_PATH: &str = "winafl_py_script";
const ENV_DONT_SKIP_MUTATIONS: &str = "winafl_py_dont_skip_mutations";
const DEFAULT_SCRIPT_NAME: &str = "fuzz.py";

fn init() -> Result<(), Box<dyn Error>> {
    if let Ok(path) = std::env::var(ENV_SCRIPT_PATH) {
        let msg = format!("Using the following python script: {}", &path);
        dbg!(msg);
        *CODE.lock()? = std::fs::read_to_string(&path)?;
    } else {
        *CODE.lock()? = std::fs::read_to_string(DEFAULT_SCRIPT_NAME)?;
    }
    if let Ok(var) = std::env::var(ENV_DONT_SKIP_MUTATIONS) {
        if var == "1" {
            dbg!("Not skipping mutations");
            *SKIP_MUTATIONS.lock()? = false;
        }
    }

    Ok(())
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => {
            // Setting up the global variables
            init().unwrap();
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    minwindef::TRUE
}

type CommonFuzzStuff = unsafe extern "C" fn(argv: u64, newbuf: *mut u8, len: u32) -> u8;

#[no_mangle]
pub unsafe fn dll_mutate_testcase(
    _argv: u64,
    input_buffer: *const u8,
    len: u32,
    common_fuzz_stuff: CommonFuzzStuff,
) -> u8 {
    let buf = slice::from_raw_parts(input_buffer, len as usize);
    let mut input = get_testcase_from_python(buf, len)
        .expect("An error occured while acquiring testcase from python");

    common_fuzz_stuff(_argv, input.as_mut_ptr(), input.len() as u32);

    if *SKIP_MUTATIONS.lock().unwrap() { 1 } else { 0 }
}

/// Calls main() from python's script and tries to get it's return value as a vector
///
/// Function is called as follows:
///  
/// main(buf, len, buf = buf, len = len)
pub fn get_testcase_from_python(buf: &[u8], len: u32) -> PyResult<Vec<u8>> {
    Python::with_gil(|py| {
        let _buf = PyByteArray::new(py, buf);
        let kwargs = PyDict::new(py);
        kwargs.set_item("buf", _buf)?;
        kwargs.set_item("len", len)?;

        let fun: Py<PyAny> = PyModule::from_code(py, &CODE.lock().unwrap(), "", "")?
            .getattr("main")?
            .into();

        let result = fun
            .call(py, (_buf, len), Some(kwargs))
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            })
            .unwrap();

        let r: Vec<u8> = result.extract(py)?;
        Ok(r)
    })
}
