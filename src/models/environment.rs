use once_cell::sync::OnceCell;
use std::{fmt::Display, str::FromStr};

use crate::utility::capitalize;

use super::error::Error;

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

impl TryFrom<String> for Environment {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Environment::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Environment {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Environment::from_str(value)
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Environment::*;
        let illegal_argument = {
            static MODES: [&str; 3] = ["local", "staging", "prod"];
            Error::IllegalArgument(format!(
                "{} must be one of: ({}|{})",
                s,
                MODES.join("|"),
                MODES.map(capitalize).join("|")
            ))
        };
        match s {
            "local" | "Local" => Ok(Local),
            "staging" | "Staging" => Ok(Staging),
            "production" | "Production" => Ok(Production),
            _ => Err(illegal_argument),
        }
    }
}

impl Environment {
    pub fn current_env() -> Self {
        static LOCK: OnceCell<Environment> = OnceCell::new();
        *LOCK.get_or_init(|| {
            let default_env = Environment::Local;

            std::env::var("MOVIES_PROFILE")
                .ok()
                .and_then(|profile| Environment::try_from(profile).ok())
                .unwrap_or_else(|| {
                    tracing::warn!(
                        default_env = %default_env,
                        "Unable to determine env using default env",
                    );
                    default_env
                })
        })
    }

    pub const fn to_str(&self) -> &str {
        use Environment::*;
        match self {
            Local => "local",
            Staging => "staging",
            Production => "production",
        }
    }

    pub fn config_file(&self) -> String {
        format!("config/{}.yaml", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok_eq};

    #[test]
    fn should_parse_env_str() {
        let envs = [
            ("local", Environment::Local),
            ("staging", Environment::Staging),
            ("production", Environment::Production),
        ];

        for (env_str, env) in envs {
            assert_ok_eq!(
                Environment::from_str(env_str),
                env,
                "Environment string: {} => {}",
                env_str,
                env
            );
        }
    }

    #[test]
    fn should_not_parse_env_str() {
        let input =
            "This is some random input which is used to test the string parse functionality"
                .split(" ");
        for word in input {
            assert_err!(
                Environment::from_str(word),
                "The input: <{}> must not be parsed",
                word
            );
        }
    }
}
