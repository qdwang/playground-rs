#![allow(dead_code)]
use std::backtrace::Backtrace;
use std::io;

erreport::gen_report_code!();
use report::*;

fn fib(x: u64) -> Result<u64, Report> {
    if x < 2 {
        let stack = Backtrace::capture();
        println!("{stack:?}");

        let err = io::Error::new(io::ErrorKind::AddrInUse, "fail");
        Err(err).to_report()?;
        Ok(x)
    } else {
        let a = fib(x - 2)?;
        let b = fib(x - 1)?;
        Ok(a + b)
    }
}

// need debug=1
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = fib(500)?;
    println!("{result}");
    Ok(())
}
