# ferment

concurrent grep clone written in rust.

`ferment` is a grep clone written in rust which uses master-worker model for parallelism.

The project was purely for learning purposes. It served as a great way to learn some basic concurrency in rust.

## Parallelism
+ For parallelism, I have used the master-worker model.
+ Main thread is master.
+ It creates workers and holds handles for communication (*Receiver*) and for joining (*JoinHandle*) purposes.
+ It recursively lists files under directories and deals them to threads in a round-robin fashion for searching purposes.

## Usage
Clone repository and `cargo build`.

`ferment -h` gives usage instructions.
