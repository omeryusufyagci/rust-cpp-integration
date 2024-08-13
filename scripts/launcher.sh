#!/bin/bash

set -e

echo "Select an example to build and run:"
echo "1) Basic Integration Example"
echo "2) Basic Concurrency Example"
echo "3) Producer-Consumer Example"
read -p "Enter choice [1-3]: " choice

case $choice in
    1)
        echo "Building and running Basic Integration Example..."
        (cd examples/basic_integration_example && docker build -t rust_lib_builder -f Dockerfile.rhel9 .)
        docker run --rm rust_lib_builder
        (cd examples/basic_integration_example && docker build -t cpp_project_tester -f Dockerfile.ubuntu .)
        docker run --rm cpp_project_tester
        ;;
    2)
        echo "Building and running Concurrency Example..."
        (cd examples/concurrency_example && docker build -t rust_concurrent_lib_builder -f Dockerfile.rhel9 .)
        docker run --rm rust_concurrent_lib_builder
        (cd examples/concurrency_example && docker build -t cpp_concurrent_project_tester -f Dockerfile.ubuntu .)
        docker run --rm cpp_concurrent_project_tester
        ;;
    3)
        echo "Building and running Producer-Consumer Example..."
        (cd examples/producer_consumer_example && docker build -t rust_producer_consumer_builder -f Dockerfile.rhel9 .)
        docker run --rm rust_producer_consumer_builder
        (cd examples/producer_consumer_example && docker build -t cpp_producer_consumer_tester -f Dockerfile.ubuntu .)
        docker run --rm cpp_producer_consumer_tester
        ;;
    *)
        echo "Invalid choice."
        ;;
esac
