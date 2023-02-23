use serde::{Deserialize, Serialize};

use crate::{errors::CommandResult, schema::Item};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filter {
    category: Option<Vec<String>>,
    rating: Option<Vec<String>>,
}
