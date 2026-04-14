use crate::action::Actionable;
use posty::{AppEvent, ResponseData};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Row, Wrap};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{StatefulWidget, Table, Widget},
};
enum ResponseArea {
    Cookies,
    Body,
    Headers,
}
pub struct ResponseDisplay {
    response: Option<ResponseData<'static>>,
    focused_area: ResponseArea,
}
impl Widget for &ResponseDisplay {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let Some(response) = &self.response else {
            return;
        };

        // Split into top bar and content area
        let [bar, content] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        // --- Top Bar ---
        let status = response.status.as_str();
        let timestamp = response.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        let response_ms = response.response_time.as_millis();
        let body_size = response
            .body
            .as_deref()
            .map(|b| format!("{}B", b.len()))
            .unwrap_or_else(|| "0B".to_string());

        let bar_text = format!(
            " {} │ {} │ {}ms │ {}",
            status, timestamp, response_ms, body_size
        );

        let status_style = match response.status.as_u16() {
            200..=299 => Style::new().fg(Color::Green),
            300..=399 => Style::new().fg(Color::Yellow),
            400..=499 => Style::new().fg(Color::Red),
            500..=599 => Style::new().fg(Color::Magenta),
            _ => Style::new(),
        };

        Line::from(vec![
            Span::styled(status, status_style.bold()),
            Span::raw(format!(
                " │ {} │ {}ms │ {}",
                timestamp, response_ms, body_size
            )),
        ])
        .render(bar, buf);

        match self.focused_area {
            ResponseArea::Body => {
                let body_text = response.body.as_deref().unwrap_or("");
                Paragraph::new(body_text)
                    .block(Block::bordered().title(" Body "))
                    .wrap(Wrap { trim: false })
                    .render(content, buf);
            }
            ResponseArea::Headers => {
                let rows: Vec<Row> = response
                    .headers
                    .iter()
                    .map(|(name, value)| {
                        Row::new(vec![
                            name.as_str().to_string(),
                            value.to_str().unwrap_or("<binary>").to_string(),
                        ])
                    })
                    .collect();
                Widget::render(
                    Table::new(rows, [Constraint::Fill(1), Constraint::Fill(2)])
                        .header(
                            Row::new(vec!["Header", "Value"])
                                .style(Style::new().bold().underlined()),
                        )
                        .block(Block::bordered().title(" Headers ")),
                    content,
                    buf,
                );
            }

            ResponseArea::Cookies => {
                let rows: Vec<Row> = response
                    .cookies
                    .iter()
                    .map(|cookie| {
                        let domain = cookie.domain().unwrap_or("").to_string();
                        let path = cookie.path().unwrap_or("").to_string();

                        let expiration = match cookie.expires() {
                            Some(cookie::Expiration::DateTime(dt)) => dt
                                .format(&time::format_description::well_known::Rfc3339)
                                .unwrap_or_else(|_| dt.to_string()),
                            Some(cookie::Expiration::Session) => "Session".to_string(),
                            None => String::new(),
                        };
                        Row::new(vec![
                            cookie.name().to_string(),
                            cookie.value().to_string(),
                            domain,
                            path,
                            expiration,
                        ])
                    })
                    .collect();

                Widget::render(
                    Table::new(rows, [Constraint::Fill(1), Constraint::Fill(2)])
                        .header(
                            Row::new(vec!["Name", "Value"]).style(Style::new().bold().underlined()),
                        )
                        .block(Block::bordered().title(" Cookies ")),
                    content,
                    buf,
                );
            }
        }
    }
}
impl Actionable for ResponseDisplay {
    fn key_event(&mut self, key: crossterm::event::KeyEvent) -> Option<AppEvent> {
        match key.code {
            _ => {}
        }
        None
    }
}
