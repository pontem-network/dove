use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::FileOffsetMap;

pub fn normalize(s: &mut MutString) -> FileOffsetMap {
    let mut source_map = FileOffsetMap::default();

    if s.as_ref().is_ascii() {
        // Move allows only ascii symbols. We can iter by byte index.
        for i in 0..s.as_ref().len() {
            let c = &s.as_ref()[i..i + 1];
            if c == "\r" {
                source_map.insert_layer(i - 1, 1);
                s.make_patch(i, i + 1, NewValue::Borrowed(""));
            }
        }
    }

    source_map
}

#[cfg(test)]
mod test {
    use crate::compiler::dialects::line_endings::normalize;
    use crate::compiler::mut_string::MutString;
    use crate::compiler::source_map::FileOffsetMap;

    #[test]
    pub fn test_line_ending() {
        let source = "
            script {\r\n
                use 0x01::Event;\r\n
                use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Math;\r\n
                use 0x02::Invald;\r\n\r\n

                fun main(account: &signer, a: u64, b: u64) {\r\n
                    let sum = Math::add(a, b);\r\n
                    Event::emit(account, sum);\r\n
                }\r\n
            }\r\n\r\n
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
        expected_source_map.insert_layer(20, 1);
        expected_source_map.insert_layer(55, 1);
        expected_source_map.insert_layer(132, 1);
        expected_source_map.insert_layer(168, 1);
        expected_source_map.insert_layer(170, 1);
        expected_source_map.insert_layer(234, 1);
        expected_source_map.insert_layer(283, 1);
        expected_source_map.insert_layer(332, 1);
        expected_source_map.insert_layer(352, 1);
        expected_source_map.insert_layer(368, 1);
        expected_source_map.insert_layer(370, 1);

        assert_eq!(source_map, expected_source_map);
    }
}
