use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filter {
    category: Option<Vec<String>>,
    rating: Option<Vec<String>>,
}
