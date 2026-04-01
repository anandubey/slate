use chrono::NaiveDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => "todo",
            Status::InProgress => "in_progress",
            Status::Done => "done",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Done => "Done",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Status::Todo => "○",
            Status::InProgress => "◐",
            Status::Done => "●",
        }
    }

    pub fn all() -> [Status; 3] {
        [Status::Todo, Status::InProgress, Status::Done]
    }

    pub fn next(&self) -> Option<Status> {
        match self {
            Status::Todo => Some(Status::InProgress),
            Status::InProgress => Some(Status::Done),
            Status::Done => None,
        }
    }

    pub fn prev(&self) -> Option<Status> {
        match self {
            Status::Todo => None,
            Status::InProgress => Some(Status::Todo),
            Status::Done => Some(Status::InProgress),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "todo" => Ok(Status::Todo),
            "in_progress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            other => Err(format!("Unknown status: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

impl Priority {
    pub fn icon(&self) -> &'static str {
        match self {
            Priority::None => "  ",
            Priority::Low => "▁▂",
            Priority::Medium => "▃▄",
            Priority::High => "▆▇",
        }
    }

    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IssueSummary {
    pub id: i64,
    pub issue_id: String,
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    pub sort_order: f64,
    pub created_at: NaiveDateTime,
}

impl IssueSummary {
    pub fn card_height(&self) -> u16 {
        // border top(1) + id+priority(1) + title(1) + created date(1) + border bottom(1)
        5
    }
}
