use iced::Alignment::Start;
use iced::widget::{
    Column, button, column, container, row, rule, scrollable, space, stack, text, text_input,
};
use iced::{Border, Color, Element, Fill, Length, Shadow, Task, Theme};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

mod security;

#[derive(Debug, Clone, Default)]
enum Screen {
    #[default]
    MainMenu,
    Overview(bool),
    Settings,
    Error(u8),
}

#[derive(Debug, Clone)]
enum Message {
    NewPortfolio,
    LoadPortfolio,
    SavePortfolio,
    SavePortfolioAs,
    Settings,
    Debug,
    OpenSecurityNameInput,
    AddSecurity(String),
    OpenSecurity(u8),
    OpenEntryInput,
    AddEntry(String, String, String),
    NewInput(String, String),
    UpdateCurrentValue,
}

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run() //.run_with(App::new)
}

#[derive(Default, Debug)]
struct App {
    current_screen: Screen,
    main_menu: MainMenu,
    overview: Overview,
    inputs_config: [Vec<(String, String)>; 2],
    current_input: Option<usize>,
    current_file_path: Option<std::path::PathBuf>,
}

impl App {
    fn title() -> String {
        "Portfolio".to_string()
    }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_screen: Screen::MainMenu,
                main_menu: MainMenu::new(),
                overview: Overview::new(),
                inputs_config: [
                    vec![("Security Name".to_string(), String::new())],
                    vec![
                        ("Date".to_string(), String::new()),
                        ("Amount".to_string(), String::new()),
                        ("Price per Unit".to_string(), String::new()),
                    ],
                ],
                current_input: None,
                current_file_path: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, mut message: Message) -> Task<Message> {
        println!("Message: {:#?}", message);
        let current_screen = self.current_screen.clone();
        match &message {
            Message::SavePortfolio => {
                // Save to current file, or open dialog if no file is set
                if let Some(path) = &self.current_file_path {
                    // Save to existing file
                    match serde_json::to_string_pretty(&self.overview) {
                        Ok(json) => match std::fs::write(path, json) {
                            Ok(_) => println!("Saved to {:?}", path),
                            Err(e) => println!("Failed to save: {}", e),
                        },
                        Err(e) => println!("Failed to serialize: {}", e),
                    }
                } else {
                    return self.update(Message::SavePortfolioAs);
                }
            }
            Message::SavePortfolioAs => {
                if let Some(path) = FileDialog::new()
                    .set_file_name("portfolio.json")
                    .add_filter("JSON", &["json"])
                    .save_file()
                {
                    // Serialize and save
                    match serde_json::to_string_pretty(&self.overview) {
                        Ok(json) => match std::fs::write(&path, json) {
                            Ok(_) => println!("Saved successfully"),
                            Err(e) => println!("Failed to save: {}", e),
                        },
                        Err(e) => println!("Failed to serialize: {}", e),
                    }
                }
            }
            Message::LoadPortfolio => self.load_file(),
            Message::NewInput(key, value) => {
                if let Some(current_input) = self.current_input {
                    // Find and update the matching key
                    if let Some(entry) = self.inputs_config[current_input]
                        .iter_mut()
                        .find(|(k, _)| k == key)
                    {
                        entry.1 = value.clone();
                    }
                } else {
                    println!("Error in current_input");
                    self.current_screen = Screen::Error(3);
                }
            }
            // handle "OpenSecurityNameINput, AddSecurity" Sequence
            Message::OpenSecurityNameInput => {
                self.current_input = Some(0);
            }
            Message::AddSecurity(_) => {
                if let Some(0) = self.current_input {
                    // Find the "Security Name" value
                    if let Some((_, name)) = self.inputs_config[0]
                        .iter()
                        .find(|(k, _)| k == "Security Name")
                    {
                        self.current_input = None;
                        message = Message::AddSecurity(name.clone());
                    } else {
                        println!("Error in get");
                        self.current_screen = Screen::Error(2);
                    }
                } else {
                    println!("Error in current_input");
                    self.current_screen = Screen::Error(2);
                }
            }
            // handle "OpenEntryInput, AddEntry" Sequence
            Message::OpenEntryInput => {
                self.current_input = Some(1);
            }
            Message::AddEntry(_, _, _) => {
                println!("ADD ENTRY");
                if let Some(1) = self.current_input {
                    // Helper function to find value by key
                    let find_value = |key: &str| {
                        self.inputs_config[1]
                            .iter()
                            .find(|(k, _)| k == key)
                            .map(|(_, v)| v.clone())
                            .unwrap_or_default()
                    };

                    let date = find_value("Date");
                    let quantity = find_value("Amount");
                    let price = find_value("Price per Unit");

                    self.current_input = None;
                    message = Message::AddEntry(date, quantity, price);
                } else {
                    println!("Error in current_input");
                    self.current_screen = Screen::Error(2);
                }
            }
            _ => {}
        }
        self.current_screen = self.overview.update(message);
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match self.current_screen {
            Screen::MainMenu => {
                container(column![self.view_utilities(), self.main_menu.view()]).into()
            }
            Screen::Overview(active_pop_up) => {
                println!("{:#?}", self);
                if active_pop_up {
                    if let Some(current_input) = self.current_input {
                        let message = match current_input {
                            0 => Message::AddSecurity("".to_string()),
                            1 => Message::AddEntry("".to_string(), "".to_string(), "".to_string()),
                            _ => Message::AddSecurity("".to_string()),
                        };
                        println!("Activate PopUp");
                        container(stack![
                            column![self.view_utilities(), self.overview.view()],
                            self.pop_up(
                                &self.inputs_config[current_input], //reference to the HashMap
                                message
                            )
                        ])
                        .into()
                    } else {
                        println!("Error in current_input");
                        text("Error").into()
                    }
                } else {
                    container(column![self.view_utilities(), self.overview.view()]).into()
                }
            }
            Screen::Settings => container(column![text("Settings!").size(50),]).into(),
            Screen::Error(error_message) => container(column![
                text(format!("Error code: {}", error_message.to_string())).size(50),
            ])
            .into(),
        }
    }

    fn view_utilities(&self) -> Element<'_, Message> {
        container(row![
            button("New").on_press(Message::NewPortfolio),
            button("Open").on_press(Message::LoadPortfolio),
            button("Save").on_press(Message::SavePortfolio),
            button("Save as").on_press(Message::SavePortfolioAs),
            button("Settings").on_press(Message::Settings),
            button("Debug").on_press(Message::Debug),
        ])
        .align_x(Start)
        .align_y(Start)
        .style(|theme: &Theme| container::Style {
            text_color: None,
            background: None,
            shadow: Shadow::default(),
            snap: false,
            border: Border {
                color: Color::from_rgb(0.0, 0.0, 0.0),
                width: 2.0,
                radius: 1.0.into(),
            },
        })
        .into()
    }

    fn pop_up(&self, entries: &Vec<(String, String)>, message: Message) -> Element<'_, Message> {
        let inputs: Column<_> = column(entries.iter().map(|(key, value)| {
            let key_clone = key.clone();
            row![
                container(text(key.clone())),
                container(
                    text_input("", value)
                        .on_input(move |new_val| { Message::NewInput(key_clone.clone(), new_val) })
                )
            ]
            .spacing(10)
            .into()
        }));
        container(
            container(column![inputs, button("Confirm").on_press(message)].spacing(10))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .padding(10)
                .height(Length::Shrink)
                .width(Length::FillPortion(2))
                .style(container::bordered_box),
        )
        .center(Fill)
        .into()
    }

    fn load_file(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
            match std::fs::read_to_string(&path) {
                Ok(json) => {
                    match serde_json::from_str::<Overview>(&json) {
                        // Deserialize to Overview
                        Ok(overview) => {
                            self.overview = overview; // Replace the current overview
                            self.current_screen = Screen::Overview(false);
                        }
                        Err(e) => {
                            println!("Failed to parse on load: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read file: {}", e);
                }
            }
        }
    }
}

#[derive(Default, Debug)]
struct MainMenu {
    //text: String,
}

impl MainMenu {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, message: Message) -> Screen {
        match message {
            Message::NewPortfolio => Screen::Overview(false),
            Message::LoadPortfolio => Screen::Overview(false),
            Message::SavePortfolio => Screen::Overview(false),
            Message::Settings => Screen::Settings,
            Message::Debug => {
                println!("{:#?}", self);
                Screen::Overview(false)
            }
            _ => Screen::Error(1),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(text("")).into()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Overview {
    securities: Vec<security::Security>,
    open_security: Option<u8>,
    last_security_id: u8,
}

impl Overview {
    fn new() -> Self {
        Self {
            securities: Vec::new(),
            open_security: None,
            last_security_id: 0,
        }
    }

    fn update(&mut self, message: Message) -> Screen {
        match message {
            Message::NewPortfolio => Screen::Overview(false),
            Message::LoadPortfolio => Screen::Overview(false),
            Message::SavePortfolio => Screen::Overview(false),
            Message::Settings => Screen::Settings,
            Message::OpenSecurityNameInput => Screen::Overview(true),
            Message::AddSecurity(security_name) => {
                if self.securities.len() == 0 {
                    self.last_security_id = 0;
                } else {
                    self.last_security_id += 1;
                }
                self.securities.push(security::Security::new(
                    self.last_security_id,
                    security_name,
                    0,
                ));
                Screen::Overview(false)
            }
            Message::NewInput(_, _) => Screen::Overview(true),
            Message::OpenSecurity(id) => {
                self.open_security = Some(id);
                Screen::Overview(false)
            }
            Message::OpenEntryInput => Screen::Overview(true),
            Message::AddEntry(date, quantity, price) => {
                if let Some(security_id) = self.open_security {
                    if let Some(security) = self.securities.iter_mut().find(|s| s.id == security_id)
                    {
                        security.add_entry(
                            date,
                            quantity.trim().parse::<u8>().unwrap(),
                            price.trim().replace(',', ".").parse::<f32>().unwrap(),
                        );
                        security.calculate_total_invested_value();
                    }
                }
                //self.securities.get(self.open_security);
                Screen::Overview(false)
            }
            Message::Debug => {
                println!("{:#?}", self);
                Screen::Overview(false)
            }
            _ => Screen::Error(1),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let security_details_container: Element<_> = if let Some(security_id) = self.open_security {
            // Find the matching security
            if let Some(security) = self.securities.iter().find(|s| s.id == security_id) {
                let entries_data = security.get_entries();
                let entries_column =
                    entries_data
                        .iter()
                        .fold(column![], |col, (date, quantity, value_per_unit)| {
                            col.push(
                                row![
                                    text("BUY").width(Length::FillPortion(2)),
                                    rule::vertical(1),
                                    text(date.clone()).width(Length::FillPortion(3)),
                                    rule::vertical(1),
                                    text(quantity.clone()).width(Length::FillPortion(2)),
                                    rule::vertical(1),
                                    text(value_per_unit.clone()).width(Length::FillPortion(2))
                                ]
                                .height(Length::Shrink),
                            )
                        });
                container(column![
                    text(format!("Security: {}", security.name)),
                    text(format!("id: {}", security.id)),
                    text(format!("quantity: {}", security.get_quantity())),
                    text(format!(
                        "total value: {}",
                        security.get_total_invested_value()
                    )),
                    button("Add Entry").on_press(Message::OpenEntryInput),
                    rule::horizontal(1),
                    row![
                        text("Action").width(Length::FillPortion(2)),
                        rule::vertical(1),
                        text("Date").width(Length::FillPortion(3)),
                        rule::vertical(1),
                        text("Quantity").width(Length::FillPortion(2)),
                        rule::vertical(1),
                        text("Buy value per unit").width(Length::FillPortion(2)),
                    ]
                    .height(Length::Shrink),
                    rule::horizontal(1),
                    entries_column
                ])
                .padding(20)
                .width(Length::FillPortion(2))
                .into()
            } else {
                space::horizontal().width(Length::FillPortion(2)).into()
            }
        } else {
            space::horizontal().width(Length::FillPortion(2)).into()
        };

        row![
            container(
                column![
                    text("Portfolio"),
                    row![
                        container(button("Add Security").on_press(Message::OpenSecurityNameInput))
                            .padding(20),
                        container(
                            button("Update Current Value").on_press(Message::UpdateCurrentValue)
                        )
                        .padding(20)
                    ],
                    rule::horizontal(1),
                    scrollable(
                        column(self.securities.iter().map(|security| {
                            container(
                                button(security.name.as_str())
                                    .on_press(Message::OpenSecurity(security.id))
                                    .padding(10),
                            )
                            .padding(10)
                            .into()
                        }))
                        .padding(10)
                    )
                ]
                .padding(10)
            )
            .padding(20)
            .align_left(Fill)
            .align_top(Fill)
            .width(Length::FillPortion(1)),
            security_details_container
        ]
        .into()
    }
}
