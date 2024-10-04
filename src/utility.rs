#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {
        {
            tracing::error!($($arg)*);
            std::process::exit(1);
        }
    };
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_capitalize_ascii() {
        let modes = [
            ("base", "Base"),
            ("local", "Local"),
            ("staging", "Staging"),
            ("prod", "Prod"),
        ];

        for (input, expected) in modes {
            assert_eq!(
                capitalize(input),
                expected,
                "capitalize({}) = {}",
                input,
                expected
            )
        }
    }
}
