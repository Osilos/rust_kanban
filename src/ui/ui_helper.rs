use crate::app::kanban::{CardPriority, CardStatus};
use crate::calculate_cursor_position;
use crate::constants::{
    APP_TITLE, CARD_ACTIVE_STATUS_STYLE, CARD_COMPLETED_STATUS_STYLE, CARD_DUE_DATE_CRITICAL_STYLE,
    CARD_DUE_DATE_DEFAULT_STYLE, CARD_DUE_DATE_WARNING_STYLE, CARD_PRIORITY_HIGH_STYLE,
    CARD_PRIORITY_LOW_STYLE, CARD_PRIORITY_MEDIUM_STYLE, CARD_STALE_STATUS_STYLE,
    DEFAULT_BOARD_TITLE_LENGTH, DEFAULT_CARD_TITLE_LENGTH, DEFAULT_STYLE, ERROR_TEXT_STYLE,
    FIELD_NOT_SET, FOCUSED_ELEMENT_STYLE, HELP_KEY_STYLE, INACTIVE_TEXT_STYLE,
    LIST_SELECTED_SYMBOL, LIST_SELECT_STYLE, LOG_DEBUG_STYLE, LOG_ERROR_STYLE, LOG_INFO_STYLE,
    LOG_TRACE_STYLE, LOG_WARN_STYLE, MAX_TOASTS_TO_DISPLAY, MIN_TERM_HEIGHT, MIN_TERM_WIDTH,
    MOUSE_HIGHLIGHT_STYLE, PROGRESS_BAR_STYLE, SCREEN_TO_TOAST_WIDTH_RATIO, SPINNER_FRAMES,
    VERTICAL_SCROLL_BAR_SYMBOL,
};
use chrono::{Local, NaiveDateTime};
use log::debug;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, BorderType, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row, Table, Wrap,
};
use tui::Frame;
use tui_logger::TuiLoggerWidget;

use crate::app::state::{AppStatus, Focus, UiMode};
use crate::app::{App, AppConfig, MainMenu, PopupMode};
use crate::io::data_handler::{get_available_local_savefiles, get_config};

use super::widgets::{ToastType, ToastWidget};

/// Draws main screen with kanban boards
pub fn render_zen_mode<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(rect.size());

    render_body(rect, chunks[0], app, false);
}

pub fn render_title_body<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
        .split(rect.size());

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app, false);
}

pub fn render_body_help<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Length(5)].as_ref())
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[1]);

    render_body(rect, chunks[0], app, false);

    let help = draw_help(app, chunks[1]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(help.0, chunks[1]);
    rect.render_stateful_widget(help.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], &mut app.state.help_state);
}

pub fn render_body_log<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Length(8)].as_ref())
        .split(rect.size());

    render_body(rect, chunks[0], app, false);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[1]);
    rect.render_widget(log, chunks[1]);
}

pub fn render_title_body_help<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(75),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app, false);

    let help = draw_help(app, chunks[2]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(help.0, chunks[2]);
    rect.render_stateful_widget(help.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], &mut app.state.help_state);
}

pub fn render_title_body_log<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(75),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app, false);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[2]);
    rect.render_widget(log, chunks[2]);
}

pub fn render_body_help_log<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Length(5),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[1]);

    render_body(rect, chunks[0], app, false);

    let help = draw_help(app, chunks[1]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(help.0, chunks[1]);
    rect.render_stateful_widget(help.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], &mut app.state.help_state);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[2]);
    rect.render_widget(log, chunks[2]);
}

pub fn render_title_body_help_log<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(60),
                Constraint::Length(5),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    render_body(rect, chunks[1], app, false);

    let help = draw_help(app, chunks[2]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(help.0, chunks[2]);
    rect.render_stateful_widget(help.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help.2, help_chunks[2], &mut app.state.help_state);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[3]);
    rect.render_widget(log, chunks[3]);
}

pub fn render_config<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let popup_mode = app.state.popup_mode.is_some();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let reset_btn_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    let config_table = draw_config_table_selector(app, chunks[1]);
    rect.render_stateful_widget(config_table, chunks[1], &mut app.state.config_state);

    let reset_both_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, reset_btn_chunks[0]) {
            app.state.mouse_focus = Some(Focus::SubmitButton);
            app.state.focus = Focus::SubmitButton;
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::SubmitButton) {
                ERROR_TEXT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let reset_config_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, reset_btn_chunks[1]) {
            app.state.mouse_focus = Some(Focus::ExtraFocus);
            app.state.focus = Focus::ExtraFocus;
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::ExtraFocus) {
                ERROR_TEXT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let reset_both_button = Paragraph::new("Reset Config and Keybinds to Default")
        .block(
            Block::default()
                .title("Reset")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(reset_both_style)
        .alignment(Alignment::Center);
    rect.render_widget(reset_both_button, reset_btn_chunks[0]);

    let reset_config_button = Paragraph::new("Reset Only Config to Default")
        .block(
            Block::default()
                .title("Reset")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(reset_config_style)
        .alignment(Alignment::Center);
    rect.render_widget(reset_config_button, reset_btn_chunks[1]);

    let config_help = draw_config_help(&app.state.focus, popup_mode, app);
    rect.render_widget(config_help, chunks[3]);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[4]);
    rect.render_widget(log, chunks[4]);

    if app.config.enable_mouse_support {
        render_close_button(rect, app)
    }
}

/// Draws config list selector
fn draw_config_table_selector(app: &mut App, render_area: Rect) -> Table<'static> {
    let popup_mode = app.state.popup_mode.is_some();
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let focus = app.state.focus.clone();
    let config_list = get_config_items();
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(mouse_coordinates, render_area) {
            app.state.mouse_focus = Some(Focus::ConfigTable);
            app.state.focus = Focus::ConfigTable;
            let top_of_list = render_area.top() + 1;
            let mut bottom_of_list = top_of_list + config_list.len() as u16;
            if bottom_of_list > render_area.bottom() {
                bottom_of_list = render_area.bottom() - 1;
            }
            let mouse_y = mouse_coordinates.1;
            if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
                app.state
                    .config_state
                    .select(Some((mouse_y - top_of_list) as usize));
            }
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if focus == Focus::ConfigTable {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let rows = config_list.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });

    let config_text_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };

    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        let mouse_row = mouse_coordinates.1 as usize;
        let current_selected_row = app.state.config_state.selected().unwrap_or(0);
        let current_selected_row_in_terminal_area =
            current_selected_row + render_area.y as usize + 1; // +1 for border
        if mouse_row == current_selected_row_in_terminal_area {
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if focus == Focus::ConfigTable {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    Table::new(rows)
        .block(
            Block::default()
                .title("Config Editor")
                .borders(Borders::ALL)
                .style(config_text_style)
                .border_style(default_style)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Percentage(40), Constraint::Percentage(60)])
}

/// returns a list of all config items as a vector of strings
fn get_config_items() -> Vec<Vec<String>> {
    let get_config_status = get_config(false);
    let config = if get_config_status.is_err() {
        debug!("Error getting config: {}", get_config_status.unwrap_err());
        AppConfig::default()
    } else {
        get_config_status.unwrap()
    };
    let config_list = config.to_list();
    return config_list;
}

pub fn render_edit_config<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let area = centered_rect(70, 70, rect.size());
    let clear_area = centered_rect(80, 80, rect.size());
    let clear_area_border = Block::default()
        .title("Config Editor")
        .borders(Borders::ALL)
        .border_style(FOCUSED_ELEMENT_STYLE)
        .border_type(BorderType::Rounded);
    rect.render_widget(Clear, clear_area);
    rect.render_widget(clear_area_border, clear_area);

    let chunks = if app.config.enable_mouse_support {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(6),
                    Constraint::Min(6),
                    Constraint::Length(4),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .split(area)
    };

    let edit_box_style =
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1]) {
            app.state.mouse_focus = Some(Focus::EditGeneralConfigPopup);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if app.state.app_status == AppStatus::UserInput {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        };

    let config_item_index = &app.config_item_being_edited.unwrap_or(0);
    let list_items = get_config_items();
    let config_item_name = list_items[*config_item_index].first().unwrap();
    let config_item_value = list_items
        .iter()
        .find(|x| x.first().unwrap() == config_item_name)
        .unwrap()
        .get(1)
        .unwrap();
    let paragraph_text = format!("Current Value is {}\n\n{}",config_item_value,
        "Press 'i' to edit, or 'Esc' to cancel, Press 'Enter' to stop editing and press 'Enter' again to save");
    let paragraph_title = Spans::from(vec![Span::raw(config_item_name)]);
    let config_item = Paragraph::new(paragraph_text)
        .block(
            Block::default()
                .title(paragraph_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(tui::widgets::Wrap { trim: true });
    let edit_item = Paragraph::new(app.state.current_user_input.clone())
        .block(
            Block::default()
                .title("Edit")
                .borders(Borders::ALL)
                .border_style(edit_box_style)
                .border_type(BorderType::Rounded),
        )
        .wrap(tui::widgets::Wrap { trim: true });

    let log = draw_logs(app, true, false, chunks[2]);

    if app.state.app_status == AppStatus::UserInput {
        let current_cursor_position = if app.state.current_cursor_position.is_some() {
            app.state.current_cursor_position.unwrap() as u16
        } else {
            app.state.current_user_input.len() as u16
        };
        let x_offset = current_cursor_position % (chunks[1].width - 2);
        let y_offset = current_cursor_position / (chunks[1].width - 2);
        let x_cursor_position = chunks[1].x + x_offset + 1;
        let y_cursor_position = chunks[1].y + y_offset + 1;
        rect.set_cursor(x_cursor_position, y_cursor_position);
    }
    rect.render_widget(config_item, chunks[0]);
    rect.render_widget(edit_item, chunks[1]);
    rect.render_widget(log, chunks[2]);

    if app.config.enable_mouse_support {
        let submit_button_style =
            if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[3]) {
                app.state.mouse_focus = Some(Focus::SubmitButton);
                MOUSE_HIGHLIGHT_STYLE
            } else {
                if app.state.app_status == AppStatus::KeyBindMode {
                    FOCUSED_ELEMENT_STYLE
                } else {
                    DEFAULT_STYLE
                }
            };
        let submit_button = Paragraph::new("Submit")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(submit_button_style)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center);
        rect.render_widget(submit_button, chunks[3]);
        render_close_button(rect, app)
    }
}

pub fn render_select_default_view<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let render_area = centered_rect(70, 70, rect.size());
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let clear_area = centered_rect(80, 80, rect.size());
    let clear_area_border = Block::default()
        .title("Default HomeScreen Editor")
        .borders(Borders::ALL)
        .border_style(FOCUSED_ELEMENT_STYLE)
        .border_type(BorderType::Rounded);
    rect.render_widget(Clear, clear_area);
    rect.render_widget(clear_area_border, clear_area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(5)].as_ref())
        .split(render_area);

    let list_items = UiMode::all();
    let list_items: Vec<ListItem> = list_items
        .iter()
        .map(|s| ListItem::new(s.to_string()))
        .collect();

    if check_if_mouse_is_in_area(mouse_coordinates, render_area) {
        app.state.mouse_focus = Some(Focus::SelectDefaultView);
        let top_of_list = render_area.top() + 1;
        let mut bottom_of_list = top_of_list + list_items.len() as u16;
        if bottom_of_list > render_area.bottom() {
            bottom_of_list = render_area.bottom();
        }
        let mouse_y = mouse_coordinates.1;
        if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
            app.state
                .default_view_state
                .select(Some((mouse_y - top_of_list) as usize));
        }
    }

    let default_view_list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(DEFAULT_STYLE)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(LIST_SELECT_STYLE)
        .highlight_symbol(LIST_SELECTED_SYMBOL);

    let up_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go up")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let down_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go down")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Use ", DEFAULT_STYLE),
        Span::styled(up_key, HELP_KEY_STYLE),
        Span::styled(" and ", DEFAULT_STYLE),
        Span::styled(down_key, HELP_KEY_STYLE),
        Span::styled("to navigate", DEFAULT_STYLE),
        Span::raw("; "),
        Span::raw("Press "),
        Span::styled("<Enter>", HELP_KEY_STYLE),
        Span::raw(" To select a Default View; Press "),
        Span::styled("<Esc>", HELP_KEY_STYLE),
        Span::raw(" to cancel"),
    ]);

    let help_span = Spans::from(help_text);
    let config_help = Paragraph::new(help_span)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .style(DEFAULT_STYLE)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center)
        .wrap(tui::widgets::Wrap { trim: true });

    rect.render_stateful_widget(
        default_view_list,
        chunks[0],
        &mut app.state.default_view_state,
    );
    rect.render_widget(config_help, chunks[1]);

    if app.config.enable_mouse_support {
        render_close_button(rect, app)
    }
}

pub fn render_edit_keybindings<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let popup_mode = app.state.popup_mode.is_some();
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(5),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(rect.size());
    let table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(95), Constraint::Length(1)].as_ref())
        .split(chunks[1]);
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let progress_bar_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        PROGRESS_BAR_STYLE
    };
    let reset_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[3])
            && app.state.popup_mode.is_none()
        {
            app.state.mouse_focus = Some(Focus::SubmitButton);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::SubmitButton) {
                ERROR_TEXT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        FOCUSED_ELEMENT_STYLE
    };

    let title_bar = draw_title(Some(app), chunks[0]);

    let mut table_items: Vec<Vec<String>> = Vec::new();
    // app.config.keybindings
    let keybindings = app.config.keybindings.clone();
    for (key, value) in keybindings.iter() {
        let mut row: Vec<String> = Vec::new();
        row.push(key.to_string());
        let mut row_value = String::new();
        for v in value.iter() {
            row_value.push_str(&v.to_string());
            row_value.push_str(" ");
        }
        row.push(row_value);
        table_items.push(row);
    }

    let rows = table_items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });

    // draw a progress bar based on the number of items being displayed as not all rows will fit on the screen, calculate the percentage of rows that are visible
    let current_index = app.state.edit_keybindings_state.selected().unwrap_or(0);
    let total_rows = table_items.len();
    let visible_rows = (table_chunks[1].height - 1) as usize;
    let percentage = ((current_index + 1) as f32 / total_rows as f32) * 100.0;
    let blocks_to_render = (percentage / 100.0 * visible_rows as f32) as usize;

    // render blocks VERTICAL_SCROLL_BAR_SYMBOL
    for i in 0..blocks_to_render {
        let block_x = table_chunks[1].right() - 2;
        let block_y = table_chunks[1].top() + i as u16;
        let block = Paragraph::new(VERTICAL_SCROLL_BAR_SYMBOL)
            .style(progress_bar_style)
            .block(Block::default().borders(Borders::NONE));
        rect.render_widget(block, Rect::new(block_x, block_y, 1, 1));
    }

    let table_border_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1])
            && app.state.popup_mode.is_none()
        {
            app.state.mouse_focus = Some(Focus::EditKeybindingsTable);
            let top_of_list = chunks[1].top() + 1;
            let mut bottom_of_list = top_of_list + rows.len() as u16;
            if bottom_of_list > chunks[1].bottom() {
                bottom_of_list = chunks[1].bottom();
            }
            let mouse_y = mouse_coordinates.1;
            if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
                app.state
                    .edit_keybindings_state
                    .select(Some((mouse_y - top_of_list) as usize));
            }
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::EditKeybindingsTable) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let t = Table::new(rows)
        .block(
            Block::default()
                .title("Edit Keybindings")
                .style(default_style)
                .border_style(table_border_style)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    let next_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let up_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go up")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let down_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go down")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let edit_keybind_help_spans = Spans::from(vec![
        Span::styled("Use ", default_style),
        Span::styled(up_key, current_element_style),
        Span::styled(" and ", default_style),
        Span::styled(down_key, current_element_style),
        Span::styled(" to select a keybinding, ", default_style),
        Span::styled("<Enter>", current_element_style),
        Span::styled(" to edit, ", default_style),
        Span::styled("<Esc>", current_element_style),
        Span::styled(
            " to cancel, To Reset Keybindings to Default, Press ",
            default_style,
        ),
        Span::styled(
            [next_focus_key, prev_focus_key].join(" or "),
            current_element_style,
        ),
        Span::styled("to highlight Reset Button and Press ", default_style),
        Span::styled("<Enter>", current_element_style),
        Span::styled(" on the Reset Keybindings Button", default_style),
    ]);

    let edit_keybind_help = Paragraph::new(edit_keybind_help_spans)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(default_style)
        .alignment(Alignment::Center)
        .wrap(tui::widgets::Wrap { trim: true });

    let reset_button = Paragraph::new("Reset Keybindings to Default")
        .block(
            Block::default()
                .title("Reset")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(reset_style)
        .alignment(Alignment::Center);

    rect.render_widget(title_bar, chunks[0]);
    rect.render_stateful_widget(t, chunks[1], &mut app.state.edit_keybindings_state);
    rect.render_widget(edit_keybind_help, chunks[2]);
    rect.render_widget(reset_button, chunks[3]);

    if app.config.enable_mouse_support {
        render_close_button(rect, app)
    }
}

pub fn render_edit_specific_keybinding<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let area = centered_rect(70, 70, rect.size());
    let clear_area = centered_rect(80, 80, rect.size());
    let clear_area_border = Block::default()
        .title("Edit Keybindings")
        .borders(Borders::ALL)
        .border_style(FOCUSED_ELEMENT_STYLE)
        .border_type(BorderType::Rounded);

    rect.render_widget(Clear, clear_area);
    rect.render_widget(clear_area_border, clear_area);
    let chunks = if app.config.enable_mouse_support {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(6),
                    Constraint::Min(6),
                    Constraint::Length(4),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .split(area)
    };

    let edit_box_style =
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1]) {
            app.state.mouse_focus = Some(Focus::EditSpecificKeyBindingPopup);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if app.state.app_status == AppStatus::KeyBindMode {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        };

    let key_id = app.state.edit_keybindings_state.selected().unwrap_or(0);
    let current_bindings = app.config.keybindings.clone();
    let mut key_list = vec![];

    for (k, v) in current_bindings.iter() {
        key_list.push((k, v));
    }

    if key_id > key_list.len() {
        return;
    } else {
        let key = key_list[key_id].0;
        let value = key_list[key_id].1;
        let mut key_value = String::new();
        for v in value.iter() {
            key_value.push_str(&v.to_string());
            key_value.push_str(" ");
        }
        let paragraph_text = format!("Current Value is {}\n\n{}",key_value,
            "Press 'i' to edit, or 'Esc' to cancel, Press 'Enter' to stop editing and press 'Enter' again to save");
        let paragraph_title = key.to_uppercase();
        let config_item = Paragraph::new(paragraph_text)
            .block(
                Block::default()
                    .title(paragraph_title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(tui::widgets::Wrap { trim: true });
        let current_edited_keybinding = app.state.edited_keybinding.clone();
        let mut current_edited_keybinding_string = String::new();
        if current_edited_keybinding.is_some() {
            for key in current_edited_keybinding.unwrap() {
                current_edited_keybinding_string.push_str(&key.to_string());
                current_edited_keybinding_string.push_str(" ");
            }
        }
        let edit_item = Paragraph::new(current_edited_keybinding_string.clone())
            .block(
                Block::default()
                    .title("Edit")
                    .borders(Borders::ALL)
                    .border_style(edit_box_style)
                    .border_type(BorderType::Rounded),
            )
            .wrap(tui::widgets::Wrap { trim: true });

        let log = draw_logs(app, true, false, chunks[2]);

        if app.state.app_status == AppStatus::KeyBindMode {
            let current_cursor_position = if app.state.current_cursor_position.is_some() {
                app.state.current_cursor_position.unwrap() as u16
            } else {
                current_edited_keybinding_string.len() as u16
            };
            let x_offset = current_cursor_position % (chunks[1].width - 2);
            let y_offset = current_cursor_position / (chunks[1].width - 2);
            let x_cursor_position = chunks[1].x + x_offset + 1;
            let y_cursor_position = chunks[1].y + y_offset + 1;
            rect.set_cursor(x_cursor_position, y_cursor_position);
        }
        rect.render_widget(config_item, chunks[0]);
        rect.render_widget(edit_item, chunks[1]);
        rect.render_widget(log, chunks[2]);
    }

    if app.config.enable_mouse_support {
        let submit_button_style =
            if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[3]) {
                app.state.mouse_focus = Some(Focus::SubmitButton);
                MOUSE_HIGHLIGHT_STYLE
            } else {
                if app.state.app_status == AppStatus::KeyBindMode {
                    FOCUSED_ELEMENT_STYLE
                } else {
                    DEFAULT_STYLE
                }
            };
        let submit_button = Paragraph::new("Submit")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(submit_button_style)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Center);
        rect.render_widget(submit_button, chunks[3]);
        render_close_button(rect, app);
    }
}

pub fn render_main_menu<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(16),
                Constraint::Min(8),
                Constraint::Length(8),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[2]);

    let title = draw_title(Some(app), chunks[0]);
    rect.render_widget(title, chunks[0]);

    let main_menu = draw_main_menu(app, chunks[1]);
    rect.render_stateful_widget(main_menu, chunks[1], &mut app.state.main_menu_state);

    let main_menu_help = draw_help(app, chunks[2]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(main_menu_help.0, chunks[2]);
    rect.render_stateful_widget(main_menu_help.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(main_menu_help.2, help_chunks[2], &mut app.state.help_state);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[3]);
    rect.render_widget(log, chunks[3]);
}

pub fn render_help_menu<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Length(4)].as_ref())
        .split(rect.size());

    let help_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(chunks[0]);

    let help_menu = draw_help(app, chunks[0]);
    let help_separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(default_style);
    rect.render_widget(help_menu.0, chunks[0]);
    rect.render_stateful_widget(help_menu.1, help_chunks[0], &mut app.state.help_state);
    rect.render_widget(help_separator, help_chunks[1]);
    rect.render_stateful_widget(help_menu.2, help_chunks[2], &mut app.state.help_state);

    let log = draw_logs(app, true, app.state.popup_mode.is_some(), chunks[1]);
    rect.render_widget(log, chunks[1]);
    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_logs_only<'a, B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(rect.size());
    let log = draw_logs(app, false, app.state.popup_mode.is_some(), chunks[0]);
    rect.render_widget(log, chunks[0]);
    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

/// Draws Help section for normal mode
fn draw_help<'a>(app: &mut App, render_area: Rect) -> (Block<'a>, Table<'a>, Table<'a>) {
    let keybind_store = &app.state.keybind_store;
    let popup_mode = app.state.popup_mode.is_some();
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let focus = &mut app.state.focus;
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(mouse_coordinates, render_area) {
            app.state.mouse_focus = Some(Focus::Help);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if *focus == Focus::Help || *focus == Focus::MainMenuHelp {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let current_element_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        FOCUSED_ELEMENT_STYLE
    };

    let rows = keybind_store.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16)
    });

    // split the rows into two tables
    let left_rows = rows.clone().take(rows.clone().count() / 2);
    let right_rows = rows.clone().skip(rows.clone().count() / 2);

    let left_table = Table::new(left_rows)
        .block(Block::default())
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
        .style(default_style);

    let right_table = Table::new(right_rows)
        .block(Block::default())
        .highlight_style(current_element_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
        .style(default_style);

    let border_block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(default_style)
        .border_type(BorderType::Rounded);

    (border_block, left_table, right_table)
}

/// Draws help section for config mode
fn draw_config_help<'a>(focus: &'a Focus, popup_mode: bool, app: &'a App) -> Paragraph<'a> {
    let helpbox_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if matches!(focus, Focus::ConfigHelp) {
            FOCUSED_ELEMENT_STYLE
        } else {
            DEFAULT_STYLE
        }
    };
    let text_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let helpkey_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_KEY_STYLE
    };

    let up_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go up")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let down_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go down")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let next_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Use ", text_style),
        Span::styled(up_key, helpkey_style),
        Span::styled(" and ", text_style),
        Span::styled(down_key, helpkey_style),
        Span::styled("to navigate", text_style),
        Span::styled("; ", text_style),
        Span::styled("To edit a value, press ", text_style),
        Span::styled("<Enter>", helpkey_style),
        Span::styled("; Press ", text_style),
        Span::styled("<Esc>", helpkey_style),
        Span::styled(
            " to cancel, To Reset Keybindings to Default, Press ",
            text_style,
        ),
        Span::styled([next_focus_key, prev_focus_key].join(" or "), helpkey_style),
        Span::styled("to highlight Reset Button and Press ", text_style),
        Span::styled("<Enter>", helpkey_style),
        Span::styled(" on the Reset Keybindings Button", text_style),
    ]);

    let help_span = Spans::from(help_text);

    Paragraph::new(help_span)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .style(helpbox_style)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center)
        .wrap(tui::widgets::Wrap { trim: true })
}

/// Draws logs
fn draw_logs<'a>(
    app: &mut App,
    enable_focus_highlight: bool,
    popup_mode: bool,
    render_area: Rect,
) -> TuiLoggerWidget<'a> {
    let focus = app.state.focus;
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let logbox_style = if check_if_mouse_is_in_area(mouse_coordinates, render_area)
        && app.state.popup_mode.is_none()
    {
        app.state.mouse_focus = Some(Focus::Log);
        MOUSE_HIGHLIGHT_STYLE
    } else {
        if matches!(focus, Focus::Log) && enable_focus_highlight {
            FOCUSED_ELEMENT_STYLE
        } else {
            DEFAULT_STYLE
        }
    };
    if popup_mode {
        TuiLoggerWidget::default()
            .style_error(INACTIVE_TEXT_STYLE)
            .style_debug(INACTIVE_TEXT_STYLE)
            .style_warn(INACTIVE_TEXT_STYLE)
            .style_trace(INACTIVE_TEXT_STYLE)
            .style_info(INACTIVE_TEXT_STYLE)
            .output_file(false)
            .output_line(false)
            .output_target(false)
            .block(
                Block::default()
                    .title("Logs")
                    .border_style(INACTIVE_TEXT_STYLE)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
    } else {
        TuiLoggerWidget::default()
            .style_error(LOG_ERROR_STYLE)
            .style_debug(LOG_DEBUG_STYLE)
            .style_warn(LOG_WARN_STYLE)
            .style_trace(LOG_TRACE_STYLE)
            .style_info(LOG_INFO_STYLE)
            .output_file(false)
            .output_line(false)
            .output_target(false)
            .block(
                Block::default()
                    .title("Logs")
                    .border_style(logbox_style)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
    }
}

/// Draws Main menu
fn draw_main_menu<'a>(app: &mut App, render_area: Rect) -> List<'a> {
    let main_menu_items = MainMenu::all();
    let popup_mode = app.state.popup_mode.is_some();
    let focus = app.state.focus;
    let mouse_coordinates = app.state.current_mouse_coordinates;
    let menu_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(mouse_coordinates, render_area) {
            if !(app.state.popup_mode.is_some()
                && app.state.popup_mode.unwrap() == PopupMode::CommandPalette)
            {
                app.state.mouse_focus = Some(Focus::MainMenu);
                // calculate the mouse_list_index based on the mouse coordinates and the length of the list
                let top_of_list = render_area.top() + 1;
                let mut bottom_of_list = top_of_list + main_menu_items.len() as u16;
                if bottom_of_list > render_area.bottom() {
                    bottom_of_list = render_area.bottom();
                }
                let mouse_y = mouse_coordinates.1;
                if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
                    app.state
                        .main_menu_state
                        .select(Some((mouse_y - top_of_list) as usize));
                }
            }
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(focus, Focus::MainMenu) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let default_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let highlight_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        LIST_SELECT_STYLE
    };
    let list_items = main_menu_items
        .iter()
        .map(|i| ListItem::new(i.to_string()))
        .collect::<Vec<ListItem>>();
    List::new(list_items)
        .block(
            Block::default()
                .title("Main menu")
                .style(default_style)
                .borders(Borders::ALL)
                .border_style(menu_style)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(highlight_style)
        .highlight_symbol(LIST_SELECTED_SYMBOL)
}

/// Draws Kanban boards
pub fn render_body<'a, B>(rect: &mut Frame<B>, area: Rect, app: &mut App, preview_mode: bool)
where
    B: Backend,
{
    let fallback_boards = vec![];
    let focus = &app.state.focus;
    let boards = if preview_mode {
        if app.state.preview_boards_and_cards.is_some() {
            &app.state.preview_boards_and_cards.as_ref().unwrap()
        } else {
            &fallback_boards
        }
    } else {
        &app.boards
    };
    let progress_bar_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        PROGRESS_BAR_STYLE
    };
    let error_text_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        ERROR_TEXT_STYLE
    };
    let current_board = &app.state.current_board_id.unwrap_or(0);

    let add_board_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Create new board")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    // check if any boards are present
    if preview_mode {
        if app.state.preview_boards_and_cards.is_none()
            || app
                .state
                .preview_boards_and_cards
                .as_ref()
                .unwrap()
                .is_empty()
        {
            let empty_paragraph = Paragraph::new("No boards found".to_string())
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title("Boards")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(error_text_style);
            rect.render_widget(empty_paragraph, area);
            return;
        }
    } else {
        if app.visible_boards_and_cards.is_empty() {
            let empty_paragraph = Paragraph::new(
                [
                    "No boards found, press ".to_string(),
                    add_board_key,
                    " to add a new board".to_string(),
                ]
                .concat(),
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Boards")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(error_text_style);
            rect.render_widget(empty_paragraph, area);
            return;
        }
    }

    // make a list of constraints depending on NO_OF_BOARDS_PER_PAGE constant
    let chunks = if app.config.disable_scrollbars {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(99), Constraint::Length(1)].as_ref())
            .split(area)
    };
    let mut constraints = vec![];
    // check if length of boards is more than NO_OF_BOARDS_PER_PAGE
    if boards.len() > app.config.no_of_boards_to_show.into() {
        for _i in 0..app.config.no_of_boards_to_show {
            constraints.push(Constraint::Percentage(
                100 / app.config.no_of_boards_to_show as u16,
            ));
        }
    } else {
        for _i in 0..boards.len() {
            constraints.push(Constraint::Percentage(100 / boards.len() as u16));
        }
    }
    let board_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(chunks[0]);
    // visible_boards_and_cards: Vec<LinkedHashMap<String, Vec<String>>>
    let visible_boards_and_cards = if preview_mode {
        app.state.preview_visible_boards_and_cards.clone()
    } else {
        app.visible_boards_and_cards.clone()
    };
    for (board_index, board_and_card_tuple) in visible_boards_and_cards.iter().enumerate() {
        // render board with title in board chunks alongside with cards in card chunks of the board
        // break if board_index is more than NO_OF_BOARDS_PER_PAGE
        if board_index >= app.config.no_of_boards_to_show.into() {
            break;
        }
        let board_id = board_and_card_tuple.0;
        // find index of board with board_id in boards
        let board = if preview_mode {
            app.state
                .preview_boards_and_cards
                .as_ref()
                .unwrap()
                .iter()
                .find(|&b| b.id == *board_id)
        } else {
            app.boards.iter().find(|&b| b.id == *board_id)
        };
        // check if board is found if not continue
        if board.is_none() {
            continue;
        }
        let board = board.unwrap();
        let board_title = board.name.clone();
        let board_cards = board_and_card_tuple.1;
        // if board title is longer than DEFAULT_BOARD_TITLE_LENGTH, truncate it and add ... at the end
        let board_title = if board_title.len() > DEFAULT_BOARD_TITLE_LENGTH.into() {
            format!(
                "{}...",
                &board_title[0..DEFAULT_BOARD_TITLE_LENGTH as usize]
            )
        } else {
            board_title
        };
        let board_title = format!("{} ({})", board_title, board.cards.len());
        let board_title = if *board_id as u128 == *current_board {
            format!("{} {}", ">>", board_title)
        } else {
            board_title
        };

        // check if length of cards is more than NO_OF_CARDS_PER_BOARD constant
        let mut card_constraints = vec![];
        if board_cards.len() > app.config.no_of_cards_to_show.into() {
            for _i in 0..app.config.no_of_cards_to_show {
                card_constraints.push(Constraint::Percentage(
                    90 / app.config.no_of_cards_to_show as u16,
                ));
            }
        } else {
            for _i in 0..board_cards.len() {
                card_constraints.push(Constraint::Percentage(100 / board_cards.len() as u16));
            }
        }

        // check if board_index is >= board_chunks.len() if yes continue
        if board_index >= board_chunks.len() {
            continue;
        }

        let board_style = if app.state.popup_mode.is_some() {
            INACTIVE_TEXT_STYLE
        } else {
            DEFAULT_STYLE
        };
        let board_border_style = if app.state.popup_mode.is_some() {
            INACTIVE_TEXT_STYLE
        } else {
            if check_if_mouse_is_in_area(
                app.state.current_mouse_coordinates,
                board_chunks[board_index],
            ) {
                app.state.mouse_focus = Some(Focus::Body);
                app.state.current_board_id = Some(*board_id);
                MOUSE_HIGHLIGHT_STYLE
            } else {
                if *board_id == *current_board
                    && matches!(focus, Focus::Body)
                    && app.state.current_card_id == None
                {
                    FOCUSED_ELEMENT_STYLE
                } else {
                    DEFAULT_STYLE
                }
            }
        };

        let board_block = Block::default()
            .title(&*board_title)
            .borders(Borders::ALL)
            .style(board_style)
            .border_style(board_border_style)
            .border_type(BorderType::Rounded);
        rect.render_widget(board_block, board_chunks[board_index]);

        let card_area_chunks = if app.config.disable_scrollbars {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(board_chunks[board_index])
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(1), Constraint::Percentage(99)].as_ref())
                .split(board_chunks[board_index])
        };

        let card_chunks = if app.config.disable_scrollbars {
            Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(card_constraints.as_ref())
                .split(card_area_chunks[0])
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(card_constraints.as_ref())
                .split(card_area_chunks[1])
        };

        if !app.config.disable_scrollbars {
            // calculate the current card scroll percentage
            // get the index of current card in board_cards
            let all_board_cards = boards
                .iter()
                .find(|&b| b.id == *board_id)
                .unwrap()
                .cards
                .clone();
            let current_card_index = all_board_cards
                .iter()
                .position(|c| c.id == app.state.current_card_id.unwrap_or(0));
            let cards_scroll_percentage =
                (current_card_index.unwrap_or(0) + 1) as f64 / all_board_cards.len() as f64;
            let cards_scroll_percentage = cards_scroll_percentage.clamp(0.0, 1.0);
            let available_height = if card_area_chunks[0].height >= 2 {
                (card_area_chunks[0].height - 2) as f64
            } else {
                0.0
            };
            // calculate number of blocks to render
            let blocks_to_render = (available_height * cards_scroll_percentage) as u16;
            // render blocks VERTICAL_SCROLL_BAR_SYMBOL
            if all_board_cards.len() > 0 {
                for i in 0..blocks_to_render {
                    let block = Paragraph::new(VERTICAL_SCROLL_BAR_SYMBOL)
                        .style(progress_bar_style)
                        .block(Block::default().borders(Borders::NONE));
                    rect.render_widget(
                        block,
                        Rect::new(
                            card_area_chunks[0].x,
                            card_area_chunks[0].y + i + 1,
                            card_area_chunks[0].width,
                            1,
                        ),
                    );
                }
            }
        };
        for (card_index, card_id) in board_cards.iter().enumerate() {
            if card_index >= app.config.no_of_cards_to_show.into() {
                break;
            }
            let inner_card_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .margin(1)
                .split(card_chunks[card_index]);
            // unwrap card if panic skip it and log it
            let mut card = board.get_card(*card_id);
            // check if card is None, if so skip it and log it
            if card.is_none() {
                continue;
            } else {
                card = Some(card.unwrap());
            }
            let card_title = card.unwrap().name.clone();
            let card_title = if card_title.len() > DEFAULT_CARD_TITLE_LENGTH.into() {
                format!("{}...", &card_title[0..DEFAULT_CARD_TITLE_LENGTH as usize])
            } else {
                card_title
            };

            let card_title = if app.state.current_card_id.unwrap_or(0) == *card_id {
                format!("{} {}", ">>", card_title)
            } else {
                card_title
            };

            let card_description = if card.unwrap().description == FIELD_NOT_SET {
                "Description: Not Set".to_string()
            } else {
                card.unwrap().description.clone()
            };
            let mut card_extra_info = vec![Spans::from("")];
            let card_due_date = card.unwrap().date_due.clone();
            if !card_due_date.is_empty() {
                let parsed_due_date =
                    NaiveDateTime::parse_from_str(&card_due_date, "%d/%m/%Y-%H:%M:%S");
                // card due date is in the format dd/mm/yyyy check if the due date is within WARNING_DUE_DATE_DAYS if so highlight it
                let card_due_date_styled = if parsed_due_date.is_ok() {
                    let parsed_due_date = parsed_due_date.unwrap();
                    let today = Local::now().naive_local();
                    let days_left = parsed_due_date.signed_duration_since(today).num_days();
                    if days_left <= app.config.warning_delta.into() && days_left >= 0 {
                        if app.state.popup_mode.is_some() {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                INACTIVE_TEXT_STYLE,
                            ))
                        } else {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                CARD_DUE_DATE_WARNING_STYLE,
                            ))
                        }
                    } else if days_left < 0 {
                        if app.state.popup_mode.is_some() {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                INACTIVE_TEXT_STYLE,
                            ))
                        } else {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                CARD_DUE_DATE_CRITICAL_STYLE,
                            ))
                        }
                    } else {
                        if app.state.popup_mode.is_some() {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                INACTIVE_TEXT_STYLE,
                            ))
                        } else {
                            Spans::from(Span::styled(
                                format!("Due: {}", card_due_date),
                                CARD_DUE_DATE_DEFAULT_STYLE,
                            ))
                        }
                    }
                } else {
                    if app.state.popup_mode.is_some() {
                        Spans::from(Span::styled(
                            format!("Due: {}", card_due_date),
                            INACTIVE_TEXT_STYLE,
                        ))
                    } else {
                        Spans::from(Span::styled(
                            format!("Due: {}", card_due_date),
                            CARD_DUE_DATE_DEFAULT_STYLE,
                        ))
                    }
                };
                card_extra_info.extend(vec![card_due_date_styled]);
            }
            let card_status = format!("Status: {}", card.unwrap().card_status.clone().to_string());
            let card_status = if card_status == "Status: Active" {
                if app.state.popup_mode.is_some() {
                    Spans::from(Span::styled(card_status, INACTIVE_TEXT_STYLE))
                } else {
                    Spans::from(Span::styled(card_status, CARD_ACTIVE_STATUS_STYLE))
                }
            } else if card_status == "Status: Complete" {
                if app.state.popup_mode.is_some() {
                    Spans::from(Span::styled(card_status, INACTIVE_TEXT_STYLE))
                } else {
                    Spans::from(Span::styled(card_status, CARD_COMPLETED_STATUS_STYLE))
                }
            } else {
                if app.state.popup_mode.is_some() {
                    Spans::from(Span::styled(card_status, INACTIVE_TEXT_STYLE))
                } else {
                    Spans::from(Span::styled(card_status, CARD_STALE_STATUS_STYLE))
                }
            };
            card_extra_info.extend(vec![card_status]);

            // if card id is same as current_card, highlight it
            let card_style = if app.state.popup_mode.is_some() {
                INACTIVE_TEXT_STYLE
            } else {
                if check_if_mouse_is_in_area(
                    app.state.current_mouse_coordinates,
                    card_chunks[card_index],
                ) {
                    app.state.mouse_focus = Some(Focus::Body);
                    app.state.current_card_id = Some(*card_id);
                    MOUSE_HIGHLIGHT_STYLE
                } else {
                    if app.state.current_card_id.unwrap_or(0) == *card_id
                        && matches!(focus, Focus::Body)
                        && *board_id == *current_board
                    {
                        FOCUSED_ELEMENT_STYLE
                    } else {
                        DEFAULT_STYLE
                    }
                }
            };
            let card_block = Block::default()
                .title(&*card_title)
                .borders(Borders::ALL)
                .border_style(card_style)
                .border_type(BorderType::Rounded);
            rect.render_widget(card_block, card_chunks[card_index]);
            let card_paragraph = Paragraph::new(card_description)
                .alignment(Alignment::Left)
                .block(Block::default())
                .wrap(tui::widgets::Wrap { trim: false });
            rect.render_widget(card_paragraph, inner_card_chunks[0]);
            let card_extra_info = Paragraph::new(card_extra_info)
                .alignment(Alignment::Left)
                .block(Block::default())
                .wrap(tui::widgets::Wrap { trim: false });
            rect.render_widget(card_extra_info, inner_card_chunks[1]);
        }
    }

    if !app.config.disable_scrollbars {
        // draw line_gauge in chunks[1]
        // get the index of the current board in boards and set percentage
        let current_board_id = app.state.current_board_id.unwrap_or(0);
        // get the index of the board with the id
        let current_board_index = boards
            .iter()
            .position(|board| board.id == current_board_id)
            .unwrap_or(0)
            + 1;
        let percentage = {
            // make sure percentage is not nan and is between 0 and 100
            let temp_percent = (current_board_index as f64 / boards.len() as f64) * 100.0;
            if temp_percent.is_nan() {
                0
            } else if temp_percent > 100.0 {
                100
            } else {
                temp_percent as u16
            }
        };
        let line_gauge = Gauge::default()
            .block(Block::default())
            .gauge_style(progress_bar_style)
            .percent(percentage as u16);
        rect.render_widget(line_gauge, chunks[1]);
    }
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

fn top_left_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[0])[0]
}

/// Draws size error screen if the terminal is too small
pub fn draw_size_error<B>(rect: &mut Frame<B>, size: &Rect, msg: String)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(*size);

    let title = draw_title(None, *size);
    rect.render_widget(title, chunks[0]);

    let mut text = vec![Spans::from(Span::styled(msg, ERROR_TEXT_STYLE))];
    text.append(&mut vec![Spans::from(Span::raw(
        "Resize the window to continue, or press 'q' to quit.",
    ))]);
    let body = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    rect.render_widget(body, chunks[1]);
}

pub fn draw_loading_screen<B>(rect: &mut Frame<B>, size: &Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(*size);

    let title = draw_title(None, *size);
    rect.render_widget(title, chunks[0]);

    let text = Spans::from(vec![
        Span::styled("Loading......", FOCUSED_ELEMENT_STYLE),
        Span::styled("`(*>﹏<*)′", FOCUSED_ELEMENT_STYLE),
        Span::styled("Please wait", FOCUSED_ELEMENT_STYLE),
    ]);
    let body = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    rect.render_widget(body, chunks[1]);
}

/// Draws the title bar
pub fn draw_title<'a>(app: Option<&mut App>, render_area: Rect) -> Paragraph<'a> {
    let mut popup_mode = false;
    let mut mouse_coordinates = (0, 0);
    let mut focus = Focus::NoFocus;
    if app.is_some() {
        popup_mode = app.as_ref().unwrap().state.popup_mode.is_some();
        mouse_coordinates = app.as_ref().unwrap().state.current_mouse_coordinates;
        focus = app.as_ref().unwrap().state.focus;
    }
    // check if focus is on title
    let title_style = if popup_mode {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(mouse_coordinates, render_area) {
            if app.is_some() {
                let mut app = app.unwrap();
                app.state.mouse_focus = Some(Focus::Title);
            }
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(focus, Focus::Title) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    Paragraph::new(APP_TITLE)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(title_style)
                .border_type(BorderType::Rounded),
        )
}

/// Helper function to check terminal size
pub fn check_size(rect: &Rect) -> String {
    let mut msg = String::new();
    if rect.width < MIN_TERM_WIDTH {
        msg.push_str(&format!(
            "For optimal viewing experience, Terminal width should be >= {}, (current width {})",
            MIN_TERM_WIDTH, rect.width
        ));
    } else if rect.height < MIN_TERM_HEIGHT {
        msg.push_str(&format!(
            "For optimal viewing experience, Terminal height should be >= {}, (current height {})",
            MIN_TERM_HEIGHT, rect.height
        ));
    } else {
        msg.push_str("Size OK");
    }
    msg
}

pub fn render_new_board_form<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    // make a form for the Board struct
    // take name and description where description is optional
    // submit button

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(8),
                Constraint::Length(4),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let name_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1]) {
            app.state.mouse_focus = Some(Focus::NewBoardName);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::NewBoardName) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let description_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[2]) {
            app.state.mouse_focus = Some(Focus::NewBoardDescription);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::NewBoardDescription) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let help_key_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_KEY_STYLE
    };
    let submit_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[4]) {
            app.state.mouse_focus = Some(Focus::SubmitButton);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::SubmitButton) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let title_paragraph = Paragraph::new("Create a new Board")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(default_style),
        );
    rect.render_widget(title_paragraph, chunks[0]);

    let wrapped_title_text =
        textwrap::wrap(&app.state.new_board_form[0], (chunks[1].width - 2) as usize);
    let board_name_field = wrapped_title_text
        .iter()
        .map(|x| Spans::from(Span::raw(&**x)))
        .collect::<Vec<Spans>>();
    let wrapped_description_text =
        textwrap::wrap(&app.state.new_board_form[1], (chunks[2].width - 2) as usize);
    let board_description_field = wrapped_description_text
        .iter()
        .map(|x| Spans::from(Span::raw(&**x)))
        .collect::<Vec<Spans>>();

    let board_name = Paragraph::new(board_name_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(name_style)
                .border_type(BorderType::Rounded)
                .title("Board Name (required)"),
        );
    rect.render_widget(board_name, chunks[1]);

    let board_description = Paragraph::new(board_description_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(description_style)
                .border_type(BorderType::Rounded)
                .title("Board Description"),
        );
    rect.render_widget(board_description, chunks[2]);

    let input_mode_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Enter input mode")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let next_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Press ", default_style),
        Span::styled(input_mode_key, help_key_style),
        Span::styled("to start typing", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Esc>", help_key_style),
        Span::styled(" to stop typing", default_style),
        Span::styled("; ", default_style),
        Span::styled("Press ", default_style),
        Span::styled(
            [next_focus_key, prev_focus_key].join(" or "),
            help_key_style,
        ),
        Span::styled("to switch focus", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Enter>", help_key_style),
        Span::styled(" to submit", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Esc>", help_key_style),
        Span::styled(" to cancel", default_style),
    ]);
    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(default_style),
        )
        .wrap(tui::widgets::Wrap { trim: true });
    rect.render_widget(help_paragraph, chunks[3]);

    let submit_button = Paragraph::new("Submit").alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .style(submit_style)
            .border_type(BorderType::Rounded),
    );
    rect.render_widget(submit_button, chunks[4]);

    if app.state.focus == Focus::NewBoardName && app.state.app_status == AppStatus::UserInput {
        if app.state.current_cursor_position.is_some() {
            let (x_pos, y_pos) = calculate_cursor_position(
                wrapped_title_text,
                app.state
                    .current_cursor_position
                    .unwrap_or_else(|| app.state.new_board_form[0].len()),
                chunks[1],
            );
            rect.set_cursor(x_pos, y_pos);
        } else {
            rect.set_cursor(chunks[1].x + 1, chunks[1].y + 1);
        }
    } else if app.state.focus == Focus::NewBoardDescription
        && app.state.app_status == AppStatus::UserInput
    {
        if app.state.current_cursor_position.is_some() {
            let (x_pos, y_pos) = calculate_cursor_position(
                wrapped_description_text,
                app.state
                    .current_cursor_position
                    .unwrap_or_else(|| app.state.new_board_form[1].len()),
                chunks[2],
            );
            rect.set_cursor(x_pos, y_pos);
        } else {
            rect.set_cursor(chunks[2].x + 1, chunks[2].y + 1);
        }
    }

    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_new_card_form<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(8),
                Constraint::Length(3),
                Constraint::Length(4),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let name_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1]) {
            app.state.mouse_focus = Some(Focus::NewCardName);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::NewCardName) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let description_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[2]) {
            app.state.mouse_focus = Some(Focus::NewCardDescription);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::NewCardDescription) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let due_date_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[3]) {
            app.state.mouse_focus = Some(Focus::NewCardDueDate);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::NewCardDueDate) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };
    let help_key_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_KEY_STYLE
    };
    let submit_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[5]) {
            app.state.mouse_focus = Some(Focus::SubmitButton);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            if matches!(app.state.focus, Focus::SubmitButton) {
                FOCUSED_ELEMENT_STYLE
            } else {
                DEFAULT_STYLE
            }
        }
    };

    let title_paragraph = Paragraph::new("Create a new Card")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(default_style),
        );
    rect.render_widget(title_paragraph, chunks[0]);

    let wrapped_card_name_text =
        textwrap::wrap(&app.state.new_card_form[0], (chunks[1].width - 2) as usize);
    let card_name_field = wrapped_card_name_text
        .iter()
        .map(|x| Spans::from(Span::raw(&**x)))
        .collect::<Vec<Spans>>();
    let wrapped_card_description_text =
        textwrap::wrap(&app.state.new_card_form[1], (chunks[2].width - 2) as usize);
    let card_description_field = wrapped_card_description_text
        .iter()
        .map(|x| Spans::from(Span::raw(&**x)))
        .collect::<Vec<Spans>>();
    let wrapped_card_due_date_text =
        textwrap::wrap(&app.state.new_card_form[2], (chunks[3].width - 2) as usize);
    let card_due_date_field = wrapped_card_due_date_text
        .iter()
        .map(|x| Spans::from(Span::raw(&**x)))
        .collect::<Vec<Spans>>();
    let card_name = Paragraph::new(card_name_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(name_style)
                .border_type(BorderType::Rounded)
                .title("Card Name (required)"),
        );
    rect.render_widget(card_name, chunks[1]);

    let card_description = Paragraph::new(card_description_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(description_style)
                .border_type(BorderType::Rounded)
                .title("Card Description"),
        );
    rect.render_widget(card_description, chunks[2]);

    let card_due_date = Paragraph::new(card_due_date_field)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(due_date_style)
                .border_type(BorderType::Rounded)
                .title("Card Due Date (DD/MM/YYYY-HH:MM:SS) or (DD/MM/YYYY)"),
        );
    rect.render_widget(card_due_date, chunks[3]);

    let input_mode_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Enter input mode")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let next_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus next")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let prev_focus_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Focus previous")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Press ", default_style),
        Span::styled(input_mode_key, help_key_style),
        Span::styled("to start typing", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Esc>", help_key_style),
        Span::styled(" to stop typing", default_style),
        Span::styled("; ", default_style),
        Span::styled("Press ", default_style),
        Span::styled(
            [next_focus_key, prev_focus_key].join(" or "),
            help_key_style,
        ),
        Span::styled("to switch focus", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Enter>", help_key_style),
        Span::styled(" to submit", default_style),
        Span::styled("; ", default_style),
        Span::styled("<Esc>", help_key_style),
        Span::styled(" to cancel", default_style),
    ]);

    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(default_style),
        )
        .wrap(tui::widgets::Wrap { trim: true });
    rect.render_widget(help_paragraph, chunks[4]);

    let submit_button = Paragraph::new("Submit").alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .style(submit_style)
            .border_type(BorderType::Rounded),
    );
    rect.render_widget(submit_button, chunks[5]);

    if app.state.focus == Focus::NewCardName && app.state.app_status == AppStatus::UserInput {
        if app.state.current_cursor_position.is_some() {
            let (x_pos, y_pos) = calculate_cursor_position(
                wrapped_card_name_text,
                app.state
                    .current_cursor_position
                    .unwrap_or_else(|| app.state.new_card_form[0].len()),
                chunks[1],
            );
            rect.set_cursor(x_pos, y_pos);
        } else {
            rect.set_cursor(chunks[1].x + 1, chunks[1].y + 1);
        }
    } else if app.state.focus == Focus::NewCardDescription
        && app.state.app_status == AppStatus::UserInput
    {
        if app.state.current_cursor_position.is_some() {
            let (x_pos, y_pos) = calculate_cursor_position(
                wrapped_card_description_text,
                app.state
                    .current_cursor_position
                    .unwrap_or_else(|| app.state.new_card_form[1].len()),
                chunks[2],
            );
            rect.set_cursor(x_pos, y_pos);
        } else {
            rect.set_cursor(chunks[2].x + 1, chunks[2].y + 1);
        }
    } else if app.state.focus == Focus::NewCardDueDate
        && app.state.app_status == AppStatus::UserInput
    {
        if app.state.current_cursor_position.is_some() {
            let (x_pos, y_pos) = calculate_cursor_position(
                wrapped_card_due_date_text,
                app.state
                    .current_cursor_position
                    .unwrap_or_else(|| app.state.new_card_form[2].len()),
                chunks[3],
            );
            rect.set_cursor(x_pos, y_pos);
        } else {
            rect.set_cursor(chunks[3].x + 1, chunks[3].y + 1);
        }
    }

    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_load_save<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let default_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        DEFAULT_STYLE
    };
    let help_key_style = if app.state.popup_mode.is_some() {
        INACTIVE_TEXT_STYLE
    } else {
        HELP_KEY_STYLE
    };
    let main_chunks = if app.config.enable_mouse_support {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(68),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(rect.size())
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(rect.size())
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(9),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let title_paragraph = Paragraph::new("Load a Save")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(default_style);
    rect.render_widget(title_paragraph, chunks[0]);

    let item_list = get_available_local_savefiles();
    let item_list = if item_list.is_none() {
        Vec::new()
    } else {
        item_list.unwrap()
    };
    if item_list.len() > 0 {
        // make a list from the Vec<string> of savefiles
        let items: Vec<ListItem> = item_list
            .iter()
            .map(|i| ListItem::new(i.to_string()))
            .collect();
        let choice_list = List::new(items)
            .block(
                Block::default()
                    .title("Available Saves")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(LIST_SELECT_STYLE)
            .highlight_symbol(LIST_SELECTED_SYMBOL)
            .style(default_style);

        if !(app.state.popup_mode.is_some()
            && app.state.popup_mode.unwrap() == PopupMode::CommandPalette)
        {
            if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, chunks[1]) {
                app.state.mouse_focus = Some(Focus::LoadSave);
                let top_of_list = chunks[1].y + 1;
                let mut bottom_of_list = chunks[1].y + item_list.len() as u16;
                if bottom_of_list > chunks[1].bottom() {
                    bottom_of_list = chunks[1].bottom();
                }
                let mouse_y = app.state.current_mouse_coordinates.1;
                if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
                    app.state
                        .load_save_state
                        .select(Some((mouse_y - top_of_list) as usize));
                }
            }
        }
        rect.render_stateful_widget(choice_list, chunks[1], &mut app.state.load_save_state);
    } else {
        let no_saves_paragraph = Paragraph::new("No saves found")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(LOG_ERROR_STYLE);
        rect.render_widget(no_saves_paragraph, chunks[1]);
    }

    let delete_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Delete focused element")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let up_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go up")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();
    let down_key = app
        .state
        .keybind_store
        .iter()
        .find(|x| x[1] == "Go down")
        .unwrap_or(&vec!["".to_string(), "".to_string()])[0]
        .clone();

    let help_text = Spans::from(vec![
        Span::styled("Use ", default_style),
        Span::styled(&up_key, help_key_style),
        Span::styled(" and ", default_style),
        Span::styled(&down_key, help_key_style),
        Span::styled("to navigate", default_style),
        Span::raw("; "),
        Span::styled("<Enter>", help_key_style),
        Span::styled(" to Load the save file", default_style),
        Span::raw("; "),
        Span::styled("<Esc>", help_key_style),
        Span::styled(" to cancel", default_style),
        Span::raw("; "),
        Span::styled(delete_key, help_key_style),
        Span::styled("to delete a save file", default_style),
        Span::styled(
            ". If using a mouse click on a save file to preview",
            default_style,
        ),
    ]);
    let help_paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true })
        .style(default_style);
    rect.render_widget(help_paragraph, chunks[2]);

    // preview pane
    if app.state.load_save_state.selected().is_none() {
        let preview_paragraph =
            Paragraph::new(format!("Select a save file with {} or {} to preview or Click on a save file to preview if using a mouse", up_key, down_key))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(default_style)
                .wrap(Wrap { trim: true });
        rect.render_widget(preview_paragraph, main_chunks[1]);
    } else {
        if app.state.preview_boards_and_cards.is_none() {
            let loading_text = if app.config.enable_mouse_support {
                "Click on a save file to preview"
            } else {
                "Loading preview..."
            };
            let preview_paragraph = Paragraph::new(loading_text)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(default_style)
                .wrap(Wrap { trim: true });
            rect.render_widget(preview_paragraph, main_chunks[1]);
        } else {
            render_body(rect, main_chunks[1], app, true)
        }
    }
    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_toast<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    // get the latest MAX_TOASTS_TO_DISPLAY number of toasts from app.state.toasts
    let all_toasts = &app.state.toasts;
    let mut loading_toasts = all_toasts
        .iter()
        .filter(|x| x.toast_type == ToastType::Loading)
        .collect::<Vec<&ToastWidget>>();
    let toasts = if loading_toasts.len() > 0 {
        // if loading_toasts are > MAX_TOASTS_TO_DISPLAY then put the loading toasts in the order of start time where the oldest is at the top only put MAX_TOASTS_TO_DISPLAY - 1 loading toasts and put the latest regular toast at the bottom
        let sorted_loading_toasts = if loading_toasts.len() > MAX_TOASTS_TO_DISPLAY - 1 {
            loading_toasts.sort_by(|a, b| a.start_time.cmp(&b.start_time));
            loading_toasts
                .iter()
                .map(|x| *x)
                .take(MAX_TOASTS_TO_DISPLAY - 1)
                .rev()
                .collect::<Vec<&ToastWidget>>()
        } else {
            loading_toasts
        };
        // append the latest regular toast to the loading toasts till length is MAX_TOASTS_TO_DISPLAY
        let mut toasts = sorted_loading_toasts;
        let mut regular_toasts = all_toasts
            .iter()
            .filter(|x| x.toast_type != ToastType::Loading)
            .collect::<Vec<&ToastWidget>>();
        regular_toasts.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        while toasts.len() < MAX_TOASTS_TO_DISPLAY {
            if let Some(toast) = regular_toasts.pop() {
                toasts.push(toast);
            } else {
                break;
            }
        }
        // check if any more loading toasts are there and if so then append them to the toasts if there is space
        if toasts.len() < MAX_TOASTS_TO_DISPLAY {
            let mut loading_toasts = all_toasts
                .iter()
                .filter(|x| x.toast_type == ToastType::Loading)
                .collect::<Vec<&ToastWidget>>();
            loading_toasts.sort_by(|a, b| a.start_time.cmp(&b.start_time));
            while toasts.len() < MAX_TOASTS_TO_DISPLAY {
                if let Some(toast) = loading_toasts.pop() {
                    // check if the toast is already present in toasts
                    if !toasts.contains(&toast) {
                        toasts.push(toast);
                    }
                } else {
                    break;
                }
            }
        }
        toasts
    } else {
        app.state
            .toasts
            .iter()
            .rev()
            .take(MAX_TOASTS_TO_DISPLAY)
            .rev()
            .collect::<Vec<&ToastWidget>>()
    };

    if toasts.len() == 0 {
        return;
    }
    let mut total_height_rendered = 1; // for messages indicator

    // loop through the toasts and draw them
    for toast in toasts.iter() {
        let toast_style = Style::default().fg(tui::style::Color::Rgb(
            toast.toast_color.0,
            toast.toast_color.1,
            toast.toast_color.2,
        ));
        let toast_title = match toast.toast_type {
            ToastType::Error => "Error",
            ToastType::Info => "Info",
            ToastType::Warning => "Warning",
            ToastType::Loading => "Loading",
        };
        // if the toast type is loading display a spinner next to the title and use the duration.elapsed() to determine the current frame of the spinner
        let toast_title = match toast.toast_type {
            ToastType::Loading => {
                let spinner_frames = &SPINNER_FRAMES;
                let frame =
                    (toast.start_time.elapsed().as_millis() / 100) % spinner_frames.len() as u128;
                let frame = frame as usize;
                format!("{} {}", spinner_frames[frame], toast_title)
            }
            _ => toast_title.to_string(),
        };
        let x_offset = rect.size().width - (rect.size().width / SCREEN_TO_TOAST_WIDTH_RATIO);
        let lines = textwrap::wrap(
            &toast.message,
            ((rect.size().width / SCREEN_TO_TOAST_WIDTH_RATIO) - 2) as usize,
        )
        .iter()
        .map(|x| Spans::from(x.to_string()))
        .collect::<Vec<Spans>>();
        let toast_height = lines.len() as u16 + 2;
        let toast_block = Block::default()
            .title(toast_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(toast_style);
        let toast_paragraph = Paragraph::new(lines)
            .block(toast_block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .style(toast_style);
        rect.render_widget(
            Clear,
            Rect::new(
                x_offset,
                total_height_rendered,
                rect.size().width / SCREEN_TO_TOAST_WIDTH_RATIO,
                toast_height,
            ),
        );
        rect.render_widget(
            toast_paragraph,
            Rect::new(
                x_offset,
                total_height_rendered,
                rect.size().width / SCREEN_TO_TOAST_WIDTH_RATIO,
                toast_height,
            ),
        );
        total_height_rendered += toast_height;
    }

    // display a total count of toasts on the top right corner
    let text_offset = 15;
    let toast_count = app.state.toasts.len();
    let toast_count_text = format!(" {} Message(s)", toast_count);
    let toast_count_paragraph = Paragraph::new(toast_count_text)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .border_type(BorderType::Rounded),
        )
        .style(DEFAULT_STYLE);
    let message_area = Rect::new(rect.size().width - text_offset, 0, text_offset, 1);
    rect.render_widget(Clear, message_area);
    rect.render_widget(toast_count_paragraph, message_area);
}

pub fn render_view_card<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let popup_area = centered_rect(90, 90, rect.size());
    rect.render_widget(Clear, popup_area);
    let card_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(12)])
        .margin(1)
        .split(popup_area);

    // get the current board id from app.state.current_board_id
    // get the board with the id
    // get the current card id from app.state.current_card_id
    // get the current card From the board

    if let Some(board) = app
        .boards
        .iter()
        .find(|b| b.id == app.state.current_board_id.unwrap_or_else(|| 0))
    {
        if let Some(card) = board
            .cards
            .iter()
            .find(|c| c.id == app.state.current_card_id.unwrap_or_else(|| 0))
        {
            let board_name = board.name.clone();
            let card_name = card.name.clone();
            let card_description = card.description.clone();

            let main_block = Block::default()
                .title(format!("{} >> Board({})", card_name, board_name))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            rect.render_widget(main_block, popup_area);

            let description_paragraph = Paragraph::new(card_description)
                .block(
                    Block::default()
                        .title("Description")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: false });
            rect.render_widget(description_paragraph, card_chunks[0]);

            let card_date_created = format!("Created: {}", card.date_created);
            let card_date_modified = format!("Modified: {}", card.date_modified);
            let card_date_completed = format!("Completed: {}", card.date_completed);
            let card_priority = format!("Priority: {}", card.priority.to_string());
            let card_status = format!("Status: {}", card.card_status.to_string());
            let card_tags = format!("Tags: {}", card.tags.join(", "));
            let card_comments = format!("Comments: {}", card.comments.join(", "));
            let parsed_due_date =
                NaiveDateTime::parse_from_str(&card.date_due, "%d/%m/%Y-%H:%M:%S");
            // card due date is in the format dd/mm/yyyy check if the due date is within WARNING_DUE_DATE_DAYS if so highlight it
            let card_due_date_styled = if parsed_due_date.is_ok() {
                let parsed_due_date = parsed_due_date.unwrap();
                let today = Local::now().naive_local();
                let days_left = parsed_due_date.signed_duration_since(today).num_days();
                if days_left <= app.config.warning_delta.into() && days_left >= 0 {
                    Span::styled(
                        format!("Due: {}", card.date_due),
                        CARD_DUE_DATE_WARNING_STYLE,
                    )
                } else if days_left < 0 {
                    Span::styled(
                        format!("Due: {}", card.date_due),
                        CARD_DUE_DATE_CRITICAL_STYLE,
                    )
                } else {
                    Span::styled(
                        format!("Due: {}", card.date_due),
                        CARD_DUE_DATE_DEFAULT_STYLE,
                    )
                }
            } else {
                Span::styled(
                    format!("Due: {}", card.date_due),
                    CARD_DUE_DATE_DEFAULT_STYLE,
                )
            };
            let card_priority_styled = if card.priority == CardPriority::High {
                Span::styled(card_priority, CARD_PRIORITY_HIGH_STYLE)
            } else if card.priority == CardPriority::Medium {
                Span::styled(card_priority, CARD_PRIORITY_MEDIUM_STYLE)
            } else if card.priority == CardPriority::Low {
                Span::styled(card_priority, CARD_PRIORITY_LOW_STYLE)
            } else {
                Span::raw(card_priority)
            };
            let card_status_styled = if card.card_status == CardStatus::Complete {
                Span::styled(card_status, CARD_COMPLETED_STATUS_STYLE)
            } else if card.card_status == CardStatus::Active {
                Span::styled(card_status, CARD_ACTIVE_STATUS_STYLE)
            } else if card.card_status == CardStatus::Stale {
                Span::styled(card_status, CARD_STALE_STATUS_STYLE)
            } else {
                Span::raw(card_status)
            };
            let card_extra_info = Paragraph::new(vec![
                Spans::from(card_date_created),
                Spans::from(card_date_modified),
                Spans::from(card_due_date_styled),
                Spans::from(card_date_completed),
                Spans::from(card_priority_styled),
                Spans::from(card_status_styled),
                Spans::from(card_tags),
                Spans::from(card_comments),
            ])
            .block(
                Block::default()
                    .title("Card Info")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
            rect.render_widget(card_extra_info, card_chunks[1]);
        } else {
            // render no cards found in <> board
            let board_name = board.name.clone();
            let no_cards_found =
                Paragraph::new(format!("No cards found in the board \"{}\"", board_name))
                    .block(
                        Block::default()
                            .title("Card Info")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .style(ERROR_TEXT_STYLE),
                    )
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
            rect.render_widget(no_cards_found, popup_area);
        }
        if app.config.enable_mouse_support {
            render_close_button(rect, app);
        }
    }
}

pub fn render_command_palette<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let current_search_text_input = app.state.current_user_input.clone();
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(rect.size());

    let search_results = if app.command_palette.search_results.is_some() {
        // convert the vec of strings to a vec of list items
        let raw_search_results = app.command_palette.search_results.as_ref().unwrap();

        let mut list_items = vec![];
        // make a for loop and go through the raw search results and check if the current item has a character that is in the charaters of the search string highlight it with selected style using Span::Styled
        for item in raw_search_results {
            let mut spans = vec![];
            for (_, c) in item.to_string().chars().enumerate() {
                if current_search_text_input
                    .to_lowercase()
                    .contains(c.to_string().to_lowercase().as_str())
                {
                    spans.push(Span::styled(c.to_string(), FOCUSED_ELEMENT_STYLE));
                } else {
                    spans.push(Span::styled(c.to_string(), DEFAULT_STYLE));
                }
            }
            list_items.push(ListItem::new(vec![Spans::from(spans)]));
        }
        list_items
    } else {
        app.command_palette
            .available_commands
            .iter()
            .map(|s| ListItem::new(vec![Spans::from(s.as_str().to_string())]))
            .collect::<Vec<ListItem>>()
    };

    let search_results_length = if (search_results.len() + 2) > 3 {
        if (search_results.len() + 2) > (rect.size().height - 7) as usize {
            rect.size().height - 7
        } else {
            (search_results.len() + 2) as u16
        }
    } else {
        3
    };

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(search_results_length as u16),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(horizontal_chunks[1]);

    let search_box_text = if app.state.current_user_input.is_empty() {
        vec![Spans::from("Start typing to search")]
    } else {
        vec![Spans::from(app.state.current_user_input.clone())]
    };

    let current_cursor_position = if app.state.current_cursor_position.is_some() {
        app.state.current_cursor_position.unwrap() as u16
    } else {
        app.state.current_user_input.len() as u16
    };
    let x_offset = current_cursor_position % (vertical_chunks[1].width - 2);
    let y_offset = current_cursor_position / (vertical_chunks[1].width - 2);
    let x_cursor_position = vertical_chunks[1].x + x_offset + 1;
    let y_cursor_position = vertical_chunks[1].y + y_offset + 1;
    rect.set_cursor(x_cursor_position, y_cursor_position);

    // make a search bar and display all the commands that match the search below it in a list
    let search_bar = Paragraph::new(search_box_text)
        .block(
            Block::default()
                .title("Command Palette")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: false });
    rect.render_widget(Clear, vertical_chunks[1]);
    rect.render_widget(search_bar, vertical_chunks[1]);

    let search_results = List::new(search_results)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(LIST_SELECT_STYLE)
        .highlight_symbol(">>");

    rect.render_widget(Clear, vertical_chunks[2]);
    rect.render_stateful_widget(
        search_results,
        vertical_chunks[2],
        &mut app.state.command_palette_list_state,
    );

    if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, vertical_chunks[2]) {
        app.state.mouse_focus = Some(Focus::CommandPalette);
        let top_of_list = vertical_chunks[2].y + 1;
        let mut bottom_of_list = vertical_chunks[2].y + search_results_length;
        if bottom_of_list > vertical_chunks[2].bottom() {
            bottom_of_list = vertical_chunks[2].bottom();
        }
        let mouse_y = app.state.current_mouse_coordinates.1;
        if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
            app.state
                .command_palette_list_state
                .select(Some((mouse_y - top_of_list) as usize));
        }
    }

    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_change_ui_mode_popup<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let all_ui_modes = UiMode::all()
        .iter()
        .map(|s| ListItem::new(vec![Spans::from(s.as_str().to_string())]))
        .collect::<Vec<ListItem>>();

    let percent_height =
        (((all_ui_modes.len() + 3) as f32 / rect.size().height as f32) * 100.0) as u16;

    let popup_area = centered_rect(50, percent_height, rect.size());

    if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, popup_area) {
        app.state.mouse_focus = Some(Focus::ChangeUiModePopup);
        let top_of_list = popup_area.y + 1;
        let mut bottom_of_list = popup_area.y + all_ui_modes.len() as u16;
        if bottom_of_list > popup_area.bottom() {
            bottom_of_list = popup_area.bottom();
        }
        let mouse_y = app.state.current_mouse_coordinates.1;
        if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
            app.state
                .default_view_state
                .select(Some((mouse_y - top_of_list) as usize));
        }
    }
    let ui_modes = List::new(all_ui_modes)
        .block(
            Block::default()
                .title("Change UI Mode")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(LIST_SELECT_STYLE)
        .highlight_symbol(">>");

    rect.render_widget(Clear, popup_area);
    rect.render_stateful_widget(ui_modes, popup_area, &mut app.state.default_view_state);

    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_change_current_card_status_popup<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let mut card_name = String::new();
    let mut board_name = String::new();
    if let Some(current_board_id) = app.state.current_board_id {
        if let Some(current_board) = app.boards.iter().find(|b| b.id == current_board_id) {
            if let Some(current_card_id) = app.state.current_card_id {
                if let Some(current_card) =
                    current_board.cards.iter().find(|c| c.id == current_card_id)
                {
                    card_name = current_card.name.clone();
                    board_name = current_board.name.clone();
                }
            }
        }
    }
    let all_statuses = CardStatus::all()
        .iter()
        .map(|s| ListItem::new(vec![Spans::from(s.to_string())]))
        .collect::<Vec<ListItem>>();
    let percent_height =
        (((all_statuses.len() + 3) as f32 / rect.size().height as f32) * 100.0) as u16;
    let popup_area = centered_rect(50, percent_height, rect.size());
    if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, popup_area) {
        app.state.mouse_focus = Some(Focus::ChangeCardStatusPopup);
        let top_of_list = popup_area.y + 1;
        let mut bottom_of_list = popup_area.y + all_statuses.len() as u16;
        if bottom_of_list > popup_area.bottom() {
            bottom_of_list = popup_area.bottom();
        }
        let mouse_y = app.state.current_mouse_coordinates.1;
        if mouse_y >= top_of_list && mouse_y <= bottom_of_list {
            app.state
                .card_status_selector_state
                .select(Some((mouse_y - top_of_list) as usize));
        }
    }
    let statuses = List::new(all_statuses)
        .block(
            Block::default()
                .title(format!(
                    "Changing Status of \"{}\" in {}",
                    card_name, board_name
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(LIST_SELECT_STYLE)
        .highlight_symbol(">>");

    rect.render_widget(Clear, popup_area);
    rect.render_stateful_widget(
        statuses,
        popup_area,
        &mut app.state.card_status_selector_state,
    );

    if app.config.enable_mouse_support {
        render_close_button(rect, app);
    }
}

pub fn render_debug_panel<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let current_ui_mode = &app.state.ui_mode.to_string();
    let popup_mode = if app.state.popup_mode.is_some() {
        app.state.popup_mode.as_ref().unwrap().to_string()
    } else {
        "None".to_string()
    };
    let tickrate = app.config.tickrate;
    let ui_render_time = if app.state.ui_render_time.is_some() {
        let render_time = app.state.ui_render_time.unwrap();
        // render time is in microseconds, so we convert it to milliseconds if render time is greater than 1 millisecond
        if render_time > 1000 {
            format!("{}ms", render_time / 1000)
        } else {
            format!("{}μs", render_time)
        }
    } else {
        "None".to_string()
    };
    let current_board_id = app.state.current_board_id;
    let current_card_id = app.state.current_card_id;

    let menu_area = top_left_rect(30, 30, rect.size());
    let debug_panel = Paragraph::new(vec![
        Spans::from(format!("UI Mode: {}", current_ui_mode)),
        Spans::from(format!("Popup Mode: {}", popup_mode)),
        Spans::from(format!("Tickrate: {}ms", tickrate)),
        Spans::from(format!("UI Render Time: {}", ui_render_time)),
        Spans::from(format!("Current Board ID: {:?}", current_board_id)),
        Spans::from(format!("Current Card ID: {:?}", current_card_id)),
    ])
    .block(
        Block::default()
            .title("Debug Panel")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(LOG_DEBUG_STYLE),
    )
    .wrap(Wrap { trim: false });
    rect.render_widget(Clear, menu_area);
    rect.render_widget(debug_panel, menu_area);
}

fn check_if_mouse_is_in_area(mouse_coordinates: (u16, u16), rect_to_check: Rect) -> bool {
    let (x, y) = mouse_coordinates;
    let (x1, y1, x2, y2) = (
        rect_to_check.x,
        rect_to_check.y,
        rect_to_check.x + rect_to_check.width,
        rect_to_check.y + rect_to_check.height,
    );
    if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
        return true;
    }
    false
}

fn render_close_button<B>(rect: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let close_btn_area = Rect::new(rect.size().width - 3, 0, 3, 3);
    let close_btn_style =
        if check_if_mouse_is_in_area(app.state.current_mouse_coordinates, close_btn_area) {
            app.state.mouse_focus = Some(Focus::CloseButton);
            MOUSE_HIGHLIGHT_STYLE
        } else {
            ERROR_TEXT_STYLE
        };
    // render a X in the top right corner of the rect
    let close_btn = Paragraph::new(vec![Spans::from("X")])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(close_btn_style),
        )
        .alignment(Alignment::Right);
    rect.render_widget(close_btn, close_btn_area);
}
