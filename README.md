> [!WARNING]
> 
> As `pgx` is being renamed to `pgrx`, this repository is outdated. Please
> head over to https://github.com/kw7oe/pgrx_string_demo for the updated version.

## A example of PostgreSQL extensions using `pgx`

This project includes a simple PostgreSQL extension written in
Rust, using the [`pgx` crate][0]. This extension contains the following
string manipulations functions:

- `to_title`: Convert a given string to title case.
- `emojify`: Convert any emoji shortcode in a string to emoji.

This is extension is for demonstration and learning purposes and by no mean for
using it in production.

If you would like to learn more about it you can refer to this post:
[Writing PostgreSQL extension in Rust With `pgx`](https://kaiwern.com/posts/2022/07/20/writing-postgresql-extension-in-rust-with-pgx/)

## Getting Started

1. Install and setup `cargo-pgx`. For more, refer to the [Getting Started][1] of
   `pgx`

```
cargo install cargo-pgx
cargo pgx init
```

2. Clone this repository and change directory into it.
3. Run it using `cargo pgx run pg14`. This will spin up a PostgreSQL.
4. In your `psql` session, load the extension first:

```sql
pgx_strings=# create extension pgx_strings;
CREATE EXTENSION

pgx_strings=# \df
                             List of functions
 Schema |       Name        | Result data type | Argument data types | Type
--------+-------------------+------------------+---------------------+------
 public | emojify           | text             | string text         | func
 public | hello_pgx_strings | text             |                     | func
 public | to_title          | text             | string text         | func
(3 rows)
```

5. Now with everything loaded you can run the following SQL:

```sql
pgx_strings=# select hello_pgx_strings();
 hello_pgx_strings
--------------------
 Hello, pgx_strings
(1 row)

pgx_strings=# select to_title('this is so cool');
    to_title
-----------------
 This Is So Cool
(1 row)

pgx_strings=# select emojify('I love pgx :100:');
   emojify
--------------
 I love pgx 💯
(1 row)
```

[0]: https://github.com/tcdi/pgx
[1]: https://github.com/tcdi/pgx/tree/master#getting-started
