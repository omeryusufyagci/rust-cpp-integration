# Basic Concurrency Example

This example provides an introductory look at concurrency-safe interoperability between Rust and C++ using a simple batch processing approach.

On the Rust side, mock temperature sensor data is generated, which is then batch-processed in C++, where a running average is calculated and displayed.

While this example serves as an entry point for experimenting with concurrency-safe interop between the two languages, for more advanced use cases, please see the `producer_consumer_example`, which more closely follows the producer-consumer pattern.

## Key Concepts
* Batch Processing with Synchronization: Demonstrates basic cross-language synchronization to ensure that data is fully generated in Rust before being processed in C++
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

## Explanation
* **Rust Code**: In this example, Rust is responsible for generating mock temperature data in a separate thread, simulating sensor readings. The data is stored in a thread-safe buffer, which is protected by a mutex to ensure safe access from multiple threads. Rust uses a condition variable to notify C++ when new data is available, ensuring that the C++ side only processes fully generated data.
* **C++ Code**: C++ processes the mock temperature data produced by Rust, calculating a running average. The synchronization between Rust and C++ ensures that data is fully generated before being processed, preventing race conditions and ensuring data consistency.
* **Synchronization Mechanisms**: This example uses mutexes and condition variables to manage concurrency. Rust produces the data and notifies C++ when the data is ready for processing. C++ waits for this signal, processes the data, and then waits for the next signal, ensuring that both languages operate in sync.