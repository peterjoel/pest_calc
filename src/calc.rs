use std::collections::HashMap;
use pest::{
    Parser,
    error::Error as PestError,
    iterators::{Pair, Pairs},
    prec_climber::{Operator, PrecClimber, Assoc},
};

#[derive(Parser)]
#[grammar = "ident.pest"]
struct CalcParser;

pub type Error = PestError<Rule>;

pub struct Calc<'a> {
    climber: PrecClimber<Rule>, 
    pairs: Pairs<'a, Rule>,
}

impl<'a> Calc<'a> {
    pub fn new(expression: &'a str) -> Result<Calc<'a>, Error> {
        let climber = PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::sub, Assoc::Left),
            Operator::new(Rule::mul, Assoc::Left) | Operator::new(Rule::div, Assoc::Left),
        ]);
        CalcParser::parse(Rule::calc, expression)
            .map(|pairs| Calc { climber, pairs })
    }

    pub fn eval(&self) -> f64 {
        let pair = self.pairs.clone().into_iter().next().unwrap();
        self.eval_rule(pair, &HashMap::new())
    }

    pub fn eval_in_context(&self, context: &HashMap<&'a str, f64>) -> f64 {
        let pair = self.pairs.clone().into_iter().next().unwrap();
        self.eval_rule(pair, context)
    }

    fn eval_rule(&self, pair: Pair<Rule>, vars: &HashMap<&str, f64>) -> f64 {
        let operator = |left, op: Pair<Rule>, right| match op.as_rule() {
            Rule::add => left + right,
            Rule::sub => left - right,
            Rule::mul => left * right,
            Rule::div => left / right,
            _ => unreachable!("Unexpected rule: {:?}", op),
        };

        let term = |pair| self.eval_rule(pair, &vars);

        match pair.as_rule() {
            Rule::expr => self.climber.climb(pair.into_inner(), term, operator),
            Rule::number => pair.as_str().parse().unwrap(),
            Rule::calc => self.eval_rule(pair.into_inner().next().unwrap(), &vars),
            Rule::term => self.eval_rule(pair.into_inner().next().unwrap(), &vars),
            Rule::var => *vars.get(&pair.as_str()).unwrap(),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }
}


#[cfg(test)]
mod test {

    use float_cmp::ApproxEqUlps;
    use super::*;

    #[test]
    fn adding() -> Result<(), Error> {
        let calc = Calc::new("1 + 5.5")?;
        assert!(6.5.approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }
    #[test]
    fn subtracting() -> Result<(), Error> {
        let calc = Calc::new("1 - 5.5")?;
        assert!((-4.5).approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }
    #[test]
    fn multiplying() -> Result<(), Error> {
        let calc = Calc::new("3 * 7.5")?;
        assert!(22.5.approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }
    #[test]
    fn dividing() -> Result<(), Error> {
        let calc = Calc::new("9 / 3")?;
        assert!(3.0.approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }
    #[test]
    fn precedence() -> Result<(), Error> {
        let calc = Calc::new("2 + 3 * 3")?;
        assert!(11.0.approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }
    #[test]
    fn parentheses() -> Result<(), Error> {
        let calc = Calc::new("(2 + 3) * 3")?;
        assert!(15.0.approx_eq_ulps(&calc.eval(), 2));
        Ok(())
    }

    #[test]
    fn variables() -> Result<(), Error> {
        let calc = Calc::new("(a_variable + 2) * (3 + b2/0.01)")?;
        let mut vars = HashMap::new();
        vars.insert("a_variable", 3.0);
        vars.insert("b2", 0.02);
        assert!(25.0.approx_eq_ulps(&calc.eval_in_context(&vars), 2));
        Ok(())
    }
}