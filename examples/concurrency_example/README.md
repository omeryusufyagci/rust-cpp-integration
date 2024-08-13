# Basic Concurrency Example

This example provides an introductory look at concurrency-safe interoperability between Rust and C++ using a simple batch processing approach.

On the Rust side, mock temperature sensor data is generated, which is then batch-processed in C++, where a running average is calculated and displayed.

While this example serves as an entry point for experimenting with concurrency-safe interop between the two languages, for more advanced use cases, please see the `producer_consumer_example`, which more closely follows the producer-consumer pattern.

## Key Concepts
* Batch Processing with Synchronization: Demonstrates basic synchronization to ensure that data is fully generated in Rust before being processed in C++
* Introduction to Cross-Language Integration: The example illustrates how to share data, manage memory, and synchronize operations between Rust and C++.

## How to run

From project root, run:

```bash
scripts/launcher.sh # Select `Basic Concurrency Example`
```

### Expected Output

The mock temperature data is seeded, so you should see the following output:

```
Rust: Generated temperature: 20.28
C++: Running average temperature: 20.2829
Rust: Generated temperature: 20.59
C++: Running average temperature: 20.4376
Rust: Generated temperature: 24.98
C++: Running average temperature: 21.952
Rust: Generated temperature: 20.85
C++: Running average temperature: 21.676
Rust: Generated temperature: 21.57
C++: Running average temperature: 21.6558
Rust: Generated temperature: 23.08
C++: Running average temperature: 21.8927
Rust: Generated temperature: 23.52
C++: Running average temperature: 22.125
Rust: Generated temperature: 21.90
C++: Running average temperature: 22.0966
Rust: Generated temperature: 23.45
C++: Running average temperature: 22.2475
Rust: Generated temperature: 23.22
C++: Running average temperature: 22.3448
```