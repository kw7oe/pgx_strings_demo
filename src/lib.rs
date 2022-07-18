use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_pgx_strings() -> &'static str {
    "Hello, pgx_strings"
}

#[pg_extern]
fn to_title(string: &str) -> String {
    string
        .split(' ')
        .map(|word| {
            word.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_lowercase().to_string()
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[pg_extern]
fn emojify(string: &str) -> String {
    string
        .split(' ')
        .map(|word| {
            let chars = word.chars().collect::<Vec<char>>();
            match &chars[..] {
                [':', shortcode @ .., ':'] => {
                    emojis::get_by_shortcode(&shortcode.iter().collect::<String>())
                        .unwrap()
                        .to_string()
                }
                _ => word.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pgx_strings() {
        assert_eq!("Hello, pgx_strings", crate::hello_pgx_strings());
    }

    #[pg_test]
    fn test_to_title() {
        assert_eq!("My Cool Extension", crate::to_title("my cool extension"));
    }

    #[pg_test]
    fn test_emojify() {
        assert_eq!("pgx is so cool 💯", crate::emojify("pgx is so cool :100:"));
        assert_eq!(
            "multiple emojis: 💯 👍",
            crate::emojify("multiple emojis: :100: :+1:")
        );
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
