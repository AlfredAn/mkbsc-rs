use std::{path::Path, fs::File, io::Read};

use clap::*;

use crate::{parser, io_game::IOGameEnum, cli::run::{RunnerEnum, RunnerTrait}, string::{Symbol, intern}};

mod run;

#[derive(Debug)]
pub struct Cli {
    input_file: Box<Path>,
    action: Action,
    post: PostProcessing
}

#[derive(Debug, Clone)]
pub enum Action {
    None,
    MKBSC(usize)
}

#[derive(Debug, Clone, Default)]
pub struct PostProcessing {
    project: Option<Symbol>,
    kbsc: bool
}

pub fn parse() -> Cli {
    let matches = command!()
        .subcommand(
            Command::new("project")
                .arg(
                    arg!(<AGT>)
                )
                .arg(
                    arg!(-k --kbsc)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("kbsc")
        )
        .subcommand(
            Command::new("mkbsc")
                .arg(
                    arg!(<N>)
                        .validator(|s| s.parse::<usize>())
                        .required(false)
                        .default_value("1")
                )
                .arg(
                    arg!(-p --project <AGT>)
                        .required(false)
                )
                .arg(
                    arg!(-k --kbsc)
                        .required(false)
                )
        )
        .arg(
            arg!(<INPUT_FILE>)
        )
        .get_matches();

    let input_file = matches.value_of("INPUT_FILE")
        .map(|f| Path::new(f).into())
        .unwrap();

    let (action, post) = match matches.subcommand() {
        None => (Action::None, Default::default()),
        Some(("project", m)) => (
            Action::None,
            PostProcessing {
                project: m.value_of("AGT")
                    .map(|m| intern(m)),
                kbsc: m.is_present("kbsc")
            }),
        Some(("kbsc", _)) => (
            Action::None,
            PostProcessing {
                kbsc: true,
                ..Default::default()
            }),
        Some(("mkbsc", m)) => (
            Action::MKBSC(
                m.value_of("N")
                    .map(|s| s.parse())
                    .unwrap().unwrap()
            ),
            PostProcessing {
                project: m.value_of("project")
                    .map(|m| intern(m)),
                kbsc: m.is_present("kbsc")
            }
        ),
        _ => unreachable!()
    };

    Cli {
        input_file,
        action,
        post
    }
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let io_game = read_input(&cli.input_file)?;

    let mut runner = RunnerEnum::new(io_game);

    runner.run(&cli.action, &cli.post)?;

    Ok(())
}

fn read_input(file: &Path) -> anyhow::Result<IOGameEnum> {
    let mut file = File::open(file)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let parsed = parser::parse(&contents)?;

    let g = IOGameEnum::new(parsed)?;

    Ok(g)
}
