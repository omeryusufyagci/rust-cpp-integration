#!/bin/bash

set -e

echo "Select an example to build and run:"
echo "1) Basic Integration Example"
echo "2) Basic Concurrency Example"
echo "3) Producer-Consumer Example"
read -p "Enter choice [1-3]: " choice

case $choice in
# TODO: provide a utility that will auto-generate the variables from example dir name. 
# Probably will require following naming conventions for examples

    1)
        RUST_LIB_DIR="examples/basic_integration_example/rusty_lib"
        CPP_PROJECT_DIR="examples/basic_integration_example/cpp_user"
        BINARY_NAME="greeter_example"
        ;;
    2)
        RUST_LIB_DIR="examples/concurrency_example/rusty_lib_concurrent"
        CPP_PROJECT_DIR="examples/concurrency_example/cpp_user_concurrent"
        BINARY_NAME="concurrency_example"
        ;;
    3)
        RUST_LIB_DIR="examples/producer_consumer_example/rusty_lib_producer_consumer"
        CPP_PROJECT_DIR="examples/producer_consumer_example/cpp_user_producer_consumer"
        BINARY_NAME="producer_consumer_example"
        ;;
    *)
        echo "Invalid choice."
        exit 1
        ;;
esac

echo "Building and running ${CPP_PROJECT_DIR} with Rust library ${RUST_LIB_DIR} and binary ${BINARY_NAME}..."

docker build \
    --build-arg RUST_LIB_DIR=${RUST_LIB_DIR} \
    --build-arg CPP_PROJECT_DIR=${CPP_PROJECT_DIR} \
    --build-arg BINARY_NAME=${BINARY_NAME} \
    -t rust_cpp_example .

docker run --rm rust_cpp_example
