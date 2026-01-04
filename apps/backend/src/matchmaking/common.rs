use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::oneshot;

pub type RegistryMap = Arc<DashMap<i32, oneshot::Sender<String>>>;
