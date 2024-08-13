#include <iostream>
#include <thread>
#include <chrono>

// Extern the Rust API with C linkage
extern "C" {
    struct SharedQueue;

    SharedQueue* make_shared_queue();
    void produce_task(SharedQueue* queue, int value);
    int consume_task(SharedQueue* queue);
    void start_producer(SharedQueue* queue);
}

class Consumer {
public:
    Consumer() {
        queue = make_shared_queue();
    }

    void consumeTask() {
        int value = consume_task(queue);
        std::cout << "C++ finished TASK-" << value << std::endl;
    }

    void startConsuming() {
        int nb_tasks = 10;

        std::thread consumer_thread([this, nb_tasks]() {
            for (int i = 0; i < nb_tasks; ++i) {
                consumeTask();
                std::this_thread::sleep_for(std::chrono::milliseconds(50)); // Introduce some delay to simulate work on a task
            }
        });
        consumer_thread.join(); // block main thread until consumer_thread finishes
    }

    void startProducer() {
        // Start the Rust producer
        start_producer(queue);
    }

private:
    SharedQueue* queue;
};

int main() {
    Consumer consumer;

    consumer.startProducer();
    consumer.startConsuming();

    return 0;
}
