use crate::{
    problemspec::spec::{MultitaskProblemSpec, ProblemSpec},
    testspec::spec::{MultitaskTestSpec, SingletaskTestSpec},
};

use self::{
    io::prepare_folder, sample::GenerateSampleTestCaseError, testcase::GenerateInputOutputError,
};
use thiserror::Error;

mod executor;
mod io;
mod sample;
mod testcase;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Generate Input/Output Error")]
    GenerateInputOutputError(#[from] GenerateInputOutputError),
    #[error("Generate Sample Testcase Error")]
    GenerateSampleTestCaseError(#[from] GenerateSampleTestCaseError),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

pub fn run_singletask<T>(
    base_folder: &str,
    solution_command: Option<&str>,
    seed: u64,
) -> Result<(), RunnerError>
where
    T: SingletaskTestSpec<T> + ProblemSpec<T>,
{
    let base_folder = prepare_folder(base_folder)?;

    println!("[ SAMPLE TEST CASES ]");
    match sample::generate::<T>(base_folder) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("  ❌");
            match &err {
                GenerateSampleTestCaseError::ConstraintsError(errors) => {
                    for error in &errors.messages {
                        println!("    * Expected: {}", error);
                    }
                }
                GenerateSampleTestCaseError::IOError(error) => {
                    println!("    * IO error: {}", error);
                }
            }
            Err(err)
        }
    }?;

    println!();
    println!("[ OFFICIAL TEST CASES ]");
    match testcase::generate::<T>(base_folder, solution_command, seed) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("  ❌");
            match &err {
                GenerateInputOutputError::ConstraintsError(errors) => {
                    for error in &errors.messages {
                        println!("    * Expected: {}", error);
                    }
                }
                GenerateInputOutputError::OutputFormatError(error) => {
                    println!("    * Formatting error: {}", error);
                }
                GenerateInputOutputError::IOError(error) => {
                    println!("    * IO error: {}", error);
                }
            }
            Err(RunnerError::GenerateInputOutputError(err))
        }
    }?;
    Ok(())
}

pub fn run_multitask<T>(
    base_folder: &str,
    solution_command: Option<&str>,
    seed: u64,
) -> Result<(), RunnerError>
where
    T: MultitaskTestSpec<T> + ProblemSpec<T> + MultitaskProblemSpec<T>,
{
    let base_folder = prepare_folder(base_folder)?;

    println!("[ SAMPLE TEST CASES ]");
    match sample::generate_multitask::<T>(base_folder) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("  ❌");
            match &err {
                GenerateSampleTestCaseError::ConstraintsError(errors) => {
                    for error in &errors.messages {
                        println!("    * Expected: {}", error);
                    }
                }
                GenerateSampleTestCaseError::IOError(error) => {
                    println!("    * IO error: {}", error);
                }
            }
            Err(err)
        }
    }?;

    println!();
    println!("[ OFFICIAL TEST CASES ]");
    match testcase::generate_multitask::<T>(base_folder, solution_command, seed) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("  ❌");
            match &err {
                GenerateInputOutputError::ConstraintsError(errors) => {
                    for error in &errors.messages {
                        println!("    * Expected: {}", error);
                    }
                }
                GenerateInputOutputError::OutputFormatError(error) => {
                    println!("    * Formatting error: {}", error);
                }
                GenerateInputOutputError::IOError(error) => {
                    println!("    * IO error: {}", error);
                }
            }
            Err(RunnerError::GenerateInputOutputError(err))
        }
    }?;
    Ok(())
}
