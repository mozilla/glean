use crate::metrics::CounterMetric;
use crate::CommonMetricData;
use crate::Lifetime;

#[derive(Debug)]
pub enum ErrorType {
    InvalidValue,
    InvalidLabel,
}

impl ErrorType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ErrorType::InvalidValue => "invalid_value",
            ErrorType::InvalidLabel => "invalid_label",
        }
    }
}

pub fn record_error(meta: &CommonMetricData, error: ErrorType) {
    let identifier = meta.identifier();

    let mut send_in_pings = meta.send_in_pings.clone();
    if !send_in_pings.contains(&"metrics".to_string()) {
        send_in_pings.push("metrics".into());
    }

    let metric = CounterMetric::new(CommonMetricData {
        name: format!("{}/{}", error.to_string(), identifier),
        category: "glean.error".into(),
        lifetime: Lifetime::Ping,
        send_in_pings,
        ..Default::default()
    });

    metric.add(1);
}
