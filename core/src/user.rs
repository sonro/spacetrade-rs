use crate::loan::Loan;
use crate::ship::Ship;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    username: String,
    credits: i64,
    ships: Vec<Ship>,
    loans: Vec<Loan>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    email: Option<String>,
    picture: Option<String>,
}
