// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mobile: mobile component model.
//!
//! Real implementation: mobile_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobileItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MobileError {
    EmptyMobileItemName,
    DuplicateMobileItem(String),
    EmptyEventSubject,
}

impl fmt::Display for MobileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMobileItemName => write!(f, "mobile_item name must not be empty"),
            Self::DuplicateMobileItem(id) => write!(f, "duplicate mobile_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MobileError {}

pub struct Mobile {
    pub name: String,
    mobile_items: BTreeMap<String, MobileItem>,
    events: BTreeMap<String, Event>,
}

impl Mobile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mobile_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mobile_item(&mut self, item: MobileItem) -> Result<(), MobileError> {
        if item.name.trim().is_empty() {
            return Err(MobileError::EmptyMobileItemName);
        }
        let id = item.id.clone();
        if self.mobile_items.contains_key(&id) {
            return Err(MobileError::DuplicateMobileItem(id.clone()));
        }
        self.mobile_items.insert(id, item);
        Ok(())
    }

    pub fn mobile_item_count(&self) -> usize {
        self.mobile_items.len()
    }

    pub fn get_mobile_item(&self, id: &str) -> Option<&MobileItem> {
        self.mobile_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MobileError> {
        if event.subject.trim().is_empty() {
            return Err(MobileError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mobile_item(id: &str, name: &str) -> MobileItem {
        MobileItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Mobile::new("Mobile");
        assert_eq!(m.name, "Mobile");
        assert_eq!(m.mobile_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mobile_item() {
        let mut m = Mobile::new("Mobile");
        m.add_mobile_item(sample_mobile_item("a1", "Alpha")).unwrap();
        assert_eq!(m.mobile_item_count(), 1);
        assert!(m.get_mobile_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Mobile::new("Mobile");
        assert!(m.add_mobile_item(sample_mobile_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Mobile::new("Mobile");
        m.add_mobile_item(sample_mobile_item("x", "One")).unwrap();
        assert!(m.add_mobile_item(sample_mobile_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Mobile::new("Mobile");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
