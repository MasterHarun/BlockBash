use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::text::{Span, Text};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::buffer::Buffer;
use ratatui::widgets::{Widget, StatefulWidget};
use core::panic;

use crate::app::{InputMode, Stats, UserInput};
use crate::colors::RgbSwatch;
use crate::root::layout;
use crate::theme::THEME;
use crate::widget::Spinner; // If you need to store history or a list of inputs

pub struct DashboardTab<'a> {
    // Example fields - adjust according to your needs
    pub input_ctx:&'a mut UserInput,
    pub stats: Stats

    // Add other fields as necessary
}

impl<'a> DashboardTab<'a> {
    pub fn new(input_ctx: &'a mut UserInput) -> DashboardTab<'a> {
        DashboardTab {
            input_ctx,
            stats: Stats::new(),
            // Initialize other fields here
        }
    }
    
    // Add methods to manipulate the dashboard state here
}

impl<'a> Widget for DashboardTab<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
       // Clear the entire area first
       Clear.render(area, buf);

       // Split the main area into three rows
       let rows = Layout::default()
           .direction(Direction::Vertical)
           .margin(1) // Adding a margin around the entire layout
           .constraints([
               Constraint::Length(6),
               Constraint::Min(8),
               Constraint::Length(7),
           ])
           .split(area);

       // Split the first row into two columns (30/70)
       let top_row_columns = Layout::default()
           .direction(Direction::Horizontal)
           .constraints([
               Constraint::Percentage(30),
               Constraint::Fill(1),
               Constraint::Percentage(65),
           ])
           .split(rows[0]);

       // Split the first column of the first row again for the two stats
       let stats_areas = Layout::default()
           .direction(Direction::Vertical)
           .constraints([
               Constraint::Ratio(1, 2)// Stat 2
               ])
               .split(top_row_columns[0]);// else { panic!("Unable to create stats area") };
       
       //  Input box
       let address_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
        ])
        .split(top_row_columns[2]);

        // self.render_input_box(chunks[0], buf);
        // self.render_output_box(chunks[1], buf);
        // self.render_price_box(chunks[1], buf);
        self.render_statistics(stats_areas[0], buf);
        // self.render_input_box(address_box[0], buf);
        self.render_input_box_with_arrows(address_box[0], buf);
        // self.input_ctx.set_box_pos((chunks[0].x, chunks[0].y));
    }

}

impl<'a> DashboardTab<'a> {
    fn render_input_box(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Input");
        
        // Calculate the inner area considering the block's borders
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Layout for arrows and input box
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                // Constraint::Length(3), // Left arrow area
                Constraint::Min(10),   // Input text area
                // Constraint::Length(3), // Right arrow area
            ])
            .split(inner_area);
        let inner = Layout::new(Direction::Horizontal, 
            [Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
            .split(chunks[0]);
        // Render left arrow
        let left_arrow_paragraph = Paragraph::new("<")
            .block(Block::default().borders(Borders::RIGHT));
        left_arrow_paragraph.render(inner[0], buf);

        // Render right arrow
        let right_arrow_paragraph = Paragraph::new(">")
            .block(Block::default().borders(Borders::LEFT));
        right_arrow_paragraph.render(inner[2], buf);

        // Render the input text
        let input_text_paragraph = Paragraph::new(self.input_ctx.input.as_str())
            .block(Block::default().borders(Borders::NONE));
        input_text_paragraph.render(inner[1], buf);
        
    }

    fn render_input_box_with_arrows(&self, area: Rect, buf: &mut Buffer) {
        // Render a block around the entire area for the input box
        let block = Block::default().borders(Borders::ALL).title("Input");
        
        // Calculate the inner area considering the block's borders
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Define layout constraints for arrows and the input box
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3), // Left arrow area
                Constraint::Min(10),   // Input text area
                Constraint::Length(3), // Right arrow area
            ])
            .split(inner_area);

        // Render left and right arrows
        self.render_arrow(chunks[0], buf, "left");
        self.render_arrow(chunks[2], buf, "right");

        // Render the text input area
        let input_text = self.input_ctx.input.as_str();
        let input_paragraph = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::NONE)).alignment(Alignment::Center); // No border as it's inside the main block
        input_paragraph.render(chunks[1], buf);
    }

    fn render_arrow(&self, area: Rect, buf: &mut Buffer, direction: &str) {
        let arrow_symbol = if direction == "left" { "<" } else { ">" };
        let paragraph = Paragraph::new(arrow_symbol)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
    fn render_output_box(&self, area: Rect, buf: &mut Buffer) {
        let messages_items: Vec<ListItem> = self.input_ctx.messages.iter()
        .map(|msg| {
            let msg_span = Span::raw(msg.clone());
            ListItem::new(Text::from(msg_span))
        })
        .collect();
        let messages_list = List::new(messages_items)
            .block(Block::default().borders(Borders::ALL).title("Messages"));
        ratatui::widgets::Widget::render(messages_list, area, buf);
    }

    fn render_statistics(&self, area: Rect, buf: &mut Buffer) {
        let [l_stat, r_stat] = *Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(area) else { panic!("Unable to create rect") };
    
        let statistics = [
            ("SOL PRICE", self.stats.solusd.map(|p| format!("{:.2} USD", p)).unwrap_or_else(|| Spinner::default().to_string())),
            ("BALANCE", self.stats.balance.map(|b| format!("{} balance", b)).unwrap_or_else(|| Spinner::default().to_string())),
        ];
    
        let areas = [l_stat, r_stat];
    
        for (area, (title, value)) in areas.iter().zip(statistics.iter()) {
            let block = Block::default().title(*title).borders(Borders::ALL);
            let paragraph = Paragraph::new(value.clone())
                .block(block)
                .alignment(Alignment::Right)
                .wrap(Wrap { trim: true });
    
            paragraph.render(*area, buf);
        }
    }
    

}

