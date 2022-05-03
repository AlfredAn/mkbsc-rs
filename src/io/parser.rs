use itertools::iproduct;
use nom::{
    *,
    character::complete::*,
    bytes::complete::*,
    sequence::*,
    combinator::*,
    multi::*,
    branch::*,
    error
};
use nom_supreme::{
    final_parser::{final_parser, self},
    multi::*,
    parser_ext::ParserExt// , error::ErrorTree
};

use super::*;

type E<I> = error::Error<I>;
type R<I, O> = IResult<I, O, E<I>>;

macro_rules! func {
    ($name:ident = $f:expr) => { func!($name = $f, $) };
    ($name:ident = $f:expr, $dol:tt) => {
        macro_rules! $name {
            () => { $f };
            ($dol($arg:expr),+) => { $f($dol($arg),+) }
        }
    };
}

const DEBUG_ENABLED: bool = false;

macro_rules! debug {
    ($s:expr, $i:expr, $f:expr) => {{ |input| {
        let (s, i) = ($s, $i);

        if DEBUG_ENABLED {
            for _ in 0..i*4 { print!(" "); }
            println!("{} {{", s);  
        }

        let result = error::context(s, ($f))(input);

        if DEBUG_ENABLED {
            for _ in 0..i*4 { print!(" "); }
            match &result {
                Ok((_, x)) => println!("}} -> {:?}\n", x),
                Err(_) => println!("}} -> ERR\n")
            }
        }

        result
    }}};
}

fn sp<T>(input: T) -> R<T, T>
where
    T: InputTakeAtPosition + InputLength,
    T::Item: AsChar + Clone
{ multispace0(input) }

fn idt(input: &str) -> R<&str, &str> {
    terminated(
        take_while1(|c: char|
            c.is_alphanumeric()
        ),
        sp
    )(input)
}

fn token<'a>(c: char) -> impl FnMut(&'a str) -> R<&'a str, char> {
    terminated(
        char(c),
        sp
    )
}

fn keyword<'a>(kw: &'a str) -> impl FnMut(&'a str) -> R<&'a str, &'a str> {
    terminated(
        tag_no_case(kw),
        pair(sp, token(':'))
    )
}

fn keyword_nc<'a>(kw: &'a str) -> impl FnMut(&'a str) -> R<&'a str, &'a str> {
    terminated(
        tag_no_case(kw),
        sp
    )
}

fn name(input: &str) -> R<&str, Symbol> {
    map(
        idt,
        intern
    )(input)
}

fn list(input: &str) -> R<&str, Vec<Symbol>> {
    debug!("list", 3, separated_list0(
        token(','),
        debug!("name", 4, name)
    ))(input) 
}

fn agt(input: &str) -> R<&str, Vec<Symbol>> {
    debug!("agt", 2, preceded(
        keyword("agt"),
        list
    ))(input)
}

fn act(input: &str) -> R<&str, Vec<Symbol>> {
    debug!("act", 2, preceded(
        keyword("act"),
        list
    ))(input)
}

fn loc(input: &str) -> R<&str, Vec<Symbol>> {
    debug!("loc", 2, preceded(
        keyword("loc"),
        list
    ))(input)
}

fn reach(input: &str) -> R<&str, Vec<Symbol>> {
    debug!("reach", 2, preceded(
        keyword("reach"),
        list
    ))(input)
}

fn l0(input: &str) -> R<&str, Symbol> {
    debug!("l0", 2, preceded(
        keyword("l0"),
        name
    ))(input)
}

fn obs(input: &str) -> R<&str, (Symbol, Vec<Vec<Symbol>>)> {
    let element = separated_list0(
        token('|'),
        name
    );

    debug!("obs", 2, map(
        tuple((
            keyword_nc("obs"),
            name,
            token(':'),
            separated_list0(
                token(','),
                element
            )
        )),
        |x| (x.1, x.3)
    ))(input)
}

fn delta(input: &str) -> R<&str, Vec<(Symbol, Vec<Symbol>, Symbol)>> {
    func!(options = |f| delimited(
        token('('),
        separated_list0(
            token(','),
            f
        ),
        token(')')
    ));

    let act_single = |input| debug!("act_single", 5,
        many1(name)
    )(input);

    let loc_entry = |input| debug!("loc_entry", 4,
        alt((
            map(name, |x| vec![x]),
            options!(name)
        ))
    )(input);

    let act_entry = |input| debug!("act_entry", 4,
        options!(act_single)
    )(input);

    let entry = |input| debug!("entry", 3,
        tuple((
            loc_entry,
            act_entry,
            loc_entry
        ))
    )(input);

    let delta = |input| debug!("delta", 2,
        preceded(
            keyword("delta"),
            parse_separated_terminated(
                entry,
                token(','),
                alt((
                    eof.value(()),
                    not(token(','))
                )),
                || vec![],
                |mut v, (l, a, l2)| {
                    for (l, a, l2) in iproduct!(&l, &a, &l2) {
                        v.push((l.clone(), a.clone(), l2.clone()));
                    }
                    v
                }
            )
        )
    )(input);

    delta(input)
}

#[derive(Debug)]
pub enum Statement {
    Agt(Vec<Symbol>),
    Act(Vec<Symbol>),
    Loc(Vec<Symbol>),
    Reach(Vec<Symbol>),
    L0(Symbol),
    Obs(Symbol, Vec<Vec<Symbol>>),
    Delta(Vec<(Symbol, Vec<Symbol>, Symbol)>)
}

fn statement(input: &str) -> R<&str, Statement> {
    debug!("statement", 1, alt((
        map(agt, |x| Statement::Agt(x)),
        map(act, |x| Statement::Act(x)),
        map(loc, |x| Statement::Loc(x)),
        map(reach, |x| Statement::Reach(x)),
        map(l0, |x| Statement::L0(x)),
        map(obs, |(x, y)| Statement::Obs(x, y)),
        map(delta, |x| Statement::Delta(x))
    )))(input)
}

pub fn parse(input: &str) -> Result<Vec<Statement>, E<final_parser::Location>> {
    final_parser(
        debug!("file", 0, preceded(
            sp,
            many1(statement)
        ))
    )(input)
}
