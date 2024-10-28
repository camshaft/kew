use super::api::*;
use std::path::Path;

#[test]
fn fifo() {
    title("fifo");

    md(r"
# FIFO

In this chapter we will explore the most basic type of queue: FIFO, or first-in-first-out.
    ");

    no_latency();
    fixed_server_latency();
    random_server_latency();
    load_shedding_latency();

    finish();
}

fn run_time() {
    async { 1000.ms().sleep().await }.primary().spawn();
}

fn no_latency() {
    let stats = sim(|| {
        let (client, server) = channel("fifo", Behavior::Unbounded);

        capture(true);

        let client = async move {
            for message in 0.. {
                client.send_back(message).await.unwrap();

                1.ms().sleep().await;
            }
        };

        let server = async move {
            while let Ok(message) = server.recv_front().await {
                println!("handled {message}");
            }
        };

        capture(false);

        run_time();
        client.group("client").spawn();
        server.group("server").spawn();
    });

    md("#### Latency");
    latency_graph(&stats, "explicit");
}

fn fixed_server_latency() {
    let stats = sim(|| {
        let (client, server) = channel("fifo", Behavior::Unbounded);

        capture(true);

        let client = async move {
            for message in 0.. {
                client.send_back(message).await.unwrap();

                1.ms().sleep().await;
            }
        };

        let server = async move {
            while let Ok(message) = server.recv_front().await {
                2.ms().sleep().await;

                println!("handled {message}");
            }
        };

        capture(false);

        run_time();
        client.group("client").spawn();
        server.group("server").spawn();
    });

    md("**Latency**");
    latency_graph(&stats, "explicit");
}

fn random_server_latency() {
    let stats = sim(|| {
        let (client, server) = channel("fifo", Behavior::Unbounded);

        capture(true);

        let client = async move {
            for message in 0.. {
                client.send_back(message).await.unwrap();

                rand::gen_range(500..=2000).us().sleep().await;
            }
        };

        let server = async move {
            while let Ok(message) = server.recv_front().await {
                rand::gen_range(1..=2).ms().sleep().await;

                println!("handled {message}");
            }
        };

        capture(false);

        run_time();
        client.group("client").spawn();
        server.group("server").spawn();
    });

    md("**Latency**");
    latency_graph(&stats, "explicit");
}

fn load_shedding_latency() {
    let stats = sim(|| {
        capture(true);
        let max_queue_depth = 75;
        capture(false);

        md("");
        md(r#"\\[ L = \frac{\lambda - \sigma}{\mu} \\]"#);
        md("");

        let (client, server) = channel("fifo", Behavior::Reject(max_queue_depth));

        let client = async move {
            for message in 0.. {
                client.send_back(message).await.unwrap();

                rand::gen_range(500..=2000).us().sleep().await;
            }
        };

        let server = async move {
            while let Ok(message) = server.recv_front().await {
                rand::gen_range(1..=2).ms().sleep().await;

                println!("handled {message}");
            }
        };

        run_time();
        client.group("client").spawn();
        server.group("server").spawn();
    });

    md("**Latency**");
    latency_graph(&stats, "explicit");

    md("**Errors**");
    count_graph(&stats, "push", "reject");
}

fn latency_graph<P: AsRef<Path>>(p: P, reason: &str) {
    let p = p.as_ref().display();

    let tsv = sql(format_args!(
        "
    SELECT
        epoch_ms(timestamp) as timestamp,
        value / 1000000 as value
    FROM read_parquet('{p}')
    WHERE
        name == 'sojourn_time'
        AND kind = 'measure'
        AND attr_reason = '{reason}'
    "
    ));

    vega(charts::latency(tsv));
}

fn count_graph<P: AsRef<Path>>(p: P, count: &str, reason: &str) {
    let p = p.as_ref().display();

    let tsv = sql(format_args!(
        "
    SELECT
        epoch_ms(timestamp) as timestamp,
        value
    FROM read_parquet('{p}')
    WHERE
        name == '{count}'
        AND kind = 'count'
        AND attr_reason = '{reason}'
    "
    ));

    vega(charts::count(tsv));
}
