use super::api::*;
use core::time::Duration;
use futures::{select_biased as select_priority, FutureExt, TryFutureExt};
use std::path::Path;

#[test]
fn priority() {
    title("priority");

    md(r"
# Priority


    ");

    first_example();

    finish();
}

fn run_time() {
    async { 1000.ms().sleep().await }.primary().spawn();
}

fn first_example() {
    let stats = sim(|| {
        type Request = (usize, usize);

        macro_rules! recv_front_open {
            ($q:ident) => {
                $q.recv_front()
                    .or_else(|_| core::future::pending::<Result<Request, channel::Closed>>())
                    .fuse()
            };
        }

        capture(true);

        let client = |name, burst, rounds, latency: Duration| {
            // TODO use backpressure of `burst`
            let (send, recv) = channel(name, Behavior::Backpressure(10000));

            async move {
                for round in 0..rounds {
                    for id in 0..burst {
                        send.send_back((round, id)).await.unwrap();
                    }

                    latency.sleep().await;
                }
            }
            .group("client")
            .spawn();

            recv
        };

        let client_a = client("client_a", 150, 3, 200.ms());
        let client_b = client("client_b", 10, 1000, 2.ms());

        async move {
            let burst = 5;

            loop {
                for _ in 0..burst {
                    select_priority! {
                        req = recv_front_open!(client_a) => {
                            println!("process {req:?} from client_a");
                        }
                        req = recv_front_open!(client_b) => {
                            println!("process {req:?} from client_b");
                        }
                    };
                }

                1.ms().sleep().await;
            }
        }
        .group("server")
        .spawn();

        capture(false);

        run_time();
    });

    md("## Client A");
    count_graph(&stats, "pop", "client_a");
    md("## Client B");
    count_graph(&stats, "pop", "client_b");
}

fn count_graph<P: AsRef<Path>>(p: P, count: &str, queue_name: &str) {
    let p = p.as_ref().display();

    let mut v = format!(
        "
    SELECT
        epoch_ms(timestamp) as timestamp,
        value
    FROM read_parquet('{p}')
    WHERE
        name == '{count}'
        AND kind = 'count'
    "
    );

    if !queue_name.is_empty() {
        v.push_str(&format!("  AND attr_queue_name = '{queue_name}'"));
    }

    let tsv = sql(&v);

    vega(charts::count(tsv));
}
