fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_add() {
        // ANCHOR: example
        println!("{} > {} = {}", 1, 'a', 1 > 'a');
        // ANCHOR_END: example
    }
}
