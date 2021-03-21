use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Loan {
    id: String,
    repayment_amount: i64,
    status: String,
    r#type: String,
    due: DateTime<Utc>,
}
