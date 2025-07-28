use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;

struct App {
    log_list_state: ListState,
    log_entries: Vec<&'static str>,
}

impl App {
    fn new() -> App {
        let log_entries = vec![
            "[2024-07-28 17:30:15] INFO: Powers of Tau ceremony initialized",
            "[2024-07-28 17:30:16] INFO: Loading ceremony configuration (power=8)",
            "[2024-07-28 17:30:17] INFO: Participant #1 joined the ceremony",
            "[2024-07-28 17:30:45] INFO: Participant #1 contribution verified successfully",
            "[2024-07-28 17:31:12] INFO: Participant #2 joined the ceremony",
            "[2024-07-28 17:31:58] INFO: Participant #2 contribution verified successfully", 
            "[2024-07-28 17:32:15] INFO: Participant #3 joined the ceremony",
            "[2024-07-28 17:32:42] WARN: Participant #3 connection unstable, retrying...",
            "[2024-07-28 17:32:45] INFO: Participant #3 reconnected successfully",
            "[2024-07-28 17:33:01] INFO: Computing phase 1 parameters...",
            "[2024-07-28 17:33:15] INFO: Phase 1 progress: 65% complete",
            "[2024-07-28 17:33:30] INFO: Waiting for participant #4 to join...",
            "[2024-07-28 17:33:45] INFO: Participant #4 joined the ceremony",
            "[2024-07-28 17:34:12] INFO: Participant #4 contribution processing...",
            "[2024-07-28 17:34:30] INFO: Phase 1 progress: 70% complete",
            "[2024-07-28 17:34:45] INFO: Validating cryptographic proofs...",
            "[2024-07-28 17:35:01] INFO: All participants verified successfully",
            "[2024-07-28 17:35:15] INFO: Generating final parameters...",
            "[2024-07-28 17:35:30] INFO: Phase 1 progress: 85% complete",
            "[2024-07-28 17:35:45] INFO: Ceremony phase 1 nearly complete...",
        ];

        let mut app = App {
            log_list_state: ListState::default(),
            log_entries,
        };
        
        // Start with the last (most recent) log selected
        app.log_list_state.select(Some(app.log_entries.len() - 1));
        app
    }

    fn next_log(&mut self) {
        let i = match self.log_list_state.selected() {
            Some(i) => {
                if i >= self.log_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.log_list_state.select(Some(i));
    }

    fn previous_log(&mut self) {
        let i = match self.log_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.log_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.log_list_state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app instance
    let mut app = App::new();

    // Run the application
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next_log(),
                KeyCode::Up => app.previous_log(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    // Create three rectangles from top to bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33), // Top rectangle
            Constraint::Percentage(34), // Middle rectangle  
            Constraint::Percentage(33), // Bottom rectangle
        ])
        .split(f.size());

    // Top rectangle - Application Info
    let top_block = Block::default()
        .title("Cardano Phase1 Trusted Setup")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    
    let top_content = "Powers of Tau Ceremony - Terminal User Interface\n\n\
        This application manages the Powers of Tau trusted setup ceremony for Cardano.\n\
        The ceremony generates cryptographic parameters for zk-SNARKs.\n\n\
        Controls:\n\
        • Press 'q' to quit\n\
        • Use ↑↓ arrow keys to navigate logs\n\
        • Press Enter to select options (future)\n\n\
        Status: Ready to begin ceremony";
    
    let top_paragraph = Paragraph::new(top_content)
        .block(top_block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    
    f.render_widget(top_paragraph, chunks[0]);

    // Middle rectangle - Status with progress bar
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // Left side for status bar
            Constraint::Min(0),     // Right side for status text
        ])
        .split(chunks[1]);

    // Status bar on the left
    let progress = 65; // Example progress value
    let status_bar = Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(progress)
        .label(format!("{}%", progress));
    
    f.render_widget(status_bar, status_chunks[0]);

    // Status text on the right
    let status_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));
    
    let status_content = "Running\n\n\
        Current Step: Computing participant contribution\n\
        Participants: 3/10\n\
        Time Elapsed: 00:15:42\n\
        Estimated Remaining: 00:45:18\n\n\
        Last Action: Verified contribution from participant #2\n\
        Next: Waiting for participant #4";
    
    let status_paragraph = Paragraph::new(status_content)
        .block(status_block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    
    f.render_widget(status_paragraph, status_chunks[1]);

    // Bottom rectangle - Logs (List widget with keyboard navigation)
    let log_items: Vec<ListItem> = app.log_entries
        .iter()
        .map(|log| {
            let style = if log.contains("WARN") {
                Style::default().fg(Color::Yellow)
            } else if log.contains("ERROR") {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*log).style(style)
        })
        .collect();

    let logs_list = List::new(log_items)
        .block(
            Block::default()
                .title("Logs (Use ↑↓ arrows to navigate)")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue))
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
        .highlight_symbol(">> ");
    
    f.render_stateful_widget(logs_list, chunks[2], &mut app.log_list_state);
}