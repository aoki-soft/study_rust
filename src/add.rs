
pub fn add(x: usize, y: usize) -> usize {
    x + y
}

#[cfg(test)]
mod tests {
    use super::add;
    #[test]
    fn it_works() {
        add(1, 3);
    }
}