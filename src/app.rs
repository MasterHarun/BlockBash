use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default, Clone)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
    pub input: UserInput,
    pub stats: Stats,
}
#[derive(Clone, Default, Debug)]
pub struct Stats {
    pub solusd: Option<f64>,
    pub balance: Option<f64>

}
impl Stats {
    pub fn new() -> Self {
        Self {
            solusd: Some(42.0),
            balance: None,
        }
    }
    pub const SOLUSD_INDEX: usize = 0;
    pub const BALANCE_INDEX: usize = 1;
}


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputContext {
    pub box_pos: (u16, u16),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInput {
    pub input: String,
    pub box_pos: (u16, u16),
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}
impl Default for UserInput {
    fn default() -> UserInput {
        UserInput {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_position: 0,
            box_pos: (0, 0),
        }
    }
}

impl UserInput {
    pub fn set_box_pos(&mut self, box_pos: (u16,u16)) {
        self.box_pos = box_pos;
    }
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub context: AppContext,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            context: AppContext::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;

        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn next_tab(&mut self) {
        let tab_index = self.context.tab_index.saturating_add(1) % TAB_COUNT;
        self.context.tab_index = tab_index; 
        self.context.row_index = 0;
    }
    
    pub fn back_tab(&mut self) {
        let tab_index = self.context.tab_index + TAB_COUNT;
        self.context.tab_index = tab_index.saturating_sub(1) % TAB_COUNT;  
        self.context.row_index = 0;

    }

}
const TAB_COUNT: usize = 3;

/// User Input methods
impl App {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.context.input.cursor_position.saturating_sub(1);
        self.context.input.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.context.input.cursor_position.saturating_add(1);
        self.context.input.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.context.input.input.insert(self.context.input.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.context.input.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.context.input.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.context.input.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.context.input.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.context.input.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.context.input.input.len())
    }

    fn reset_cursor(&mut self) {
        self.context.input.cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        self.context.input.messages.push(self.context.input.input.clone());
        self.context.input.input.clear();
        self.reset_cursor();
    } 

 
}