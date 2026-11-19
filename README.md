# CS230H Homework 5: Concurrent Job Submissions

Three users submit jobs to the same scheduler. This example uses the jobs
and IDs from HW3 and HW4. The simulation and its test are in `src/main.rs`.

## Run

```sh
cargo run
cargo test
```

## Shared queue

The scheduler stores pending jobs in a `Vec<Job>` wrapped in an
`Arc<Mutex<Vec<Job>>>`. Each user thread owns one job and a cloned `Arc`.
Cloning the `Arc` gives the thread a handle to the same queue and keeps
the queue alive while the thread uses it.

Each user sleeps for a random 5 to 50 milliseconds to simulate a request
in transit. The sleep happens before locking, so another user can submit
a job during that delay. When the request arrives, the thread locks the
mutex and pushes its job. Only one thread can modify the vector at a time.
The lock guard drops at the end of the push statement.

## Final state

The main thread joins all three users before printing the queue. If the
email request acquires the lock first, `send_email` prints first, even
though the program spawned the backup thread first. All three jobs appear
exactly once. The test checks their names and IDs in any submission order.

## Assigned reading

- [Chapter 16, Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
