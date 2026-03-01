//! Task value objects

use crate::shared::domain::value_objects::string_id;
use serde::{Deserialize, Serialize};

string_id!(TaskId, "Task");
