# Rust and C++ Integration Examples
This project provides a curated collection of examples that demonstrate how to integrate a Rust library into an existing C++ codebase as a shared object. From basic integration techniques to more advanced patterns, these examples aim to offer a comprehensive catalog of Rust-C++ interoperability scenarios.

The project was inspired by real-world challenges I encountered while integrating a new Rust library into a legacy C++ codebase. I hope these examples will help you understand the key mechanisms behind successful Rust-C++ integration.

Contributions are welcome! I encourage you to contribute your own examples and help build a robust catalog of solutions for real-world challenges that arise in Rust-C++ integration.

## Project Structure

* **examples/**: Contains various example projects that demonstrate different aspects of Rust and C++ integration. Each sub-directory includes a README with documentation.
   * **basic_integration/**: A simple example demonstrating basic Rust-C++ integration.
   * **concurrency_example/**: A batch processing example showcasing concurrency-safe interoperability between Rust and C++.
   * **producer_consumer_example/**: An advanced example implementing the producer-consumer pattern using Rust and C++.
* **scripts/**: Contains a helper script (`launcher.sh`) for building and running the examples.

## Docker Setup

The project has been consolidated to use a single Docker setup, streamlining the build process for both Rust and C++ code. This approach simplifies the environment and ensures consistency across different examples.

### Key Features:
* **Unified Dockerfile**: Instead of separate Dockerfiles per example and platform, a single Dockerfile now handles the building of Rust libraries and the C++ code.
* **Multi-Stage Build**: The Dockerfile is optimized to reuse common dependencies and efficiently compile both the libraries and the user code, reducing the final image size and speeding up the build process.
* **Parametric Builds**: The Dockerfile is designed to be flexible, using build arguments to handle different examples and binary names. This makes it easy to add new examples or modify existing ones without needing to change the Dockerfile itself.

## Getting Started

The repository includes a launcher script that builds and runs the Docker containers for you.

### Dependencies

Since all examples are containerized, docker is the only dependency. Please make sure it's installed.

For Fedora or RHEL-based systems:
```bash
sudo dnf install docker
```

**Start the launcher**:
```bash
scripts/launcher.sh
```

**Select an Example**: The launcher will prompt you to select an example from the list below:
* Basic Integration Example
* Concurrency Example
* Producer-Consumer Example

**Enjoy Your Sandbox!**: The launcher will build and run your selected example.
* The first run will take longer as dependencies are resolved. Subsequent runs will be significantly faster.
* Monitor the output to understand how it works; modify the code and re-run to quickly prototype your custom implementations!

## Contributing

Contributions to this project are highly encouraged and greatly appreciated. If you have an example that showcases Rust-C++ interoperability or ideas for improving existing examples, please share them.

**How to Contribute**: 
1. Fork this repository.
2. Open a new branch for your changes.
3. Include a README.md (that follows the existing documentation conventions) in your example root.
4. Test your changes to ensure they work.
5. Submit a pull request.

## LICENSE

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.