#include <iostream>
#include <memory>
#include <cstring>

// Forward declare the Rust struct
struct RustGreeter;

extern "C" {
    RustGreeter* generate_greeter(const char* name);
    void greeter_say_hello(RustGreeter* greeter);
    void destroy_greeter(RustGreeter* greeter);
}

// C++ base class acting as a wrapper for the Rust struct
class Greeter {
public:
    explicit Greeter(const std::string& name) {
        greeter = generate_greeter(name.c_str());
        if (!greeter) {
            throw std::runtime_error("Failed to generate Greeter");
        }
    }

    virtual ~Greeter() {
        if (greeter) {
            destroy_greeter(greeter);
        }
    }

    virtual void sayHello() const {
        if (greeter) {
            greeter_say_hello(greeter);
        }
    }

protected:
    RustGreeter* greeter;
};

// C++ derived class
class CustomGreeter : public Greeter {
public:
    explicit CustomGreeter(const std::string& name) : Greeter(name) {}

    void sayHello() const override {
        Greeter::sayHello();
        std::cout << "CustomGreeter says: Hello from C++, extending a Rust Struct!" << std::endl;  // need to flush to ensure order of output
    }

};

int main() {
    // Stack allocation
    CustomGreeter stackGreeter("Stack Allocated C++ User");
    stackGreeter.sayHello();

    // Heap allocation with raw pointers
    CustomGreeter* heapGreeter = new CustomGreeter("Heap Allocated C++ User");
    heapGreeter->sayHello();
    delete heapGreeter;

    // Heap allocation with smart pointers
    std::unique_ptr<CustomGreeter> smartGreeter = std::make_unique<CustomGreeter>("Smart Pointer C++ User");
    smartGreeter->sayHello();

    return 0;
}
