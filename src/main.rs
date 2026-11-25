use std::cmp::Ordering;

struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        (self.salary > other.salary) && (self.commute_time > other.commute_time)
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)
    }
}

fn main() {
    let long_commute_job = Job {
        salary: 1,
        commute_time: 2,
    };

    let short_commute_job = Job {
        salary: 0,
        commute_time: 1,
    };

    println!("{}", long_commute_job > short_commute_job);
}
