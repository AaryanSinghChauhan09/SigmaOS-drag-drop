// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-drag-drop: drag drop component model.
//!
//! Real implementation: drops, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drop {
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
pub enum DragDropError {
    EmptyDropName,
    DuplicateDrop(String),
    EmptyEventSubject,
}

impl fmt::Display for DragDropError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDropName => write!(f, "drop name must not be empty"),
            Self::DuplicateDrop(id) => write!(f, "duplicate drop {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DragDropError {}

pub struct DragDrop {
    pub name: String,
    drops: BTreeMap<String, Drop>,
    events: BTreeMap<String, Event>,
}

impl DragDrop {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            drops: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_drop(&mut self, item: Drop) -> Result<(), DragDropError> {
        if item.name.trim().is_empty() {
            return Err(DragDropError::EmptyDropName);
        }
        let id = item.id.clone();
        if self.drops.contains_key(&id) {
            return Err(DragDropError::DuplicateDrop(id.clone()));
        }
        self.drops.insert(id, item);
        Ok(())
    }

    pub fn drop_count(&self) -> usize {
        self.drops.len()
    }

    pub fn get_drop(&self, id: &str) -> Option<&Drop> {
        self.drops.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DragDropError> {
        if event.subject.trim().is_empty() {
            return Err(DragDropError::EmptyEventSubject);
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

    fn sample_drop(id: &str, name: &str) -> Drop {
        Drop {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DragDrop::new("DragDrop");
        assert_eq!(m.name, "DragDrop");
        assert_eq!(m.drop_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_drop() {
        let mut m = DragDrop::new("DragDrop");
        m.add_drop(sample_drop("a1", "Alpha")).unwrap();
        assert_eq!(m.drop_count(), 1);
        assert!(m.get_drop("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DragDrop::new("DragDrop");
        assert!(m.add_drop(sample_drop("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DragDrop::new("DragDrop");
        m.add_drop(sample_drop("x", "One")).unwrap();
        assert!(m.add_drop(sample_drop("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DragDrop::new("DragDrop");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
