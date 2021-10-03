use crate::problemspec::generator::Generator;

use super::problemspec::spec::*;
use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Pahlevi Fikri Auliya <pahlevi.fikri.auliya@gmail.com>"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Generate(GenerateCommand),
    Grade(GradeCommand),
}

#[derive(Clap)]
struct GenerateCommand {
    #[clap(short, long, default_value = "tc")]
    output: String,

    #[clap(short, long)]
    solution: Option<String>,

    #[clap(long, default_value = "0")]
    seed: usize,
}

#[derive(Clap)]
struct GradeCommand {
    #[clap(short, long, default_value = "tc")]
    output: String,

    #[clap(short, long, default_value = "./solution")]
    solution: String,
}

pub fn run(spec: Box<dyn ProblemSpec>) {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Generate(g) => {
            println!("Generating testcases...");
            let output = spec.input_format().generate();
            println!("{}", output);

            if let Err(errors) = spec.constraints() {
                println!("Constraints errors");
                println!("{:?}", errors);
            }
        }
        SubCommand::Grade(g) => {
            println!("Grading...");
        }
    }
}
