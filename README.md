
# Rust and C++ Integration Examples
This project offers a collection of working examples that demonstrate how to integrate a Rust library into an existing C++ codebase as a shared object. These examples range from basic integration techniques to more advanced patterns, aiming to provide a comprehensive catalogue of Rust-C++ interoperability examples.

The idea for this project came from real-world challenges I encountered while integrating a new Rust library into a legacy C++ codebase. My hope is that these examples will help you understand the principles and mechanisms behind successful Rust-C++ integration.

I invite you to contribute your own examples to help build a robust catalog of solutions for scenarios that are likely to arise during integrating real-world projects.

## Project Structure

* **examples/**: Contains multiple example projects that demonstrate different aspects of Rust and C++ integration. Each sub-directory includes a README with documentation.
   * **basic_integration/**: A simple example demonstrating basic Rust-C++ integration.
   * **concurrency_example/**: A batch processing example showcasing concurrency-safe interoperability between Rust and C++.
   * **producer_consumer_example/**: An advanced example implementing the producer-consumer pattern using Rust and C++.
* **scripts/**: Contains helper script(s) for building and running the examples.
* **Dockerfile.rhel9**: Dockerfile for building the Rust libraries in a RHEL9 environment.
* **Dockerfile.ubuntu**: Dockerfile for compiling and running the C++ code in an Ubuntu environment.

## Getting Started

The repo comes with a launcher script that builds and runs the docker containers for you.

1. **Start the launcher**:
```bash
scripts/launcher.sh
```

2. **Select an Example**: The launcher will prompt you to select an example from the list below:
* Basic Integration Example
* Concurrency Example
* Producer-Consumer Example

3. **Enjoy Your Sandbox!**: 
* The launcher will build and run your selected example. 
* Monitor to output to understand how it works; modify the code and re-run to quickly protoype your custom implementations!

## Contributing

Contributions to this project are encouraged and greatly appreciated. If you have an example that showcases Rust-C++ interoperability, or improvements to existing examples, please share them.

**How to Contribute**: Please fork this repository, add your changes in a new branch, test that they work, and submit a pull request.

## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.