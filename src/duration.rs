use serde::{Deserialize, Serialize};

type Num = u32;

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
    #[must_use]
    pub fn builder() -> DurationBuilder {
        DurationBuilder::default()
    }

    #[must_use]
    pub fn from_seconds(n: Num) -> Self {
        Self::builder().with_seconds(n).build()
    }

    #[must_use]
    pub fn from_actions(n: Num) -> Self {
        Self::builder().with_actions(n).build()
    }

    #[must_use]
    pub fn from_turns(n: Num) -> Self {
        Self::builder().with_turns(n).build()
    }

    #[must_use]
    pub fn from_minutes(n: Num) -> Self {
        Self::builder().with_minutes(n).build()
    }

    #[must_use]
    pub fn from_hours(n: Num) -> Self {
        Self::builder().with_hours(n).build()
    }

    #[must_use]
    pub fn from_days(n: Num) -> Self {
        Self::builder().with_days(n).build()
    }

    #[must_use]
    pub fn in_seconds(self) -> Num {
        self.seconds + 2 * self.in_actions()
    }

    #[must_use]
    pub fn in_actions(self) -> Num {
        self.actions + 3 * self.in_turns()
    }

    #[must_use]
    pub fn in_turns(self) -> Num {
        self.turns + 10 * self.in_minutes()
    }

    #[must_use]
    pub fn in_minutes(self) -> Num {
        self.minutes + 60 * self.in_hours()
    }

    #[must_use]
    pub fn in_hours(self) -> Num {
        self.hours + 24 * self.in_days()
    }

    #[must_use]
    pub fn in_days(self) -> Num {
        self.days
    }

    #[must_use]
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
    #[must_use]
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

    #[must_use]
    pub fn with_seconds(self, n: Num) -> Self {
        Self { seconds: Some(n), ..self }
    }

    #[must_use]
    pub fn with_actions(self, n: Num) -> Self {
        Self { actions: Some(n), ..self }
    }
    
    #[must_use]
    pub fn with_turns(self, n: Num) -> Self {
        Self { turns: Some(n), ..self }
    }
    
    #[must_use]
    pub fn with_minutes(self, n: Num) -> Self {
        Self { minutes: Some(n), ..self }
    }

    #[must_use]
    pub fn with_hours(self, n: Num) -> Self {
        Self { hours: Some(n), ..self }
    }

    #[must_use]
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
        assert_eq!(5, Duration::from_turns(5).in_turns());
    }

    #[test]
    fn duration_from_1_min_gives_duration_of_30_turns() {
        assert_eq!(10, Duration::from_minutes(1).in_turns());
    }

    #[test]
    fn duration_from_1_turn_gives_duration_of_3_actions() {
        assert_eq!(3, Duration::from_turns(1).in_actions());
    }

    #[test]
    fn duration_from_6_seconds_gives_duration_of_1_turn() {
        assert_eq!(1, Duration::from_seconds(6).in_turns());
    }

    #[test]
    fn duration_from_1_action_gives_duration_of_2_seconds() {
        assert_eq!(2, Duration::from_actions(1).in_seconds());
    }

    #[test]
    fn duration_from_1_day_gives_duration_of_14_400_turns() {
        assert_eq!(14_400, Duration::from_days(1).in_turns());
    }
}
