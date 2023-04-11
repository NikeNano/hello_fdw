hello_fdw
=========

### About

This project is a rust port of [hello_fdw](https://github.com/wikrsh/hello_fdw) which was written in C.
This port builds on `pgx` which is a powerful lib for building postgres extensions in rust. Even though `pgx`
offers a safe-ish interface, this foreign data wrapper uses a _lot_ of unsafe code. This is because we call
quite a few postgres C functions.


### Running

You can run the project by doing:

```
cargo pgx run pg13
```

or you can run the test suite by running:

```
cargo pgx test pg13
```

```SQL
hello_fdw=# CREATE EXTENSION hello_fdw;
CREATE EXTENSION
hello_fdw=# CREATE FOREIGN DATA WRAPPER hello_fdw HANDLER hello_fdw_handler VALIDATOR hello_fdw_validator;
CREATE FOREIGN DATA WRAPPER
hello_fdw=# CREATE SERVER hello_server FOREIGN DATA WRAPPER hello_fdw;
CREATE SERVER
hello_fdw=# CREATE FOREIGN TABLE hello_fdw_table (id text, data text) SERVER hello_server;
CREATE FOREIGN TABLE
hello_fdw=# SELECT * FROM hello_fdw_table;
     id      |    data
-------------+-------------
 Hello,World | Hello,World
(1 row)

hello_fdw=#
```

### License

MIT
