use std::{path::Path, fs::File, io::{Read, stdin, stdout, sink, BufWriter}, str::FromStr, error::Error};
use anyhow::bail;
use clap::*;
use derive_more::IsVariant;
use crate::{*, string::Symbol, io_game::IOGameEnum, cli::run::*};

mod run;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct CliInternal {
    /// Path to an input file. If unspecified, the standard input is used.
    #[clap(short, long, value_name("PATH"))] input: Option<String>,

    /// Path to an output file. If unspecified, the standard output is used. If used with no parameter, the output is discarded.
    #[clap(short, long, value_name("PATH"))] output: Option<Option<String>>,

    /// Output format
    #[clap(short, long, arg_enum, default_value_t)] format: Format,

    /// Maximum number of MKBSC iterations
    #[clap(short = 'm', long = "mkbsc",
        parse(try_from_str = parse_mkbsc),
        default_value = "_default",
        default_missing_value = "_missing",
        value_name("ITERATIONS")
    )] max_iterations: Option<Option<u64>>,

    /// Apply the KBSC to the output
    #[clap(short, long)] kbsc: bool,

    /// Project the output onto the specified agent
    #[clap(short, long, value_name("AGENT"))] project: Option<String>,

    /// Do not check for isomorphism.
    #[clap(long, visible_alias("ni"))] no_iso_check: bool,

    /// Discard the structure of the game when expanding.
    #[clap(long, visible_alias("ns"))] no_structure: bool,

    /// Find all strategies instead of just one.
    #[clap(short = 'a', long)] find_all: bool,

    /// Perform a transformation and output the result. This is implied if unspecified.
    #[clap(short, long, conflicts_with(
        "find-all"
    ))] transform: bool,

    /// Synthesize strategies. The game will be expanded until at least one strategy is found, or until further expansions lead to isomorphic games.
    #[clap(short, long, conflicts_with_all(&[
        "output", "format",
        "kbsc", "project",
        "no-iso-check", "no-structure",
        "transform"
    ]))] synthesize: bool,

    /// Only output the results.
    #[clap(short, long)] quiet: bool,

    /// Also output intermediate expansions.
    #[clap(short, long, conflicts_with("quiet"))] verbose: bool
}

fn parse_mkbsc(s: &str) -> Result<Option<Option<u64>>, impl Error> {
    match s {
        "_default" => Ok(None),
        "_missing" => Ok(Some(None)),
        "." => Ok(Some(Some(u64::MAX))),
        s => FromStr::from_str(s).map(|x| Some(Some(x)))
    }
}

#[derive(Debug, Clone)]
pub struct Cli {
    input: Input,
    output: Output,

    action: Action,
    verbosity: Verbosity
}

#[derive(Debug, Clone, IsVariant)]
pub enum Input {
    StdIn,
    File(Box<Path>)
}

#[derive(Debug, Clone, IsVariant)]
pub enum Output {
    StdOut,
    File(Box<Path>),
    None
}

#[derive(Debug, Clone, IsVariant)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose
}

#[derive(Debug, Clone, IsVariant)]
pub enum Action {
    Transform(TransformAction),
    Synthesize(SynthesizeAction)
}

#[derive(Debug, Clone)]
pub struct TransformAction {
    output_format: Format,
    max_iterations: u64,
    kbsc: bool,
    project: Option<Symbol>,
    check_iso: bool,
    keep_structure: bool
}

#[derive(Debug, Clone)]
pub struct SynthesizeAction {
    max_iterations: u64,
    find_all: bool
}

#[derive(ArgEnum, Debug, Clone, IsVariant, SmartDefault)]
pub enum Format {
    #[default] Default,
    Tikz
}

pub fn parse() -> anyhow::Result<Cli> {
    let c = CliInternal::parse();

    let input = match c.input {
        None => Input::StdIn,
        Some(s) => Input::File(Path::new(&s).into())
    };

    let output = match c.output {
        None => Output::StdOut,
        Some(None) => Output::None,
        Some(Some(s)) => Output::File(Path::new(&s).into())
    };

    let action = match (c.transform, c.synthesize) {
        (_, false) => Action::Transform(TransformAction {
            output_format: c.format,
            max_iterations: match c.max_iterations {
                None => 0,
                Some(None) => 1,
                Some(Some(n)) => n as u64
            },
            kbsc: c.kbsc,
            project: c.project.map(|s| intern(&s)),
            check_iso: !c.no_iso_check,
            keep_structure: !c.no_structure
        }),
        (false, true) => Action::Synthesize(SynthesizeAction {
            max_iterations: match c.max_iterations {
                Some(Some(n)) => n as u64,
                Some(None) => bail!("--mkbsc requires an argument when used with --synthesize"),
                _ => u64::MAX
            },
            find_all: c.find_all
        }),
        _ => unreachable!()
    };

    let verbosity = match (c.quiet, c.verbose) {
        (false, false) => Verbosity::Normal,
        (true, false) => Verbosity::Quiet,
        (false, true) => Verbosity::Verbose,
        _ => unreachable!(),
    };
    
    Ok(Cli {
        input,
        output,
        action,
        verbosity
    })
}

pub fn run(cli: &Cli) -> anyhow::Result<()> {
    let io_game = match &cli.input {
        Input::StdIn => read_input(&mut stdin())?,
        Input::File(path) => read_input(&mut File::open(path)?)?
    };

    let runner = RunnerEnum::new(io_game, cli.verbosity.clone());
    
    let before = SystemTime::now();

    match &cli.output {
        Output::StdOut => runner.run(&cli.action, &mut BufWriter::new(
            stdout()
        ))?,
        Output::File(path) => runner.run(&cli.action, &mut BufWriter::new(
            File::create(path.as_ref())?
        ))?,
        Output::None => runner.run(&cli.action, &mut sink())?,
    }
    
    let elapsed = before.elapsed()?;

    if !cli.verbosity.is_quiet() {
        println!("action: {:?}", elapsed);
    }

    Ok(())
}

pub fn read_input(input: &mut impl Read) -> anyhow::Result<IOGameEnum> {
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let parsed = parser::parse(&contents)?;

    let g = IOGameEnum::new(parsed)?;

    Ok(g)
}
