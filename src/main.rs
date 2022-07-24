mod add;

pub fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::main;
    #[test]
    fn it_works() {
        main();
    }
}