## A example of PostgreSQL extensions using `pgx`

This project includes a simple PostgreSQL extension written in
Rust, using the [`pgx` crate][0]. This extension contains the following
string manipulations functions:

- `to_title`: Convert a given string to title case.
- `emojify`: Convert any emoji shortcode in a string to emoji.

This is extension is for demonstration and learning purposes and by no mean for
using it in production.

[0]: https://github.com/tcdi/pgx
