use core::time::Duration;
use kew_book_api::*;

static RUN_TIME: Label = Label::new("Run Time").description("The total runtime of the simulation");

struct Inputs {
    run_time: Duration,
}

impl Inputs {
    fn new(context: &mut Context) -> Self {
        let run_time = Duration::from_secs_f32(context.input(
            RUN_TIME,
            input::Range {
                step: 1.0,
                value: 1.0,
                ..Default::default()
            },
        ));

        Self { run_time }
    }
}

#[chapter]
fn fifo(context: &mut Context) {
    let inputs = Inputs::new(context);
    no_latency(context, &inputs);
    // fixed_server_latency();
    // random_server_latency();
    // load_shedding_latency();
}

fn no_latency(context: &mut Context, inputs: &Inputs) {
    let stats = context.sim(|| {
        let (client, server) = channel("fifo", Behavior::Unbounded);

        let client = async move {
            for message in 0.. {
                client.send_back(message).await.unwrap();

                1.ms().sleep().await;
            }
        };

        let server = async move {
            while let Ok(message) = server.recv_front().await {
                let _ = message;
                2.ms().sleep().await;
            }
        };

        run_for(inputs.run_time);
        client.group("client").spawn();
        server.group("server").spawn();
    });

    let latency_table = stats
        .filter(
            col("name")
                .eq(lit("sojourn_time"))
                .and(col("kind").eq(lit("measure"))),
        )
        .select_exprs(&["timestamp", "value"]);

    context.figure(
        Label::new("Queue Latency")
            .id("no-latency")
            .description("The latency of each item in the queue"),
        latency_table.figure(
            r#"
        ({ data }) => ({
            x: { grid: true, label: "Timestamp (ms)" },
            y: { grid: true, label: "Sojourn Time (ms)" },
            marks: [
                Plot.ruleY([0]),
                Plot.lineY(data, { x: "timestamp", y: "value", tip: true }),
            ],
        })
        "#,
        ),
    );
}

fn run_for(duration: Duration) {
    async move { duration.sleep().await }.primary().spawn();
}

/*


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

        math(r#"L = \frac{\lambda - \sigma}{\mu}"#);

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

    let tsv = sql_tsv(format_args!(
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

    if !reason.is_empty() {
        v.push_str(&format!("  AND attr_reason = '{reason}'"));
    }

    let tsv = sql_tsv(&v);

    vega(charts::count(tsv));
}
*/
