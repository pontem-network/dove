use crate::stdout::STDOUT_STREAM;

/// Prints output.
#[macro_export]
macro_rules! stdout {
        ($e:expr) => (
            $crate::stdout::print::print($e)
        );
        ($($args:tt)*) => {
            $crate::stdout!(&format!($($args)*));
        };
    }

/// Prints output with a newline.
#[macro_export]
macro_rules! stdoutln {
        () => (
            $crate::stdout!("\n")
        );
        ($($args:expr),*) => (
            $crate::stdout!(&format!($($args),*));
            $crate::stdout!("\n");
        )
    }

/// Print output.
pub fn print(text: &str) {
    STDOUT_STREAM
        .get()
        .expect("STDOUT_STREAM is not initialized")
        .lock()
        .expect("Failed to get STDOUT_STREAM")
        .as_mut()
        .print(text)
        .expect("Failed write to STDOUT_STREAM")
}

#[cfg(test)]
mod tests {
    use crate::stdout::{get_buffer_value_and_erase, set_print_to_string};
    use crate::stdout::colorize::good;

    #[test]
    fn test_print_to_string() {
        set_print_to_string();
        stdout!("test value");
        assert_eq!(Some("test value".to_string()), get_buffer_value_and_erase());

        set_print_to_string();
        stdoutln!("test value");
        assert_eq!(
            Some("test value\n".to_string()),
            get_buffer_value_and_erase()
        );

        set_print_to_string();
        stdout!("{}", good("test value"));
        assert_eq!(Some("test value".to_string()), get_buffer_value_and_erase());
    }
}
