use core::time::Duration;

pub trait LiteralDuration {
    fn s(self) -> Duration;
    fn ms(self) -> Duration;
    fn us(self) -> Duration;
    fn ns(self) -> Duration;
}

impl LiteralDuration for u64 {
    fn s(self) -> Duration {
        Duration::from_secs(self)
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self)
    }

    fn us(self) -> Duration {
        Duration::from_micros(self)
    }

    fn ns(self) -> Duration {
        Duration::from_nanos(self)
    }
}

pub trait GroupExt {
    type Output;

    fn group<N: core::fmt::Display>(self, name: N) -> Self::Output;
}

impl<F> GroupExt for F {
    type Output = F;

    fn group<N: core::fmt::Display>(self, name: N) -> Self::Output {
        todo!();
        self
    }
}

pub trait SpawnExt {
    type Output;

    fn spawn(self) -> Self::Output;
}

impl<F> SpawnExt for F {
    type Output = F;

    fn spawn(self) -> Self::Output {
        todo!()
    }
}

pub trait PrimaryExt {
    type Output;

    fn primary(self) -> Self::Output;
}

impl<F> PrimaryExt for F {
    type Output = F;

    fn primary(self) -> Self::Output {
        todo!()
    }
}
