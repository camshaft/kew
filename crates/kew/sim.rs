use base::{def, model};

def!(
    pub fn PointToPoint(
        #[default = 1] queue_capacity: usize,
        #[default = 1] count: usize,
        #[default = 1.0] production_time: f32,
        #[default = 1] producer_capacity: usize,
        #[default = 1.0] consumption_time: f32,
        #[default = true] backpressure: bool,
        #[default = false] lifo: bool,
        #[default = false] prefer_recent: bool,
    ) {
        let queue_capacity = queue_capacity.max(1);
        let producer_capacity = producer_capacity.max(1);
        let count = count.max(1);
        let production_time =
            Duration::try_from_secs_f32(production_time).unwrap_or(Duration::from_secs(1));
        let consumption_time =
            Duration::try_from_secs_f32(consumption_time).unwrap_or(Duration::from_secs(2));

        model::register_group("Sender");

        let disc = if lifo {
            queue::vec_deque::Discipline::Lifo
        } else {
            queue::vec_deque::Discipline::Fifo
        };

        let overflow = if prefer_recent {
            queue::vec_deque::Overflow::PreferRecent
        } else {
            queue::vec_deque::Overflow::PreferOldest
        };

        let (mut sender, mut receiver) = queue::vec_deque::Queue::builder()
            .with_capacity(Some(queue_capacity))
            .with_discipline(disc)
            .with_overflow(overflow)
            .build()
            .items("Queue", disc)
            .mutex()
            .channel();

        model::register_group("Receiver");

        match (backpressure, producer_capacity) {
            (true, 1) => {
                async move {
                    for _ in 0..count {
                        let item = Default::default();
                        production_time.sleep().await;
                        let _ = sender.push(item).await;
                    }
                }
                .group("Sender")
                .primary()
                .spawn();
            }
            (false, 1) => {
                async move {
                    for _ in 0..count {
                        let item = Default::default();
                        production_time.sleep().await;
                        let _ = sender.push_nowait(item).await;
                    }
                }
                .group("Sender")
                .primary()
                .spawn();
            }
            (backpressure, producer_capacity) => {
                async move {
                    let (mut item_s, mut item_r) = queue::vec_deque::Queue::builder()
                        .with_capacity(Some(producer_capacity))
                        .build()
                        .mutex()
                        .channel();

                    async move {
                        for _ in 0..count {
                            let item = Default::default();
                            production_time.sleep().await;
                            let _ = item_s.push(item).await;
                        }
                    }
                    .spawn();

                    while let Ok(item) = item_r.pop().await {
                        if backpressure {
                            let _ = sender.push(item).await;
                        } else {
                            let _ = sender.push_nowait(item).await;
                        }
                    }
                }
                .group("Sender")
                .primary()
                .spawn();
            }
        }

        async move {
            consumption_time.sleep().await;
            while let Ok(i) = receiver.recv().await {
                println!("Received {}", i);
                consumption_time.sleep().await;
            }
        }
        .group("Receiver")
        .primary()
        .spawn();
    }
);
