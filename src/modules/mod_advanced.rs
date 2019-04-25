use std::fmt::Result;
use std::io::Result as IoResult; // use a local name to avoid conflict 

fn function1() -> Result {
}

fn function2() -> IoResult<()> {
}