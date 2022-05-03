use std::{path::Path, fs::File, io::Read};

use clap::*;

use crate::{*, string::{Symbol, intern}, io_game::IOGameEnum};

use self::run::{RunnerEnum, RunnerTrait};

mod run;

#[derive(Debug)]
pub struct Cli {
    input_file: Box<Path>,
    action: Action,
}

#[derive(Debug, Clone)]
pub enum Action {
    MKBSC(MKBSCAction),
    Synth(SynthesizeAction)
}

#[derive(Debug, Clone, SmartDefault)]
pub struct MKBSCAction {
    #[default(0)]
    iterations: usize,

    #[default(true)]
    check_convergence: bool,

    project: Option<Symbol>,
    kbsc: bool,

    #[default(false)]
    print_games: bool,

    #[default(true)]
    print_iteration: bool,
    
    #[default(true)]
    print_result: bool,

    #[default(true)]
    print_sizes: bool,

    #[default(true)]
    keep_structure: bool
}

#[derive(Debug, Clone)]
pub struct SynthesizeAction;

pub fn parse() -> Cli {
    let matches = command!()
        .subcommand(
            Command::new("transform")
                .visible_short_flag_alias('t')

                .arg(
                    arg!(-m --mkbsc <N>)
                        .validator(|s| s.parse::<usize>())
                        .required(false)
                        .default_value("0")
                )

                .arg(arg!(-p --project <AGT>).required(false))
                .arg(arg!(-k --kbsc).required(false))
                .arg(arg!(-i --print_iterations).required(false))
                .arg(arg!(-n --no_structure).required(false))
                .arg(arg!(-r --no_result).required(false))
        )
        .subcommand(
            Command::new("synthesize")
                .visible_short_flag_alias('s')
        )

        .arg(
            arg!(<INPUT_FILE>)
        )
        .get_matches();

    let input_file = matches.value_of("INPUT_FILE")
        .map(|f| Path::new(f).into())
        .unwrap();

    let action = match matches.subcommand() {
        Some(("transform", m)) => Action::MKBSC(
            MKBSCAction {
                iterations: m.value_of("mkbsc")
                    .unwrap()
                    .parse()
                    .unwrap(),
                project: m.value_of("project")
                    .map(|m| intern(m)),
                kbsc: m.is_present("kbsc"),
                print_games: m.is_present("print_iterations"),
                keep_structure: !m.is_present("no_structure"),
                print_result: !m.is_present("no_result"),
                ..Default::default()
            }
        ),
        Some(("synthesize", _)) => Action::Synth(SynthesizeAction {}),
        _ => unreachable!()
    };

    Cli {
        input_file,
        action
    }
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let io_game = read_input(&cli.input_file)?;

    let mut runner = RunnerEnum::new(io_game);

    runner.run(&cli.action)?;

    Ok(())
}

pub fn read_input(file: &Path) -> anyhow::Result<IOGameEnum> {
    let mut file = File::open(file)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let parsed = parser::parse(&contents)?;

    let g = IOGameEnum::new(parsed)?;

    Ok(g)
}
