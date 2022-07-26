use study_rust::logger;
use tracing::*;

fn main() {
    logger::init();

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
