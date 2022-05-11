use std::{path::Path, fs::File, io::Read};

use clap::*;

use crate::{*, string::{Symbol, intern}, io_game::IOGameEnum};

use self::run::{RunnerEnum, RunnerTrait};

mod run;

#[derive(Debug)]
pub struct Cli {
    input_file: Option<Box<Path>>,
    action: Action,
}

#[derive(Debug, Clone)]
pub enum Action {
    MKBSC(MKBSCAction),
    Synth,
    // GridPursuit(usize, usize)
}

#[derive(Debug, Clone, SmartDefault)]
pub enum Format {
    #[default]
    Default,
    Tikz
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
    keep_structure: bool,

    format: Format
}

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
                .arg(arg!(-f --format <FORMAT>).required(false))
        )
        .subcommand(
            Command::new("synthesize")
                .visible_short_flag_alias('s')
        )
        .arg(arg!(<INPUT_FILE>))
        .subcommand_required(true)
        .get_matches();

    let input_file = matches.value_of("INPUT_FILE")
        .map(|f| Path::new(f).into());

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
                format: match m.value_of("format") {
                    Some("default") => Format::Default,
                    Some("tikz") => Format::Tikz,
                    None => Default::default(),
                    _ => panic!()
                },
                ..Default::default()
            }
        ),
        Some(("synthesize", _)) => Action::Synth,
        /*Some(("grid_pursuit", m)) => Action::GridPursuit(
            m.value_of("X").unwrap().parse().unwrap(),
            m.value_of("Y").unwrap().parse().unwrap()
        ),*/
        _ => unreachable!()
    };

    Cli {
        input_file,
        action
    }
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    println!("{:?}", cli);

    let io_game = cli.input_file.as_ref()
        .map(|path| read_input(&path).unwrap());

    if let Some(io_game) = io_game {
        let mut runner = RunnerEnum::new(
            io_game,
            if let Action::MKBSC(a) = &cli.action {
                a.keep_structure
            } else {
                true
            });
        
        let before = SystemTime::now();
        runner.run(&cli.action)?;
        let elapsed = before.elapsed()?;

        println!("action: {:?}", elapsed);
    } else {
        match cli.action {
            /*Action::GridPursuit(x, y) => {

            },*/
            _ => unimplemented!()
        }
    }

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
