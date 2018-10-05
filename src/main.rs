extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use pest::{
    Parser,
    iterators::{Pair, Pairs},
    prec_climber::{Operator, PrecClimber, Assoc},
};

#[derive(Parser)]
#[grammar = "ident.pest"]
struct CalcParser;

fn eval(pair: Pair<Rule>, climber: &PrecClimber<Rule>, vars: &HashMap<&str, f64>) -> f64 {
    let operator = |left: f64, op: Pair<Rule>, right: f64| match op.as_rule() {
        Rule::add => left + right,
        Rule::sub => left - right,
        Rule::mul => left * right,
        Rule::div => left / right,
        _ => unreachable!()
    };

    let term = |pair| eval(pair, climber, &vars);

    match pair.as_rule() {
        Rule::expr => climber.climb(pair.into_inner(), term, operator),
        Rule::number => pair.as_str().parse().unwrap(),
        Rule::calc => eval(pair.into_inner().next().unwrap(), climber, &vars),
        Rule::term => eval(pair.into_inner().next().unwrap(), climber, &vars),
        Rule::var => vars.get(&pair.as_str()).unwrap().clone(),
        _ => unreachable!()
    }
}

struct Calc<'a> {
    climber: PrecClimber<Rule>, 
    pairs: Pairs<'a, Rule>,
}

impl<'a> Calc<'a> {
    fn new(expression: &'a str) -> Calc<'a> {
        let climber = PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::sub, Assoc::Left),
            Operator::new(Rule::mul, Assoc::Left) | Operator::new(Rule::div, Assoc::Left),
        ]);
        let pairs = CalcParser::parse(Rule::calc, expression).unwrap();
        Calc { climber, pairs }
    }

    fn eval(&self, vars: &HashMap<&'a str, f64>) -> f64 {
        let pairs = self.pairs.clone();
        eval(pairs.into_iter().next().unwrap(), &self.climber, vars)
    }
}

fn main() {
    let calc = Calc::new("x * (9 + 1/10) * (5 + 1 + foo)");

    let mut vars = HashMap::new();
    vars.insert("x", 10.0);
    vars.insert("foo", 2.0);

    println!("result = {:?}", calc.eval(&vars));
}