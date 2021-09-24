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

### License

MIT
