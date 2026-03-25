use super::model::PaintDocument;
use crate::domain::PaintTab;

impl PaintDocument {
    pub fn add_tab_with_title(&mut self, title: String) -> u64 {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        self.tabs.push(PaintTab::new(id, title));
        self.active_tab_id = id;
        id
    }

    pub fn set_active_tab(&mut self, id: u64) {
        if self.tabs.iter().any(|tab| tab.id == id) {
            self.active_tab_id = id;
        }
    }

    pub fn rename_tab(&mut self, id: u64, title: String) -> bool {
        let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) else {
            return false;
        };
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return false;
        }
        tab.title = trimmed.to_string();
        true
    }

    pub fn remove_tab(&mut self, id: u64) -> bool {
        if self.tabs.len() <= 1 {
            return false;
        }
        let before = self.tabs.len();
        self.tabs.retain(|tab| tab.id != id);
        if self.tabs.len() == before {
            return false;
        }
        if self.active_tab_id == id {
            if let Some(tab) = self.tabs.first() {
                self.active_tab_id = tab.id;
            }
        }
        true
    }

    pub fn remove_tab_or_create_new(&mut self, id: u64, fallback_title: String) -> bool {
        if self.tabs.len() == 1 {
            let only_id = self.tabs[0].id;
            if only_id != id {
                return false;
            }
            self.tabs.clear();
            let _ = self.add_tab_with_title(fallback_title);
            return true;
        }
        self.remove_tab(id)
    }
}
