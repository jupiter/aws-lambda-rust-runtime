#![feature(async_await, start)]

use lambda::{lambda, LambdaCtx};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[runtime::main]
async fn main(s: String, ctx: LambdaCtx) -> Result<String, Error> {
    let _ = ctx;
    Ok(s)
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}