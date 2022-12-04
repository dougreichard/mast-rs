
use pom::parser::one_of;

use pom::parser::Parser;

pub(crate) fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}
