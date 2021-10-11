//! Interact with your dashboard lists through the API to make it easier to organize, find, and share all of your dashboards with your team and organization.

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::client::{*};
