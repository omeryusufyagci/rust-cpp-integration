# Basic Integration Example

This example demonstrates the fundamental steps required to integrate a Rust library into an existing C++ codebase as a shared object. 

It serves as an introductory guide to Rust-C++ interoperability, showcasing how Rust can be used within a legacy C++ project.

## Key Concepts
* **Basic FFI Integration**: Demonstrates how to expose Rust code to C++ using the Foreign Function Interface (FFI).
* **Cross-Language Integration**: Illustrates how to wrap Rust structs and extend them in C++, allowing native extension of Rust APIs within C++.
* **Architecture-Targeted Compilation**: Demonstrates cross-compilation by building the Rust library (`.so` file) in a RHEL9 environment and using it within a C++ project on an Ubuntu environment, showcasing the use of architecture-targeted flags like `x86_64-unknown-linux-gnu`.

## How to run

From project root, run:

```bash
scripts/launcher.sh  # Select `Basic Integration Example`
```

### Expected Output

You should see the following output:

```
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Stack Allocated C++ User
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Heap Allocated C++ User
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Smart Pointer C++ User
```

Explanation
* **Rust Code**: Implements a RustGreeter struct and a Greeter trait, exposing functions to C++ via FFI. This allows C++ to interact with Rust as if it were a native library.
* **C++ Code**: Demonstrates how to utilize the Rust library by integrating it into a C++ project.
* **Cross-Platform Docker Setup**: Uses containerized environments to build the Rust library on RHEL9 and run the C++ project on Ubuntu. This setup highlights Rust's cross-platform capabilities and ensures consistent results across different environments.