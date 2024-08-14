# Base stage: To reuse dependencies accross different stages
FROM ubuntu:22.04 AS base

# Install common dependencies to both Rust and C++
RUN apt-get update && apt-get install -y \
    gcc \
    g++ \
    make \
    curl \
    git \
    && apt-get clean

# Keeping it in base for now, although only required for Rust stage
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Stage 1: Build Rust library
FROM base AS rust_builder

ARG RUST_LIB_DIR
WORKDIR /usr/src/rust_lib
COPY ${RUST_LIB_DIR}/ .
RUN cargo build --release --target=x86_64-unknown-linux-gnu

# Stage 2: Build the C++ user using the Rust library
FROM base AS cpp_builder

ARG CPP_PROJECT_DIR
WORKDIR /usr/src/cpp_project
COPY ${CPP_PROJECT_DIR}/ .

# Copy the Rust .so from Stage 1
COPY --from=rust_builder /usr/src/rust_lib/target/x86_64-unknown-linux-gnu/release/*.so ./lib/

# Make the library visible
ENV LD_LIBRARY_PATH=/usr/src/cpp_project/lib:$LD_LIBRARY_PATH
RUN make

# Ensure permissions
RUN chmod +x ./bin/${BINARY_NAME}

# Run the binary 
# TODO: had to use shell form, problems on some machines otherwise, to be checked
ARG BINARY_NAME
ENV BINARY_NAME=${BINARY_NAME}
CMD sh -c "./bin/${BINARY_NAME}" 
