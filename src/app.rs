use color_eyre::eyre::Result;

use crate::db::Db;
use crate::model::{IssueSummary, Status};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Insert,
    Confirm,
}

pub struct ColumnState {
    pub status: Status,
    pub issues: Vec<IssueSummary>,
    pub total_count: usize,
    pub selected: usize,
    pub scroll_offset: usize,
}

impl ColumnState {
    fn new(status: Status) -> Self {
        Self {
            status,
            issues: Vec::new(),
            total_count: 0,
            selected: 0,
            scroll_offset: 0,
        }
    }

    pub fn select_next(&mut self) {
        if !self.issues.is_empty() && self.selected < self.issues.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    pub fn select_last(&mut self) {
        if !self.issues.is_empty() {
            self.selected = self.issues.len() - 1;
        }
    }

    pub fn clamp_selection(&mut self) {
        if self.issues.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.issues.len() {
            self.selected = self.issues.len() - 1;
        }
    }

    pub fn selected_issue(&self) -> Option<&IssueSummary> {
        self.issues.get(self.selected)
    }
}

pub struct App {
    pub db: Db,
    pub columns: [ColumnState; 3],
    pub active_column: usize,
    pub mode: AppMode,
    pub input: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let db = Db::open()?;
        let mut app = Self {
            db,
            columns: [
                ColumnState::new(Status::Todo),
                ColumnState::new(Status::InProgress),
                ColumnState::new(Status::Done),
            ],
            active_column: 0,
            mode: AppMode::Normal,
            input: String::new(),
            should_quit: false,
        };
        app.refresh_all()?;
        Ok(app)
    }

    pub fn active_col(&self) -> &ColumnState {
        &self.columns[self.active_column]
    }

    pub fn active_col_mut(&mut self) -> &mut ColumnState {
        &mut self.columns[self.active_column]
    }

    pub fn column_for_status(&self, status: Status) -> usize {
        match status {
            Status::Todo => 0,
            Status::InProgress => 1,
            Status::Done => 2,
        }
    }

    pub fn move_column_left(&mut self) {
        if self.active_column > 0 {
            self.active_column -= 1;
        }
    }

    pub fn move_column_right(&mut self) {
        if self.active_column < 2 {
            self.active_column += 1;
        }
    }

    pub fn refresh_all(&mut self) -> Result<()> {
        for col in &mut self.columns {
            col.issues = self.db.load_column(col.status)?;
            col.total_count = self.db.count_by_status(col.status)?;
            col.clamp_selection();
        }
        Ok(())
    }

    pub fn refresh_column(&mut self, idx: usize) -> Result<()> {
        let col = &mut self.columns[idx];
        col.issues = self.db.load_column(col.status)?;
        col.total_count = self.db.count_by_status(col.status)?;
        col.clamp_selection();
        Ok(())
    }

    pub fn create_issue(&mut self) -> Result<()> {
        let title = self.input.trim().to_string();
        if title.is_empty() {
            return Ok(());
        }
        let status = self.columns[self.active_column].status;
        self.db.create_issue(&title, status)?;
        self.input.clear();
        self.refresh_column(self.active_column)?;
        // Select the newly created issue (last in list)
        let col = &mut self.columns[self.active_column];
        if !col.issues.is_empty() {
            col.selected = col.issues.len() - 1;
        }
        Ok(())
    }

    pub fn delete_selected(&mut self) -> Result<()> {
        let col = &self.columns[self.active_column];
        if let Some(issue) = col.selected_issue() {
            let id = issue.id;
            self.db.delete_issue(id)?;
            self.refresh_column(self.active_column)?;
        }
        Ok(())
    }

    pub fn move_selected_forward(&mut self) -> Result<()> {
        let col = &self.columns[self.active_column];
        if let Some(issue) = col.selected_issue() {
            if let Some(next_status) = issue.status.next() {
                let id = issue.id;
                self.db.move_issue(id, next_status)?;
                let target_idx = self.column_for_status(next_status);
                self.refresh_column(self.active_column)?;
                self.refresh_column(target_idx)?;
            }
        }
        Ok(())
    }

    pub fn move_selected_backward(&mut self) -> Result<()> {
        let col = &self.columns[self.active_column];
        if let Some(issue) = col.selected_issue() {
            if let Some(prev_status) = issue.status.prev() {
                let id = issue.id;
                self.db.move_issue(id, prev_status)?;
                let target_idx = self.column_for_status(prev_status);
                self.refresh_column(self.active_column)?;
                self.refresh_column(target_idx)?;
            }
        }
        Ok(())
    }
}
