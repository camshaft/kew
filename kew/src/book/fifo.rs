use super::api::*;

#[test]
fn fifo() {
    title("fifo");

    md(r"
# FIFO

In this chapter we will explore the most basic type of queue: FIFO, or first-in-first-out.
    ");

    let stats = sim(|| {
        let (client, server) = channel("fifo", None);

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

        async {
            2000.ms().sleep().await;
        }.primary().spawn();

        client.group("client").spawn();
        server.group("server").spawn();
    });

    let tsv = sql(format_args!("
    SELECT
        epoch_ms(timestamp) as timestamp,
        value / 1000000 as value
    FROM read_parquet('{}')
    WHERE
        name == 'sojourn_time'
        AND kind = 'measure'
    ", stats.display()));

    vega(charts::line(tsv));

    finish();
}
