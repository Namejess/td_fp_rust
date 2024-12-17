// Typedef
type BinaryMathOperation = i8;


fn main() {
    // Add
    add(2, 2);

}

// Add
fn add(x:BinaryMathOperation, y: BinaryMathOperation) -> BinaryMathOperation {
    x + y
}

// Multiply 
fn multiply(x: BinaryMathOperation, y: BinaryMathOperation) -> BinaryMathOperation {
    x * y
}

// Divide 
fn divide(x: BinaryMathOperation, y: BinaryMathOperation) -> BinaryMathOperation {
    x / y
}

// Substract
fn substract(x: BinaryMathOperation, y: BinaryMathOperation) -> BinaryMathOperation {
    x - y
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_multiply(){
        let result = multiply(2, 3);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_divide(){
        let result = divide(10, 2);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_substract(){
        let result = substract(23,3);
        assert_eq!(result, 20)
    }



}