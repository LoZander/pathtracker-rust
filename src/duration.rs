use serde::{Deserialize, Serialize};

type Num = u16;

#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Duration {
    seconds: Num,
    actions: Num,
    turns: Num,
    minutes: Num,
    hours: Num,
    days: Num
}

impl Duration {
    pub fn builder() -> DurationBuilder {
        DurationBuilder::default()
    }

    pub fn from_seconds(n: Num) -> Self {
        Self::builder()
            .with_seconds(n)
            .build()
    }

    pub fn from_turns(n: Num) -> Self {
        Self::builder()
            .with_turns(n)
            .build()
    }

    pub fn from_minutes(n: Num) -> Self {
        Self::builder()
            .with_minutes(n)
            .build()
    }

    pub fn from_hours(n: Num) -> Self {
        Self::builder()
            .with_hours(n)
            .build()
    }

    pub fn in_seconds(self) -> Num {
        self.seconds + 2 * self.in_actions()
    }

    pub fn in_actions(self) -> Num {
        self.actions + 3 * self.in_turns()
    }

    pub fn in_turns(self) -> Num {
        self.turns + 10 * self.in_minutes()
    }

    pub fn in_minutes(self) -> Num {
        self.minutes + 60 * self.in_hours()
    }

    pub fn in_hours(self) -> Num {
        self.hours + 24 * self.in_days()
    }

    pub fn in_days(self) -> Num {
        self.days
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        let x = self.in_seconds();
        let y = rhs.in_seconds();

        Self::from_seconds(x.saturating_sub(y))
    }
}

impl std::ops::Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.in_seconds();
        let y = rhs.in_seconds();

        Self::from_seconds(x + y)
    }
}

impl std::ops::Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.in_seconds();
        let y = rhs.in_seconds();

        Self::from_seconds(x - y)
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct DurationBuilder {
    seconds: Option<Num>,
    actions: Option<Num>,
    turns: Option<Num>,
    minutes: Option<Num>,
    hours: Option<Num>,
    days: Option<Num>
}

impl DurationBuilder {
    pub fn build(self) -> Duration {
        let seconds  = self.seconds.unwrap_or(0);
        let actions  = self.actions.unwrap_or(0);
        let turns    = self.turns.unwrap_or(0);
        let minutes  = self.minutes.unwrap_or(0);
        let hours    = self.hours.unwrap_or(0);
        let days     = self.days.unwrap_or(0);

        let raw = RawDuration { seconds, actions, turns, minutes, hours, days };

        raw.normalize()
    }

    pub fn with_seconds(self, n: Num) -> Self {
        Self { seconds: Some(n), ..self }
    }

    pub fn with_actions(self, n: Num) -> Self {
        Self { actions: Some(n), ..self }
    }
    
    pub fn with_turns(self, n: Num) -> Self {
        Self { turns: Some(n), ..self }
    }
    
    pub fn with_minutes(self, n: Num) -> Self {
        Self { minutes: Some(n), ..self }
    }

    pub fn with_hours(self, n: Num) -> Self {
        Self { hours: Some(n), ..self }
    }

    pub fn with_days(self, n: Num) -> Self {
        Self { days: Some(n), ..self }
    }
}

struct RawDuration {
    seconds: Num,
    actions: Num,
    turns: Num,
    minutes: Num,
    hours: Num,
    days: Num
}

impl RawDuration {
    fn normalize(self) -> Duration {
        let normal = self
            .normalize_seconds()
            .normalize_actions()
            .normalize_turns()
            .normalize_minutes()
            .normalize_hours();

        Duration {
            seconds: normal.seconds,
            actions: normal.actions,
            turns: normal.turns,
            minutes: normal.minutes,
            hours: normal.hours,
            days: normal.days
        }
    }

    fn normalize_seconds(self) -> Self {
        let seconds = self.seconds % 2;
        let actions = self.actions + self.seconds / 2;
        Self { seconds, actions, ..self }
    }

    fn normalize_actions(self) -> Self {
        let actions = self.actions % 3;
        let turns = self.turns + self.actions / 3;
        Self { actions, turns, ..self }
    }

    fn normalize_turns(self) -> Self {
        let turns = self.turns % 10;
        let minutes = self.minutes + self.turns / 10;
        Self { turns, minutes, ..self }
    }

    fn normalize_minutes(self) -> Self {
        let minutes = self.minutes % 60;
        let hours = self.hours + self.minutes / 60;
        Self { minutes, hours, ..self }
    }

    fn normalize_hours(self) -> Self {
        let hours = self.hours % 24;
        let days = self.days + self.hours / 24;
        Self { hours, days, ..self }
    }
}

#[cfg(test)]
mod tests{
    use super::Duration;

    #[test]
    fn duration_from_5_turns_gives_duration_of_5_turns() {
        assert_eq!(5, Duration::from_turns(5).in_turns())
    }
}
