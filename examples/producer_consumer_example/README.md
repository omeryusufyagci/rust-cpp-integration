# Producer-Consumer Example

This example demonstrates a typical implementation of the producer-consumer pattern, showcasing concurrency-safe interoperability between Rust and C++.

In this example, Rust acts as the producer, generating a sequence of tasks and placing them in a thread-safe queue. C++ acts as the consumer, dequeuing these tasks by finishing them.

This example is meant to provide a more advanced look at how to manage shared resources between Rust and C++ in a multi-threaded environment, highlighting the use of synchronization mechanisms to ensure safe and efficient concurrency.

## Key Concepts
* Producer-Consumer Pattern with Synchronization: Demonstrates how to implement the classic producer-consumer pattern using a shared, thread-safe queue protected by a mutex and synchronized with a condition variable.
* Cross-Language Integration: Shows how to share data, manage memory, and synchronize operations between Rust and C++ in a concurrent setting.

# How to Run

From the project root, run:

```bash
scripts/launcher.sh # Select `Producer-Consumer Example`
```

## Expected Output

You should see output similar to the following, where Rust produces some tasks and C++ consumes them:

```
Rust produced: TASK-1
C++ finished TASK-1
Rust produced: TASK-2
C++ finished TASK-2
Rust produced: TASK-3
C++ finished TASK-3
Rust produced: TASK-4
C++ finished TASK-4
Rust produced: TASK-5
C++ finished TASK-5
Rust produced: TASK-6
C++ finished TASK-6
Rust produced: TASK-7
C++ finished TASK-7
Rust produced: TASK-8
C++ finished TASK-8
Rust produced: TASK-9
C++ finished TASK-9
Rust produced: TASK-10
C++ finished TASK-10
```

## Explanation
* **Rust Code**: Rust acts as the producer, generating a predefined number of tasks and placing them into a thread-safe queue. The queue is protected by a mutex to ensure access safety. A condition variable is used to notify the C++ consumer when new tasks are available. This setup ensures that Rust can safely and efficiently produce tasks without interfering with the C++ side.
* **C++ Code**: C++ acts as the consumer, retrieving tasks from the queue and processing them. The consumer waits for tasks to be available, processes each task, and then waits for the next one. The condition variable used by Rust ensures that C++ is notified as soon as a new task is ready, allowing for efficient task processing without busy-waiting.
* **Producer-Consumer Pattern**: This example demonstrates a typical producer-consumer pattern in a multi-threaded, cross-language context. Rust and C++ work together to manage shared resources, with Rust producing tasks and C++ consuming them. The use of mutexes and condition variables ensures that the two languages operate efficiently and safely in a concurrent environment.