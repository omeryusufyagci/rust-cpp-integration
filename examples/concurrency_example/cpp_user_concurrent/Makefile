CXX = g++
CXXFLAGS = -std=c++14 -Wall -Wextra -pthread

SRCDIR = src
OBJDIR = obj
BINDIR = bin
LIBDIR = lib

# Target executable
TARGET = $(BINDIR)/concurrency_example

# Rust lib location
RUST_LIB_DIR = $(LIBDIR)
RUST_LIB = rusty_lib_concurrent

# Sources
SRCS = $(wildcard $(SRCDIR)/*.cpp)
OBJS = $(SRCS:$(SRCDIR)/%.cpp=$(OBJDIR)/%.o)

# Ensure dir location
$(shell mkdir -p $(OBJDIR) $(BINDIR))

# Linking
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) $(OBJS) -L$(RUST_LIB_DIR) -l$(RUST_LIB) -o $@

# Compiling
$(OBJDIR)/%.o: $(SRCDIR)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	rm -rf $(OBJS) $(BINDIR)/*

.PHONY: clean
