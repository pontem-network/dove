use std::borrow::Cow;
use std::str::Chars;

use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::FileOffsetMap;

pub fn normalize(s: &mut MutString) -> FileOffsetMap {
    let mut source_map = FileOffsetMap::default();

    let mut _n_prev = false;

    // Move allows only ascii symbols. We can iter by byte index.
    for i in 0..s.as_ref().len() {
        let c = &s.as_ref()[i..i + 1];
        if c == "\n" {
            _n_prev = true;
        } else if c == "\r" {
            source_map.insert_layer(i, 1);
            s.make_patch(i - 1, i + 1, NewValue::Borrowed("\n"));
            _n_prev = false;
        } else {
            _n_prev = false;
        }
    }

    source_map
}

#[cfg(test)]
mod test {
    use crate::compiler::dialects::line_endings::normalize;
    use crate::compiler::mut_string::MutString;
    use crate::compiler::source_map::FileOffsetMap;
    use std::str::Chars;

    #[test]
    pub fn test_line_ending() {
        let source = "
            script {\n\r
                use 0x01::Event;\n\r
                use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Math;\n\r
                use 0x02::Invald;\n\r\n\r

                fun main(account: &signer, a: u64, b: u64) {\n\r
                    let sum = Math::add(a, b);\n\r
                    Event::emit(account, sum);\n\r
                }\n\r
            }\n\r\n\r
        ";
        let mut mut_str = MutString::new(source);
        let source_map = normalize(&mut mut_str);
        let expected = "
            script {\n
                use 0x01::Event;\n
                use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Math;\n
                use 0x02::Invald;\n\n

                fun main(account: &signer, a: u64, b: u64) {\n
                    let sum = Math::add(a, b);\n
                    Event::emit(account, sum);\n
                }\n
            }\n\n
        ";

        assert_eq!(expected, mut_str.freeze());
        let mut expected_source_map = FileOffsetMap::default();
        expected_source_map.insert_layer(22, 1);
        expected_source_map.insert_layer(57, 1);
        expected_source_map.insert_layer(134, 1);
        expected_source_map.insert_layer(170, 1);
        expected_source_map.insert_layer(172, 1);
        expected_source_map.insert_layer(236, 1);
        expected_source_map.insert_layer(285, 1);
        expected_source_map.insert_layer(334, 1);
        expected_source_map.insert_layer(354, 1);
        expected_source_map.insert_layer(370, 1);
        expected_source_map.insert_layer(372, 1);

        assert_eq!(source_map, expected_source_map);
    }
}
