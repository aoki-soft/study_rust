mod add;
mod logger;

use tracing::*;

use crate::logger::init_logger;

pub fn main() {
    init_logger();
    
    info!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::main;
    #[test]
    fn it_works() {
        main();
    }
}