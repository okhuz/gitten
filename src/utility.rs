use std::fmt::Display;
use std::path::PathBuf;
use git2::{Repository};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, ListItem};
use crate::App;
use crate::app::Selection;

pub fn is_repository(path: PathBuf) -> bool {
    match Repository::open(path) {
        Ok(_repo) => true,
        _error => false
    }
}

pub fn get_repository(path: PathBuf) -> Option<Repository>{
    match Repository::open(path) {
        Ok(repo) => Some(repo),
        Err(_e) => None
    }
}

pub fn get_repository_tags(repository: &Option<Repository>) -> Vec<String> {
    let mut tags = Vec::new();
    if let Some(r) = repository {
        r.tag_names(Some("[0-99999999].[0-99999999].[0-99999999]")).iter().for_each(|f| {
            f.iter().for_each(|x| {
                if let Some(tag) = x {
                    tags.push(tag.to_string());
                };
            });
        });
    }
    tags
}

pub fn get_repository_branches(repository: &Option<Repository>) -> Vec<String> {
    let mut branches_string = Vec::new();

    if let Some(r) = repository {
        let branches = match r.branches(None) {
            Ok(branches) => Some(branches),
            Err(_) => None
        };

        branches.unwrap().for_each(|b| {
            let b1 = b.unwrap().0.name().unwrap().unwrap().to_string();
            branches_string.push(b1);
        });
    }
    branches_string
}

pub fn get_repository_active_branch(repository: &Option<Repository>) -> String {
    let mut branch_id: String = "".to_string();
    if let Some(r) = repository {
        branch_id = r.head().unwrap().name().unwrap().replace("refs/heads/", "").to_string()
    }
    branch_id
}

pub fn convert_to_list_item<T: Display>(iterator: &Vec<T>) -> Vec<ListItem<'static>> {
    iterator.iter()
        .rev()
        .map(|f| {
            ListItem::new(vec![
                Spans::from(vec![
                    Span::raw(format!("{}", f))
                ])
            ])
        })
        .collect()
}

pub fn create_block_with_title(app: &App, selection: Selection) -> Block<'static> {
    let b = Block::default();

    let style = if app.selection == selection {
        Style::default().bg(Color::White).fg(Color::Black).add_modifier(Modifier::BOLD)
    } else {
        Style::default().bg(Color::Black).fg(Color::White)
    };

    b.borders(Borders::ALL)
        .title(Spans::from(vec![
            Span::styled(selection.to_string(), style)
        ]))
}

pub fn create_block() -> Block<'static> {
    let b = Block::default();
    b.borders(Borders::NONE)
}