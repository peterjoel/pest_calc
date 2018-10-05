extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate float_cmp;

mod calc;

use std::collections::HashMap;
use calc::{Calc, Error};

fn main() -> Result<(), Error> {
    let calc = Calc::new("x * (9 + 1/10) * (5 + 1 + foo) / d")?;

    let mut vars = HashMap::new();
    vars.insert("x", 10.0);
    vars.insert("foo", 2.0);
    vars.insert("d", 1.0);

    println!("result = {:?}", calc.eval_in_context(&vars));

    Ok(())
}