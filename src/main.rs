use common_macros::hash_map;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyInt;
use pyo3::types::PyString;
use pyo3::types::PyTuple;
use pyo3_ffi::c_str;
use std::time::Instant;

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
        let start = Instant::now();
        if let Ok(r) = r {
            if let Ok(dict) = r.downcast::<PyDict>() {
                println!("dict! {}", dict);
                println!("items: {}", dict.items());
                for item in dict.items() {
                    println!("item: {}", item.get_type());
                    if let Ok(tuple) = item.downcast::<PyTuple>() {
                        println!("tuple: {}", tuple);
                        for idx in 0..tuple.len() {
                            let v = tuple.get_item(idx);
                            println!("value{}: {:?}", idx, v);
                            if let Ok(v) = v {
                                if let Ok(s) = v.downcast::<PyString>() {
                                    println!("str{}: {}", idx, s);
                                }
                                if let Ok(i) = v.downcast::<PyInt>() {
                                    println!("str{}: {}", idx, i);
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("Time: {} mks", start.elapsed().as_micros());
    });
}
