use base::{def, model};

def!(
    pub fn Fifo(depth: usize, count: usize, arrival_rate: f32, departure_rate: f32) {
        let count = count.max(1);
        let arrival_rate =
            Duration::try_from_secs_f32(arrival_rate).unwrap_or(Duration::from_secs(1));
        let departure_rate =
            Duration::try_from_secs_f32(departure_rate).unwrap_or(Duration::from_secs(2));

        model::register_group("Sender");

        let (mut sender, mut receiver) = queue::vec_deque::Queue::builder()
            .with_capacity(Some(depth))
            .build()
            .items("Queue")
            .mutex()
            .channel();

        model::register_group("Receiver");

        async move {
            for _ in 0..count {
                let item = Default::default();
                arrival_rate.sleep().await;
                let _ = sender.send(item).await;
            }
        }
        .group("Sender")
        .primary()
        .spawn();

        async move {
            departure_rate.sleep().await;
            while let Ok(i) = receiver.recv().await {
                println!("Received {}", i);
                departure_rate.sleep().await;
            }
        }
        .group("Receiver")
        .primary()
        .spawn();
    }
);
