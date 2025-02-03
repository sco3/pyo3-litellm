use common_macros::hash_map;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_ffi::c_str;

fn main() {
    pyo3::prepare_freethreaded_python();

    let code = c_str!(
        r#"
import sys

print (sys.version)
	
def test(d:list) -> list:
    return {"a":1}
	
"#
    );

    let msgs = vec![
        hash_map! {
            "role"=>"system",
            "content"=>"Help  to answer geographical questions."
        },
        hash_map! {
            "role"=>"user",
            "content"=> "What is the capital of Italy?",
        },
    ];

    println!("{:?}", msgs);

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py, //
            code,
            c_str!(""),
            c_str!(""), //
        )
        .expect("Module code should be loaded");

        let fn_test = module //
            .getattr("test")
            .expect("test should be there");

        let r = fn_test //
            .call1((msgs,));

        if let Ok(dict) = r {
            println!("dict: {}", dict);
        }
    });
}
