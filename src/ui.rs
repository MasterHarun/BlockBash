use ratatui::Frame;

use crate::{app::{App, InputMode}, root::Root};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(Root::new(&mut app.context), frame.size());
    
    if app.context.input.input_mode == InputMode::Editing {
        // Calculate cursor's X position within the input area
        // Note: This calculation might need adjustment based on your input box's layout and padding
        let cur = app.context.input.box_pos;
        let cursor_x = cur.0 + app.context.input.cursor_position as u16 + 1; // +1 for left padding or border
        let cursor_y = cur.1 + 1; // Assuming single-line input, adjust if your input box has top padding or border
            
        frame.set_cursor(cursor_x, cursor_y);
    }
}

