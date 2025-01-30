use iced::border::Radius;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Vector, Shadow};

use chrono::{NaiveDate, Datelike};

use crate::Message;
use crate::date_manager::DateManager;

pub fn view(state: &DateManager) -> Element<Message> {
    let header = render_header(state);
    let calendar_grid = render_calendar_grid(state);

    let content = column![
        header,
        calendar_grid,
    ]
    .spacing(20)
    .padding(20)
    .width(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Start)
        .into()
}

fn render_header(state: &DateManager) -> Element<Message> {
    let month_year = format!("{} {}", state.selected_month(), state.selected_year());

    row![
        container(button("←")
            .on_press(Message::PreviousMonth)
            .padding([10, 30]))
            .width(Length::FillPortion(1))
            .align_x(Alignment::Start),
        container(text(month_year).size(32))
            .width(Length::FillPortion(2))
            .align_x(Alignment::Center),
        container(button("→")
            .on_press(Message::NextMonth)
            .padding([10, 30]))
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
    ]
    .spacing(20)
    .width(Length::Fill)
    .into()
}

fn render_calendar_grid(state: &DateManager) -> Element<Message> {
    let weekdays = render_weekday_header(state);
    let date_grid = render_date_grid(state);

    column![
        weekdays,
        date_grid,
    ]
    .into()
}

fn render_weekday_header(_state: &DateManager) -> Element<Message> {
    row![
        container(text("Sunday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Monday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Tuesday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Wednesday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Thursday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Friday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
        container(text("Saturday")
            .align_x(Alignment::Center))
            .width(Length::Fill)
            .padding(10),
    ]
    .width(Length::Fill)
    .into()
}

fn render_date_grid(state: &DateManager) -> Element<Message> {
    let dates = state.calendar_dates();
    
    column![
        // Week 1
        row![
            render_date_cell(state, dates[0]),
            render_date_cell(state, dates[1]),
            render_date_cell(state, dates[2]),
            render_date_cell(state, dates[3]),
            render_date_cell(state, dates[4]),
            render_date_cell(state, dates[5]),
            render_date_cell(state, dates[6]),
        ].width(Length::Fill),
        // Week 2
        row![
            render_date_cell(state, dates[7]),
            render_date_cell(state, dates[8]),
            render_date_cell(state, dates[9]),
            render_date_cell(state, dates[10]),
            render_date_cell(state, dates[11]),
            render_date_cell(state, dates[12]),
            render_date_cell(state, dates[13]),
        ].width(Length::Fill),
        // Week 3
        row![
            render_date_cell(state, dates[14]),
            render_date_cell(state, dates[15]),
            render_date_cell(state, dates[16]),
            render_date_cell(state, dates[17]),
            render_date_cell(state, dates[18]),
            render_date_cell(state, dates[19]),
            render_date_cell(state, dates[20]),
        ].width(Length::Fill),
        // Week 4
        row![
            render_date_cell(state, dates[21]),
            render_date_cell(state, dates[22]),
            render_date_cell(state, dates[23]),
            render_date_cell(state, dates[24]),
            render_date_cell(state, dates[25]),
            render_date_cell(state, dates[26]),
            render_date_cell(state, dates[27]),
        ].width(Length::Fill),
        // Week 5
        row![
            render_date_cell(state, dates[28]),
            render_date_cell(state, dates[29]),
            render_date_cell(state, dates[30]),
            render_date_cell(state, dates[31]),
            render_date_cell(state, dates[32]),
            render_date_cell(state, dates[33]),
            render_date_cell(state, dates[34]),
        ].width(Length::Fill),
        // Week 6
        row![
            render_date_cell(state, dates[35]),
            render_date_cell(state, dates[36]),
            render_date_cell(state, dates[37]),
            render_date_cell(state, dates[38]),
            render_date_cell(state, dates[39]),
            render_date_cell(state, dates[40]),
            render_date_cell(state, dates[41]),
        ].width(Length::Fill),
    ]
    .spacing(2)
    .width(Length::Fill)
    .into()
}

fn render_date_cell(state: &DateManager, date: NaiveDate) -> Element<Message> {
    let is_current = state.is_current_date(date);
    let is_selected_month = state.is_selected_month(date);

    let style = if !is_selected_month{
        container::Style {
            text_color: Some(Color::BLACK),
            background: Some(iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            border: Border {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 0.0,
                    bottom_right: 0.0,
                },
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            }
        }
    } else if is_current {
        container::Style {
            text_color: Some(Color::BLACK),
            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.9, 1.0))),  // Light blue
            border: Border {
                color: Color::from_rgb(0.7, 0.8, 0.9),  // Darker blue border
                width: 1.0,
                radius: Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 0.0,
                    bottom_right: 0.0,
                },
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            }
        }
    } else {
        container::Style {
            text_color: Some(Color::BLACK),
            background: Some(iced::Background::Color(Color::WHITE)),
            border: Border {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_left: 0.0,
                    bottom_right: 0.0,
                },
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            }
        }
    };

    container(
        text(date.day().to_string())
            .align_x(Alignment::Center)
            .width(Length::Fill)
    )
    .style(move |_| style)
    .width(Length::Fill)
    .padding([40, 10])
    .into()
}

