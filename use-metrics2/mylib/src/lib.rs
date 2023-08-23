use metrics::register_counter;
pub fn add(left: usize, right: usize) -> usize {
    // And registering them:
    let counter1 = register_counter!("test_counter======");
    counter1.increment(1);

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
