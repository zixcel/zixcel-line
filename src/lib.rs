#![forbid(unsafe_code)]
#![doc = "Planning-only LINE observation boundary that excludes message content."]

mod boundary;
mod config;
mod plan;
mod reports;
#[cfg(test)]
mod unit_tests;

pub use boundary::ConnectorError;
pub use config::{ConnectorConfig, parse_config};
pub use plan::{ConnectorPlan, PlanStep, build_plan};
pub use reports::{
    CapabilityReport, DoctorReport, ValidationReport, capabilities, doctor, validation_report,
};

/// Stable connector identity.
pub const CONNECTOR: &str = "zixcel-line";
/// Provider family represented by this connector.
pub const PROVIDER: &str = "line";
/// Accepted configuration schema.
pub const CONFIG_SCHEMA: &str = "zixcel://line/config/v1";
/// Emitted planning contract schema.
pub const PLAN_SCHEMA: &str = "zixcel://contracts/connector-plan/v1";
