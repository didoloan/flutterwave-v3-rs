use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Interval {
    Yearly,
    Quarterly,
    Monthly,
    Weekly,
    Daily,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Yearly => f.write_str("yearly"),
            Interval::Quarterly => f.write_str("quarterly"),
            Interval::Monthly => f.write_str("monthly"),
            Interval::Weekly => f.write_str("weekly"),
            Interval::Daily => f.write_str("daily"),
        }
    }
}
