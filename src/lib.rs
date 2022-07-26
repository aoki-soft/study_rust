pub mod logger;
pub mod sort;
use tracing::*;
use anyhow::Result;

pub fn main() -> Result<()> {
    logger::init();

    info!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::main;
    #[test]
    fn it_works() -> Result<()> {
        main()
    }
}