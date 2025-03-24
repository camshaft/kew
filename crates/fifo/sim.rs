use base::def;

def!(
    pub fn Fifo(depth: usize, seconds: usize) {
        let (mut sender, mut receiver) = queue::vec_deque::Queue::builder()
            .with_capacity(Some(depth))
            .build()
            .latent(2.ms())
            .mutex()
            .channel();

        async move {
            for i in 0u32.. {
                let _ = sender.send(i).await;
            }
        }
        .spawn();

        async move {
            while let Ok(i) = receiver.recv().await {
                println!("Received {}", i);
            }
        }
        .spawn();

        async move {
            sleep((seconds as u64).s()).await;
        }
        .primary()
        .spawn();
    }
);
