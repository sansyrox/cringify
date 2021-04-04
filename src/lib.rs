use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn cringify(main_string: &str, prefix: &str) -> String {

    let mut index = 0;
    let final_string = main_string
        .chars()
        .map(|letter| {
            let return_character = letter;
            if !return_character.is_ascii_alphabetic() {
                index+=1;
                return return_character;
            }

            let cringe_character;
            if index%2==1 {
                cringe_character = return_character.to_uppercase().next().unwrap();
            } else {
                cringe_character = return_character.to_lowercase().next().unwrap();
            }
            index += 1;
            return cringe_character
    }).collect::<String>();

    if prefix!="" {
        return format!("{} {}", prefix, final_string.as_str());
    }
    else {
        return final_string;
    }
}


#[pymodule]
fn cringify_python(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cringify, m)?)?;
    Ok(())
}
