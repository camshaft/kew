use super::api::*;

#[test]
fn summary() {
    title("SUMMARY");

    md("# Summary");
    md("[Introduction](./introduction.md)");

    md("- [Capacity Management]()");
    md("  - [Unbounded]()");
    md("  - [Backpressure]()");
    md("  - [Prefer Old]()");
    md("  - [Prefer New]()");

    md("- [Service Disciplines]()");
    md("  - [FIFO: First In, First Out](./fifo.md)");
    md("  - [LIFO: Last In, First Out]()");
    md("  - [Round Robin]()");
    md("  - [Priority](./priority.md)");
    md("  - [Shortest Job First]()");

    md("- [Active Queue Management]()");
    md("  - [CoDel]()");
    md("  - [CAKE]()");

    md("- [Congestion Control Algorithms]()");
    md("  - [CUBIC]()");
    md("  - [BBR]()");

    finish();
}
