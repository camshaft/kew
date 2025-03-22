mod fifo;
// mod introduction;
mod aqm;
mod backpressure;
mod bbr;
mod cake;
mod capacity_management;
mod cca;
mod codel;
mod cubic;
mod lifo;
mod prefer_new;
mod prefer_old;
mod priority;
mod round_robin;
mod service_disciplines;
mod shortest_job_first;
mod unbounded;

#[test]
fn toc() {
    kew_book_api::toc!(
        (
            "Capacity Management",
            ["Unbounded", "Backpressure", "Prefer Old", "Prefer New",]
        ),
        (
            "Service Disciplines",
            [
                "FIFO: First In, First Out" as fifo,
                "LIFO: Last In, First Out" as lifo,
                "Round Robin",
                "Priority",
                "Shortest Job First"
            ]
        ),
        ("Active Queue Management" as aqm, ["CoDel", "CAKE",]),
        ("Congestion Control Algorithms" as cca, ["CUBIC", "BBR",]),
    )
    .render();
}
