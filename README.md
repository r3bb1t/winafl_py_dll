# winafl_py_dll
This code provides simple integration with your python code for generating fuzzing inputs for fuzzing with WinAFL.
It was intended to be as simple as it can be, so the people who are not familiar with Rust could still use and adjust it to 
their own needs by only editing the python code.


## Short introduction
This project itself, is a simple [custom mutator for WinAFL](https://github.com/googleprojectzero/winafl#custom-mutators) written in Rust.
Generating dll is as simple as follows: <pre>cargo build --release</pre>


## How it works
Generated dll expects a python script which **must** meet two conditions: *having a function named main* and *returning the bytes object*. Dll will read your python script and will feed it's input right into the fuzzer.


## Customization
By default, dll searches for the file named "fuzz.py" in the curren't user's directory and uses it to get fuzzing results from.
You can provide your own custom path by setting up the enviroment varible "winafl_py_script" and providing it the path of the python script which you would like to use.

Also, by default, function dll_mutate_testcase always returns 1 to skip the rest of mutations.
If you don't want this behaviour, you need to set the "winafl_py_dont_skip_mutations" enviroment variable with 1.

**Note:** for sake of speed, customization can be performed only once, during startup.

## Examples
In the "examples" directory there are some scripts, that can be used for better understanding what kind of files does this mutator expect.

## Ps
Despite this project's flexibility, it's better to write your code as shown [here](https://pyo3.rs/main/python_from_rust.html?highlight=path#include-multiple-python-files).