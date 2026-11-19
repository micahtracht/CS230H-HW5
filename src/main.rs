use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Job {
    name: String,
    id: u16,
}

impl Job {
    fn new(name: &str, id: u16) -> Job {
        Job {
            name: String::from(name),
            id,
        }
    }

    fn show(&self) {
        println!("{:<16} id=0x{:04X}", self.name, self.id);
    }
}

fn submit_jobs() -> Vec<Job> {
    let queue = Arc::new(Mutex::new(Vec::new()));
    let mut users = Vec::new();

    for job in [
        Job::new("backup", 0x1234),
        Job::new("send_email", 0x9876),
        Job::new("generate_report", 0x6767),
    ] {
        let queue = Arc::clone(&queue);
        users.push(thread::spawn(move || {
            let delay = Duration::from_millis(rand::random_range(5..=50));
            thread::sleep(delay); // each user's request takes time to arrive
            queue.lock().unwrap().push(job);
        }));
    }

    for user in users {
        user.join().unwrap();
    }
    // Only the main thread still owns the queue.
    Arc::into_inner(queue).unwrap().into_inner().unwrap()
}

fn main() {
    let jobs = submit_jobs();
    println!("Final job queue (first submission first):");
    for job in &jobs {
        job.show();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_users_job_reaches_the_queue_once() {
        let jobs = submit_jobs();
        let mut entries: Vec<_> = jobs.iter().map(|job| (job.name.as_str(), job.id)).collect();
        entries.sort_unstable();
        assert_eq!(
            entries,
            [
                ("backup", 0x1234),
                ("generate_report", 0x6767),
                ("send_email", 0x9876),
            ]
        );
    }
}
