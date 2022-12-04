
use std::str::{self, FromStr};

#[test]
fn test_jump() {
    let input = b"
    -> JumpTest";
    let ast = mast_rs::parse::mast_commands().parse(input.as_slice());
    assert!(ast.is_ok())
    
    
}