use crate::stdout::STDOUT_STREAM;

/// Prints output.
#[macro_export]
macro_rules! stdout {
        ($($args:tt)*) => {
            $crate::stdout::print::print(&format!($($args)*));
        };
    }

/// Prints output with a newline.
#[macro_export]
macro_rules! stdoutln {
        () => ($crate::stdout::print::print("\n"));
        ($e:expr) => ($crate::stdout::print::println($e));
        ($($args:expr),*) => (
            $crate::stdout::print::println(&format!($($args),*));
        )
    }

/// Print output.
pub fn print(text: &str) {
    STDOUT_STREAM
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .as_mut()
        .print(text)
        .unwrap()
}

/// Print output with a newline.
pub fn println(text: &str) {
    print(text);
    print("\n");
}

#[cfg(test)]
mod tests {
    use crate::stdout::{set_buffer, get_buffer_value};
    use crate::stdout::colorize::good;

    #[test]
    fn test_stdout_string_buff() {
        set_buffer(String::new()).unwrap();
        stdout!("test value");
        assert_eq!(Some("test value".to_string()), get_buffer_value());
    }

    #[test]
    fn test_stdoutln_string_buff() {
        set_buffer(String::new()).unwrap();
        stdoutln!("test value");
        assert_eq!(Some("test value\n".to_string()), get_buffer_value());
    }

    #[test]
    fn test_stdoutln_string_buff_with_color() {
        set_buffer(String::new()).unwrap();
        stdout!("{}", good("test value"));
        assert_eq!(Some("test value".to_string()), get_buffer_value());
    }
}
