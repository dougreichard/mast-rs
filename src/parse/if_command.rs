use super::utils::space;
use super::valid_id;
use super::MastCmd;
use pom::parser::{seq, sym, none_of};
use pom::parser::Parser;

/* 
pub(crate) fn atom<'a>() -> Parser<'a, u8, String> {
    valid_id()
    | seq(b"True")
    | seq(b"False")
    | seq(b"None")
    | strings()
    | number()

}


pub(crate) fn factor<'a>() -> Parser<'a, u8, MastCmd> {
    sym(b'+') + factor()
    | sym(b'-') + factor()
    | atom()
}

pub(crate) fn term<'a>() -> Parser<'a, u8, MastCmd> {
    let t = term() + sym(b'*') + factor()
    | term() + sym(b'/') + factor()
    | term() + seq(b"//") + factor()
    | term() + seq(b"%") + factor()
    | term() + seq(b"@") + factor()
    | factor();
    
}


pub(crate) fn sum<'a>() -> Parser<'a, u8, MastCmd> {
    sum() + sym(b'+') + term()
    | sum() + sym(b'-') + term()
    | term()
}


// sum:
//     | sum '+' term 
//     | sum '-' term 
//     | term

// term:
//     | term '*' factor 
//     | term '/' factor 
//     | term '//' factor 
//     | term '%' factor 
//     | term '@' factor 
//     | factor

// factor:
//     | '+' factor 
//     | '-' factor 
//     | '~' factor 
//     | power

// power:
//     | await_primary '**' factor 
//     | await_primary

// # Primary elements
// # ----------------

// # Primary elements are things like "obj.something.something", "obj[something]", "obj(something)", "obj" ...

// await_primary:
//     | AWAIT primary 
//     | primary

// primary:
//     | primary '.' NAME 
//     | primary genexp 
//     | primary '(' [arguments] ')' 
//     | primary '[' slices ']' 
//     | atom

// slices:
//     | slice !',' 
//     | ','.(slice | starred_expression)+ [','] 

// slice:
//     | [expression] ':' [expression] [':' [expression] ] 
//     | named_expression 

// atom:
//     | NAME
//     | 'True' 
//     | 'False' 
//     | 'None' 
//     | strings
//     | NUMBER
//     | (tuple | group | genexp)
//     | (list | listcomp)
//     | (dict | set | dictcomp | setcomp)
//     | '...' 
*/


pub(crate) fn if_command<'a>() -> Parser<'a, u8, MastCmd> {
    let condition  = none_of(b":").repeat(0..) - sym(b':');
    seq(b"if") * space() * condition
        .convert(String::from_utf8)
        .map(|s| MastCmd::If(s))
}

pub(crate) fn elif_command<'a>() -> Parser<'a, u8, MastCmd> {
    let condition  = none_of(b":").repeat(0..) - sym(b':');
    seq(b"elif") * space() * condition
        .convert(String::from_utf8)
        .map(|s| MastCmd::ElIf(s))
}

pub(crate) fn end_if_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"end_if").map(|s| MastCmd::EndIf())
}
pub(crate) fn else_command<'a>() -> Parser<'a, u8, MastCmd> {
    seq(b"else:").map(|s| MastCmd::Else())
}


#[cfg(test)]
mod tests {
    #[test]
    fn if_test() {
        // One sapce
        let jump = super::if_command().parse(b"if x>2:");
        let _label: String = String::from(" x>2");
        assert!(matches!(jump, Ok(super::MastCmd::If(_label))));
        let jump = super::if_command().parse(b"if   x > 2 :");
        let _label: String = String::from("   x > 2 ");
        assert!(matches!(jump, Ok(super::MastCmd::If(_label))));
    }
    #[test]
    fn elif_test() {
        // One sapce
        let jump = super::elif_command().parse(b"elif x>2:");
        let _label: String = String::from(" x>2");
        assert!(matches!(jump, Ok(super::MastCmd::ElIf(_label))));
        let jump = super::elif_command().parse(b"elif   x > 2 :");
        let _label: String = String::from("   x > 2 ");
        assert!(matches!(jump, Ok(super::MastCmd::ElIf(_label))));
    }
    #[test]
    fn else_test() {
        // One sapce
        let jump = super::else_command().parse(b"else:");
        assert!(matches!(jump, Ok(super::MastCmd::Else())));
        
    }
    #[test]
    fn end_if_test() {
        // One sapce
        let jump = super::end_if_command().parse(b"end_if");
        assert!(matches!(jump, Ok(super::MastCmd::EndIf())));
        
    }
}
