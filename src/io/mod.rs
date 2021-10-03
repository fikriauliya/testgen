use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

use crate::problemspec::generator::Generator;

use super::problemspec::spec::*;
use super::testspec::spec::*;

pub fn generate_inputs<T>(output_folder: String)
where
    T: ProblemSpec + TestSpec<T>,
{
    let specs = T::test_cases();
    println!("Generating test cases...");
    println!("[ OFFICIAL TEST CASES ]");

    create_dir(&output_folder);
    let base_path = Path::new(&output_folder);

    for (i, spec) in specs.iter().enumerate() {
        if let Err(errors) = spec.constraints() {
            println!("  {}: FAILED", i + 1);
            println!("    Reasons:");
            //TODO: print the variable values
            for error in errors {
                println!("    * Expected: {}", error);
            }
        } else {
            let output = spec.input_format().generate();
            let path = base_path.join(format!("{}.in", i + 1));
            let mut file = File::create(path).unwrap();
            file.write_all(output.as_bytes()).unwrap();

            println!("  {}: OK", i + 1);
        }
    }
}
