pub mod interpreter;
mod jump_table;
mod memory;
mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
