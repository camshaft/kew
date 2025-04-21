use std::time::Duration;

use base::{def, model};

def!(
    pub fn AcceptQueue(
        #[default = 1] queue_capacity: usize,
        #[default = 1] baseline_client_count: usize,
        #[default = 1] baseline_client_tps: usize,
        #[default = 10.0] baseline_duration: f32,
        #[default = 1] burst_client_count: usize,
        #[default = 1] burst_client_tps: usize,
        #[default = 5.0] burst_duration: f32,
        #[default = 10.0] recovery_duration: f32,
        #[default = 0.10] service_time: f32,
        #[default = 1] worker_count: usize,
        #[default = false] lifo: bool,
        #[default = false] prefer_recent: bool,
        #[default = 0] codel_multiplier: u32,
    ) {
        let queue_capacity = queue_capacity.max(1);

        let baseline_client_tps = baseline_client_tps.max(1);
        let baseline_client_delay = Duration::from_secs_f32(1.0 / baseline_client_tps as f32);
        let baseline_duration = Duration::from_secs_f32(baseline_duration);

        let burst_client_tps = burst_client_tps.max(1);
        let burst_client_delay = Duration::from_secs_f32(1.0 / burst_client_tps as f32);
        let burst_duration = Duration::from_secs_f32(burst_duration);

        let recovery_duration = Duration::from_secs_f32(recovery_duration);
        let service_time = Duration::from_secs_f32(service_time);

        let total_time = baseline_duration + burst_duration + recovery_duration;

        model::register_group("Clients");

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

        let (sender, receiver) = queue::vec_deque::Queue::builder()
            .with_capacity(Some(queue_capacity))
            .with_discipline(disc)
            .with_overflow(overflow)
            .build()
            .items("Accept Queue", disc)
            .mutex()
            .channel();

        model::register_group("Server");

        for _ in 0..baseline_client_count {
            let mut sender = sender.clone();
            async move {
                let start = Instant::now();
                while start.elapsed() < total_time {
                    let item = Default::default();
                    baseline_client_delay.sleep().await;
                    let _ = sender.push_nowait(item).await;
                }
            }
            .group("Clients")
            .primary()
            .spawn();
        }

        for _ in 0..burst_client_count {
            let mut sender = sender.clone();
            async move {
                baseline_duration.sleep().await;

                let start = Instant::now();

                while start.elapsed() < burst_duration {
                    let item = Default::default();
                    burst_client_delay.sleep().await;
                    let _ = sender.push_nowait(item).await;
                }
            }
            .group("Clients")
            .primary()
            .spawn();
        }

        for _ in 0..worker_count {
            let mut receiver = receiver.clone();
            async move {
                let mut is_first = true;
                let mut smoothed_sojourn_time = Duration::from_secs(60);

                while let Ok(item) = receiver.recv().await {
                    let sojourn_time = item.sojourn_time();

                    if codel_multiplier > 0 {
                        if smoothed_sojourn_time * codel_multiplier < sojourn_time {
                            item.error();
                            continue;
                        }

                        // update the sojurn time since we accepted it
                        if core::mem::take(&mut is_first) {
                            smoothed_sojourn_time = sojourn_time;
                        } else {
                            smoothed_sojourn_time =
                                weighted_average(smoothed_sojourn_time, sojourn_time, 8);
                        }
                    }

                    service_time.sleep().await;
                    drop(item);
                }
            }
            .group("Server")
            .primary()
            .spawn();
        }
    }
);

/// Optimized function for averaging two durations with a weight
/// See https://godbolt.org/z/65f9bYEcs
#[inline]
fn weighted_average(a: Duration, b: Duration, weight: u64) -> Duration {
    let mut a = a.as_nanos() as u64;
    // it's more accurate to multiply first but it risks overflow so we divide first
    a /= weight;
    a *= weight - 1;

    let mut b = b.as_nanos() as u64;
    b /= weight;

    Duration::from_nanos(a + b)
}
