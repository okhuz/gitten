use std::{io};
use std::fmt::Display;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use tui::backend::Backend;
use tui::{Frame, Terminal};
use tui::widgets::{Block, List, ListItem, Paragraph};
use tui::layout::{Alignment, Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use crate::{App};
use crate::app::{AlfredRepository, Selection};
use crate::utility::{convert_to_list_item, create_block, create_block_with_title};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| {
            ui( f, &mut app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.repositories.unselect(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('r') => app.change_selection(Selection::REPOSITORIES),
                    KeyCode::Char('t') => app.change_selection(Selection::TAGS),
                    KeyCode::Char('b') => app.change_selection(Selection::BRANCHES),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now()
        }
    }
}

fn ui<'a, B: Backend>(f: &'a mut Frame<B>, app: &'a mut App) {
    let size = f.size();

    // Big chunk divides screen for part and bottom info
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(95),
                Constraint::Percentage(5)
            ]
        )
        .split(size);

    // Divides main part into two
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
        .split(chunks[0]);

    // Files & folders
    let items: Vec<ListItem> = app
        .repositories
        .items
        .iter()
        .map(|i| {
            convert_alfred_repository_to_list_item(i, &main_chunks[0])
        })
        .collect();

    // Repositories
    let items = List::new(items)
        .block(create_block().title("Repositories"))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, main_chunks[0], &mut app.repositories.state);

    // Info at the bottom
    let paragraph = Paragraph::new(format!("{}",  app.selected_repository_path))
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block())
        .alignment(Alignment::Left);

    f.render_widget(paragraph, chunks[1]);

    //Branches and Tags screens
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
        .split(main_chunks[1]);

    // Tags

    let tag_list = create_selection_list(&app.tags.items, create_block_with_title(&app, Selection::TAGS));
    f.render_stateful_widget(tag_list, right_chunks[0], &mut app.tags.state);

    // Branches
    let branch_list = create_selection_list(&app.branches.items, create_block_with_title(&app, Selection::BRANCHES));
    f.render_stateful_widget(branch_list, right_chunks[1], &mut app.branches.state);
}

fn convert_alfred_repository_to_list_item<'a>(item: &'a AlfredRepository, chunk: &'a Rect) -> ListItem<'a> {
    let mut lines: Spans = Spans::default();
    let mut line_color = Color::Black;
    if item.is_repository {
        lines.0.push(Span::from(item.folder_name.clone()));
        lines.0.push(Span::from(" ".repeat((chunk.width - (item.active_branch_name.len() as u16) - (item.folder_name.len() as u16) - 6) as usize)));
        lines.0.push(Span::raw("("));
        lines.0.push(Span::from(item.active_branch_name.to_string()));
        lines.0.push(Span::raw(")"));
        line_color = Color::Green
    } else {
        lines.0.push(Span::from(item.folder_name.clone()));
    }
    ListItem::new(lines).style(Style::default().fg(Color::White).bg(line_color))
}

fn create_selection_list<'a, T: Display>(v: &'a Vec<T>, b: Block<'a>) -> List<'a > {
    List::new(convert_to_list_item(v))
        .block(b)
        .start_corner(Corner::TopLeft)
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}