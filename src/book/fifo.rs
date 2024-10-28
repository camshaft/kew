use super::api::*;

#[test]
fn fifo() {
    let db = duckdb::Connection::open_in_memory().unwrap();

    title("01_FIFO");

    md(r"
# FIFO

In this chapter we will explore the most basic type of queue: FIFO, or first-in-first-out.
    ");

    capture(true);
    let foo = 1;
    let bar = 2;

    assert_eq!(foo + bar, 3);
    capture(false);

    let stats = sim(|| {
        let (client, server) = channel("fifo", None);

        capture(true);

        let client = async move {
            for req in 0..1000 {
                client.send_back(req).await.unwrap();

                sleep(1.ms()).await;
            }
        };

        let server = async move {
            while let Ok(req) = server.recv_front().await {
                sleep(2.ms()).await;

                println!("done {req}");
            }
        };

        capture(false);

        client.group("client").primary().spawn();
        server.group("server").primary().spawn();
    });

    let tsv = {
        let out = stats.with_extension("tsv");

        db.execute(
            &format!(
                "
            COPY (
                SELECT
                    epoch_ms(timestamp) as t_ms,
                    value / 1000000 as value
                FROM read_parquet('{}')
                WHERE
                    name == 'sojourn_time'
                    AND kind = 'measure'
            ) TO '{}'
            (FORMAT CSV, DELIM '\t')
            ",
                stats.display(),
                out.display()
            ),
            [],
        )
        .unwrap();

        emit_file(out)
    };

    dbg!(tsv);

    finish();
}
