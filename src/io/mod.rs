use std::fs::{create_dir, remove_dir, remove_dir_all, File};
use std::io::prelude::*;
use std::io::Write;
use std::ops::Shl;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::problemspec::generator::Generator;

use super::problemspec::spec::*;
use super::testspec::spec::*;

pub fn generate_inputs_outputs<T>(
    base_folder: String,
    solution_command: Option<String>,
    seed: usize,
) where
    T: ProblemSpec + TestSpec<T>,
{
    let specs = T::test_cases();
    println!("Generating test cases...");
    println!("[ OFFICIAL TEST CASES ]");

    let base_folder = Path::new(&base_folder);
    if base_folder.exists() {
        remove_dir_all(&base_folder).expect("Failed to remove folder");
    }
    create_dir(&base_folder).expect("Failed to create output folder");
    for (i, spec) in specs.iter().enumerate() {
        if let Err(errors) = spec.constraints() {
            println!("  {}: FAILED", i + 1);
            println!("    Reasons:");
            //TODO: print the variable values
            for error in errors {
                println!("    * Expected: {}", error);
            }
        } else {
            let input = spec.input_format().generate();
            let input_path = base_folder.join(format!("{}.in", i + 1));
            let mut input_file = File::create(input_path).unwrap();
            input_file.write_all(input.as_bytes()).unwrap();

            if let Some(solution_command) = &solution_command {
                let output_path = base_folder.join(format!("{}.out", i + 1));
                let mut output_file = File::create(output_path).unwrap();

                let args = shlex::split(solution_command).unwrap();
                let mut cmd = Command::new(&args[0]);
                let mut child = cmd
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .args(args.iter().skip(1))
                    .spawn()
                    .expect("Failed to execute solution");
                child
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(input.as_bytes())
                    .unwrap();

                let output = child.wait_with_output().unwrap();
                output_file.write_all(output.stdout.as_slice()).unwrap();
            }

            println!("  {}: OK", i + 1);
        }
    }
}
