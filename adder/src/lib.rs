#[cfg(test)]
mod tests {
    #[test]
    fn it_works1() {
        println!("1");
    }
    #[test]
    fn it_works2() {
        println!("2");
        assert!(false);
    }
    #[test]
    fn it_works3() {
        println!("3");
    }
   
}
