#include <iostream>
#include <thread>

// Extern the Rust API with C linkage
extern "C" {
    struct TemperatureSensor;

    TemperatureSensor* generate_sensor();
    void start_sensor(TemperatureSensor* sensor, uint64_t seed);
    float* get_readings(TemperatureSensor* sensor);
    size_t get_readings_len(TemperatureSensor* sensor);
    void free_readings(float* readings);
    void wait_for_data(TemperatureSensor* sensor);
}

float calculate_running_average(float* readings, size_t len) {
    float sum = 0.0;
    for (size_t i = 0; i < len; ++i) {
        sum += readings[i];
    }
    return sum / len;
}

void process_temperatures(TemperatureSensor* sensor) {
    for (int i = 0; i < 10; ++i) {
        wait_for_data(sensor);  // Data ready will be signalled via condition variable from Rust

        size_t len = get_readings_len(sensor);
        if (len > 0) {
            float* readings = get_readings(sensor);
            float average = calculate_running_average(readings, len);
            std::cout << "C++: Running average temperature: " << average << std::endl;
            
            free_readings(readings);  // Rust will free the memory
        }
    }
}

int main() {
    TemperatureSensor* sensor = generate_sensor();

    // Start the sensor with a seed
    start_sensor(sensor, 1234);

    process_temperatures(sensor);

    return 0;
}
