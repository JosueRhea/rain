use sysinfo::{CpuExt, SystemExt};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::state::State;

pub fn cpu_monitor(state: &mut State) -> List {
    // Misc
    let brand = state.system.global_cpu_info().brand();
    let usage = state.system.global_cpu_info().cpu_usage();
    let cpus = state.system.cpus();
    let memory_available = state.system.available_memory() as f64 / 1024_f64.powi(3);
    let total_memory = state.system.total_memory() as f64 / 1024_f64.powi(3);

    // let comp = state.system.components().get(0).unwrap();

    let texts = vec![
        format!("CPU: {}%", usage.ceil()),
        format!("Brand: {}", brand),
        format!("Cores: {}", cpus.len()),
        format!("Memory: {:.2} of {:.2}", memory_available, total_memory),
    ];

    let spans = (0..texts.len()).map(|i| {
        Spans::from(Span::styled(
            texts[i].clone(),
            Style::default().fg(Color::White),
        ))
    });

    let items: Vec<ListItem> = spans.map(|span| ListItem::new(span)).collect();
    let component = List::new(items).block(
        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Plain)
            .title("System"),
    );
    return component;
}
