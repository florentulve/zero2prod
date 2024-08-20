#![warn(clippy::all, clippy::pedantic)]
use std::error::Error;

mod webapp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    webapp::boostrap().await;

    Ok(())
}
