use ratatui::style::Color;

// Dark theme inspired by Linear
pub const BG_BASE: Color = Color::Rgb(18, 18, 20);
pub const BG_COLUMN: Color = Color::Rgb(25, 25, 28);
pub const BG_CARD: Color = Color::Rgb(32, 32, 36);
pub const BG_CARD_SELECTED: Color = Color::Rgb(45, 42, 58);
pub const BG_INPUT: Color = Color::Rgb(38, 38, 44);

pub const FG_PRIMARY: Color = Color::Rgb(225, 225, 230);
pub const FG_SECONDARY: Color = Color::Rgb(160, 160, 170);
pub const FG_DIM: Color = Color::Rgb(90, 90, 100);
pub const FG_ACCENT: Color = Color::Rgb(129, 140, 248);
pub const FG_BORDER: Color = Color::Rgb(50, 50, 58);
pub const FG_BORDER_ACTIVE: Color = Color::Rgb(99, 102, 241);

pub const STATUS_TODO: Color = Color::Rgb(148, 163, 184);
pub const STATUS_IN_PROGRESS: Color = Color::Rgb(251, 191, 36);
pub const STATUS_DONE: Color = Color::Rgb(52, 211, 153);

pub const PRIORITY_HIGH: Color = Color::Rgb(248, 113, 113);
pub const PRIORITY_MEDIUM: Color = Color::Rgb(251, 191, 36);
pub const PRIORITY_LOW: Color = Color::Rgb(96, 165, 250);
pub const PRIORITY_NONE: Color = Color::Rgb(90, 90, 100);

pub const FG_CONFIRM: Color = Color::Rgb(248, 113, 113);

// Gradient colors for the SLATE logo (purple -> blue -> cyan)
pub const GRAD_1: Color = Color::Rgb(168, 85, 247);
pub const GRAD_2: Color = Color::Rgb(139, 92, 246);
pub const GRAD_3: Color = Color::Rgb(99, 102, 241);
pub const GRAD_4: Color = Color::Rgb(59, 130, 246);
pub const GRAD_5: Color = Color::Rgb(34, 211, 238);

// Card left-border accent per status
pub const CARD_ACCENT_TODO: Color = Color::Rgb(100, 116, 139);
pub const CARD_ACCENT_IN_PROGRESS: Color = Color::Rgb(245, 158, 11);
pub const CARD_ACCENT_DONE: Color = Color::Rgb(34, 197, 94);
