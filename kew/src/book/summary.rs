use super::*;

#[test]
fn summary() {
    use api::*;

    title("SUMMARY");

    md("# Summary");
    md("[Introduction](./introduction.md)");

    md("- [Service Disciplines]()");
    md("  - [FIFO: First In, First Out](./fifo.md)");
    md("  - [LIFO: Last In, First Out]()");
    md("  - [Round Robin]()");
    md("  - [Priority](./priority.md)");
    md("  - [Shortest Job First]()");

    finish();
}
