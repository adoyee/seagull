use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Debug, Copy, Clone)]
pub enum TransMode {
    None,
    Smart,
    Strict,
}

#[derive(Debug, Clone)]
pub struct Site {
    pub domain4: String,
    pub domain6: String,
    pub trans_mode: TransMode,
    pub smart_set: HashSet<String>,
    pub strict_set: HashSet<String>,
}

impl Site {
    pub fn new() -> Self {
        Self {
            domain4: String::new(),
            domain6: String::new(),
            trans_mode: TransMode::None,
            smart_set: HashSet::new(),
            strict_set: HashSet::new(),
        }
    }
}

type SiteDB = RwLock<HashMap<String, Arc<Site>>>;

pub struct Sites {
    sites: SiteDB,
}

impl Sites {
    pub fn new() -> Self {
        Self {
            sites: SiteDB::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &String) -> Option<Arc<Site>> {
        let sites = self.sites.read().unwrap();
        match sites.get(name) {
            None => None,
            Some(v) => Some(v.clone()),
        }
    }

    pub fn insert(&self, site: &Arc<Site>) {
        if site.domain4.is_empty() {
            return;
        }

        let mut sites = self.sites.write().unwrap();
        sites.insert(site.domain4.clone(), site.clone());

        if !site.domain6.is_empty() && site.domain4 != site.domain6 {
            sites.insert(site.domain6.clone(), site.clone());
        }
    }

    pub fn remove(&self, name: &String) {
        let mut sites = self.sites.write().unwrap();
        let site = sites.get(name);
        if site.is_none() {
            return;
        }

        let site = site.unwrap().clone();

        sites.remove(&site.domain4);
        if !site.domain6.is_empty() {
            sites.remove(&site.domain6);
        }
    }
}
