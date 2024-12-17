
fn main() {
    // Add
    add(2, 2);

}

fn add(x:i8, y: i8) -> i8 {
    x + y
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn another() {
        panic!("FAIL BON SANG");
    }
    


}