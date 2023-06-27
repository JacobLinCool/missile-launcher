use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Line, Map, MapResolution, Rectangle},
        Clear, Paragraph,
    },
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Row, Sparkline,
        Table, Tabs,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };

    if app.typing {
        let area = centered_rect(60, 20, f.size());
        let block = Block::default()
            .title("Enter Launch Code")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .style(Style::default().bg(Color::Black));

        let text = vec![
            Spans::from(vec![Span::styled(
                "Press <Esc> to exit | <Enter> to launch | <Backspace> to delete",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            )]),
            Spans::from(""),
            Spans::from(vec![
                Span::from(" "),
                Span::from(app.code.as_str()),
                Span::from(" "),
            ]),
            Spans::from(""),
            Spans::from(vec![Span::styled(
                if app.code == app.correct_code {
                    "Correct Code! Press <Enter> to launch!"
                } else {
                    "Incorrect Code!"
                },
                Style::default().fg(if app.code == app.correct_code {
                    Color::Green
                } else {
                    Color::Red
                }),
            )]),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        // put the paragraph in block

        f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[1]);
    draw_packets(f, app, chunks[2]);
}

fn draw_gauges<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("System Health");
    f.render_widget(block, area);

    let label = format!("{:.2}%", app.power);
    let gauge = Gauge::default()
        .block(Block::default().title("Core Stress:"))
        .gauge_style(
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(app.power / 100.0);
    f.render_widget(gauge, chunks[0]);

    let sparkline = Sparkline::default()
        .block(Block::default().title("Broadcast Signal Strength:"))
        .style(Style::default().fg(Color::Green))
        .data(&app.sparkline.points)
        .bar_set(symbols::bar::NINE_LEVELS);
    f.render_widget(sparkline, chunks[2]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        .direction(Direction::Horizontal)
        .split(area);

    // Draw logs
    let info_style = Style::default().fg(Color::Blue);
    let warning_style = Style::default().fg(Color::Yellow);
    let error_style = Style::default().fg(Color::Magenta);
    let critical_style = Style::default().fg(Color::Red);
    let logs: Vec<ListItem> = app
        .logs
        .items
        .iter()
        .map(|&(evt, level)| {
            let s = match level {
                "ERROR" => error_style,
                "CRITICAL" => critical_style,
                "WARNING" => warning_style,
                _ => info_style,
            };
            let content = vec![Spans::from(vec![
                Span::styled(format!("{:<9}", level), s),
                Span::raw(evt),
            ])];
            ListItem::new(content)
        })
        .collect();
    let logs = List::new(logs).block(
        Block::default()
            .borders(Borders::ALL)
            .title("System Message"),
    );
    f.render_stateful_widget(logs, chunks[0], &mut app.logs.state);

    let x_labels = vec![
        Span::styled(
            format!("{}", app.signals.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "{}",
            (app.signals.window[0] + app.signals.window[1]) / 2.0
        )),
        Span::styled(
            format!("{}", app.signals.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![
        Dataset::default()
            .name("CS Wave")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.signals.sin1.points),
        Dataset::default()
            .name("IE Wave")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.signals.sin2.points),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Signals",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("T (cycle)")
                .style(Style::default().fg(Color::Gray))
                .bounds(app.signals.window)
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Strength (dBm)")
                .style(Style::default().fg(Color::Gray))
                .bounds([-20.0, 20.0])
                .labels(vec![
                    Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, chunks[1]);
}

fn draw_packets<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Packets"))
        .data(&app.packets)
        .bar_width(3)
        .bar_gap(2)
        .bar_set(symbols::bar::NINE_LEVELS)
        .value_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .label_style(Style::default().fg(Color::Yellow))
        .bar_style(Style::default().fg(Color::Cyan));
    f.render_widget(barchart, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);
    let rows = app.launchers.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::new(vec![s.name, s.location, s.status]).style(style)
    });
    let table = Table::new(rows)
        .header(
            Row::new(vec!["Launcher", "Location", "Status"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(Block::default().title("Launchers").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(10),
        ]);
    f.render_widget(table, chunks[0]);

    let map = Canvas::default()
        .block(Block::default().title("World Map").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();

            for launcher in &app.launchers {
                let color = if launcher.status == "Up" {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.print(
                    launcher.coords.1,
                    launcher.coords.0,
                    Span::styled("x", Style::default().fg(color)),
                );
            }

            if !app.missile_launched {
                ctx.draw(&Line {
                    x1: -180.0,
                    y1: -5.0,
                    x2: 180.0,
                    y2: 25.0,
                    color: Color::Cyan,
                });

                ctx.draw(&Rectangle {
                    x: 90.0,
                    y: 7.0,
                    width: 20.0,
                    height: 20.0,
                    color: Color::Cyan,
                });

                ctx.print(100.0, 17.0, Span::styled("🛰️", Style::default()));
            }
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, chunks[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
