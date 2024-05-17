use crate::RespFrame;
use dashmap::{DashMap, DashSet};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    pub(crate) map: DashMap<String, RespFrame>,
    pub(crate) hmap: DashMap<String, DashMap<String, RespFrame>>,
    pub(crate) setmap: DashMap<String, DashSet<String>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::default()))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
            setmap: DashMap::new(),
        }
    }
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }

    pub fn sadd(&self, key: &str, member: &str) -> RespFrame {
        // check if the key exists in the setmap, if it does not, add the member to the set and return 1; else
        // if the member already exists in the set, return 0; else add the member to the set and return 1
        let set = self.setmap.entry(key.to_string()).or_default();
        if set.contains(member) {
            RespFrame::Integer(0)
        } else {
            set.insert(member.to_string());
            RespFrame::Integer(1)
        }
    }

    pub fn sismember(&self, key: &str, member: &str) -> RespFrame {
        //check if the key and the member exists in the setmap, if they do, return 1; else return 0
        if let Some(set) = self.setmap.get(key) {
            if set.contains(member) {
                return RespFrame::Integer(1);
            }
        }

        RespFrame::Integer(0)
    }
}
