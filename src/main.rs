use pyo3::prelude::*;
use pyo3_ffi::c_str;

fn main() {
    pyo3::prepare_freethreaded_python();

    let add_code = c_str!(
        r#"
import sys

print (sys.version)
	
def test(d:str) -> str:
    print (d)
    return d
	
"#
    );

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            //
            py,
            add_code,
            c_str!(""),
            c_str!(""), //
        )
        .expect("Module code should be loaded");

        let fn_test = module.getattr("test").expect("test should be there");
        
		
		
        let result_value: String = fn_test
            .call1(("asdf",))
            .expect("result expected")
            .extract()
            .expect("value expected");

        println!("{}", result_value)
    });
}
