# Prisma Query Engine C API

Bind the Prisma ORM query engine to any programming language you like ❤️

## Features

- Rust bindings for the C API
- Static link library
- Dynamic link library

## Usage

You can download the dynamic link library and static link library automatically built by CI on the Releases page. Of course, the header files you need to use the C language can also be found here.

Libraries currently built by CI:

- `api.h` - The C APIs for the query engine
- `linux-x86_64-gnu.so` - Linux x86_64 dynamic link library, Using `gun` to build this library
- `macos-silicon.dylib` - macOS Silicon dynamic link library.
- `macos-x86_64.dylib` - macOS x86_64 dynamic link library.

> **Note**: More static link library and dynamic link library will be available soon.
