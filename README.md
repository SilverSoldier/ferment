# ferment

concurrent grep clone written in rust.

`ferment` is a grep clone written in rust which uses parallelism.

It has not been tested for scaling or for speed wrt grep.

Tests have also not been written yet.

## Parallelism
+ For parallelism, I have used the master-worker model.
+ Main thread is master.
+ It creates workers and holds handles for communication (*Receiver*) and for joining (*JoinHandle*) purposes.
+ It recursively lists files under directories and deals them to threads in a round-robin fashion for searching purposes.
