use chrono::{DateTime, TimeDelta, Utc};

mod post_system;
pub use post_system::*;

mod get_measurements_under_system;
pub use get_measurements_under_system::*;

mod get_measurements_under_system_and_label;
pub use get_measurements_under_system_and_label::*;

mod delete_system;
pub use delete_system::*;

mod get_system;
pub use get_system::*;

mod get_systems;
pub use get_systems::*;

mod put_system;
pub use put_system::*;

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct MeasurementRangeFilter {
    /// 計測開始日時 (省略時は現在時刻の1時間前)
    #[param(example = "2026-06-26T11:34:56Z", required = false)]
    from: DateTime<Utc>,
    /// 計測終了日時 (省略時は現在時刻)
    #[param(example = "2026-06-26T21:34:56Z", required = false)]
    to: DateTime<Utc>,
}

impl Default for MeasurementRangeFilter {
    fn default() -> Self {
        let to = Utc::now();
        let from = to - TimeDelta::hours(1);
        Self { to, from }
    }
}
