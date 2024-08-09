
# Rust and C++ Integration Example

This project gives an example on how a Rust library can be integrated into an existing C++ code base, linked as a shared object. 

I decided to make this available here, after integrating a new Rust library into an existing, legacy C++ codebase. 

It's surprisingly straight forward, and I hope it helps you to grasp the principal mechanism of this project.

## Project Structure

- **cpp_user/**: An example C++ project that will be the user of the Rust library. It demonstrates how to wrap the Rust struct and extend them in C++. 
- **rusty_lib/**: A hello world Rust library that will be treated as if it's a base class by the user. 
- **Dockerfile.rhel9**: I chose RHEL9 to be the source image, where I am building the library
- **Dockerfile.ubuntu**: Similarly, I chose Ubuntu to be the target. Different distros to showcase rust builds for architecture, which allows for great flexibility. 

## Step-by-Step Guide

Ensure you're on project root, and follow along

### Step 1: Build the Rust Library in a RHEL-based Docker Container

1. **Build the Docker Image**:
   ```
   docker build -t rust_lib_builder -f Dockerfile.rhel9 .
   ```

2. **Run the Docker Container** to build the Rust library:
   ```
   docker run --name rust_lib_builder_container rust_lib_builder
   ```

3. **Copy the shared object** to your local `lib/` directory:
   ```
   docker cp rust_lib_builder_container:/usr/src/rust_lib/target/x86_64-unknown-linux-gnu/release/librusty_lib.so ./cpp_user/lib/
   ```

### Step 2: Build and Run the C++ Project in an Ubuntu-based Docker Container

1. **Build the Docker Image**:
   ```
   docker build -t cpp_project_tester -f Dockerfile.ubuntu .
   ```

2. **Run the Docker Container** to compile and execute the C++ code:
   ```
   docker run --rm cpp_project_tester
   ```

### Expected Output

If everything is set up correctly, you should see the following output:

```
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Stack Allocated C++ User
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Heap Allocated C++ User
CustomGreeter says: Hello from C++, extending a Rust Struct!
Hello from RustGreeter! My name is Smart Pointer C++ User
```

## Explanation

- **Rust Code**: Implements a `RustGreeter` struct and a `Greeter` trait, with functions exposed to C++ via FFI (Foreign Function Interface).
- **C++ Code**: Uses the Rust library by calling the FFI functions, and demonstrates stack, heap, and smart pointer allocations.
- **Docker**: Used to build the Rust library in a RHEL environment and then compile and run the C++ code in an Ubuntu environment, showcasing cross-platform compatibility.

## Troubleshooting

- **Shared Library Not Found**: If you encounter an error about `librusty_lib.so` not being found, ensure the `LD_LIBRARY_PATH` is correctly set in the Dockerfile.
- **Linking Errors**: Ensure that the Rust functions are declared with `extern "C"` and that the `Makefile` correctly includes and links all necessary files.

## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.