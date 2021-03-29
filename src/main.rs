use iced::{
    button, executor, scrollable, time, Align, Application, Button, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings,
    Subscription, Text,
};

use std::fmt;
use std::time::Instant;

pub fn main() -> iced::Result {
    Talos::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore,
    Crystal,
    Energy,
    Science,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const ORE_HARVEST_RATE: u32 = 1;

struct Talos {
    ore: u32,
    crystal: u32,
    energy: u32,
    science: u32,

    ore_harvest_units: u32,
    ore_harvest_unit_price: u32,
    crystal_harvest_units: u32,
    crystal_harvest_unit_price: u32,
    energy_harvest_units: u32,
    energy_harvest_unit_price: u32,
    science_harvest_units: u32,
    science_harvest_unit_price: u32,

    ore_harvest_units_incr_button: button::State,
    ore_harvest_units_decr_button: button::State,

    crystal_harvest_units_incr_button: button::State,
    crystal_harvest_units_decr_button: button::State,

    energy_harvest_units_incr_button: button::State,
    energy_harvest_units_decr_button: button::State,

    science_harvest_units_incr_button: button::State,
    science_harvest_units_decr_button: button::State,

    scroll: scrollable::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed(Resource),
    DecrementPressed(Resource),
    Tick(Instant),
}

impl Application for Talos {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Talos {
                ore: 0,
                ore_harvest_units: 1,
                ore_harvest_unit_price: 1,
                crystal: 0,
                crystal_harvest_units: 0,
                crystal_harvest_unit_price: 1,
                energy: 0,
                energy_harvest_units: 0,
                energy_harvest_unit_price: 1,
                science: 0,
                science_harvest_units: 0,
                science_harvest_unit_price: 1,

                ore_harvest_units_incr_button: button::State::default(),
                ore_harvest_units_decr_button: button::State::default(),
                crystal_harvest_units_incr_button: button::State::default(),
                crystal_harvest_units_decr_button: button::State::default(),
                energy_harvest_units_incr_button: button::State::default(),
                energy_harvest_units_decr_button: button::State::default(),
                science_harvest_units_incr_button: button::State::default(),
                science_harvest_units_decr_button: button::State::default(),

                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_instant) => {
                self.ore += self.ore_harvest_units * ORE_HARVEST_RATE;
            }
            Message::IncrementPressed(resource) => match resource {
                Resource::Ore => {
                    if self.ore >= self.ore_harvest_unit_price {
                        self.ore -= self.ore_harvest_unit_price;
                        self.ore_harvest_units += 1;
                        self.ore_harvest_unit_price = 2u32.pow(self.ore_harvest_units)
                    }
                }
                Resource::Crystal => {
                    if self.crystal >= self.crystal_harvest_unit_price {
                        self.crystal -= self.crystal_harvest_unit_price;
                        self.crystal_harvest_units += 1;
                        self.crystal_harvest_unit_price = 2u32.pow(self.crystal_harvest_units)
                    }
                }
                Resource::Energy => {
                    if self.energy >= self.energy_harvest_unit_price {
                        self.energy -= self.energy_harvest_unit_price;
                        self.energy_harvest_units += 1;
                        self.energy_harvest_unit_price = 2u32.pow(self.energy_harvest_units)
                    }
                }
                Resource::Science => {
                    if self.science >= self.science_harvest_unit_price {
                        self.science -= self.science_harvest_unit_price;
                        self.science_harvest_units += 1;
                        self.science_harvest_unit_price = 2u32.pow(self.science_harvest_units)
                    }
                }
            },
            Message::DecrementPressed(resource) => match resource {
                Resource::Ore => {
                    if self.ore_harvest_units > 1 {
                        self.ore_harvest_units -= 1;
                        self.ore_harvest_unit_price = 2u32.pow(self.ore_harvest_units)
                    }
                }
                Resource::Crystal => {
                    if self.crystal_harvest_units > 0 {
                        self.crystal_harvest_units -= 1;
                        self.crystal_harvest_unit_price = 2u32.pow(self.crystal_harvest_units)
                    }
                }
                Resource::Energy => {
                    if self.energy_harvest_units > 0 {
                        self.energy_harvest_units -= 1;
                        self.energy_harvest_unit_price = 2u32.pow(self.energy_harvest_units)
                    }
                }
                Resource::Science => {
                    if self.science_harvest_units > 0 {
                        self.science_harvest_units -= 1;
                        self.science_harvest_unit_price = 2u32.pow(self.science_harvest_units)
                    }
                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_secs(1)).map(|instant| Message::Tick(instant))
    }

    fn view(&mut self) -> Element<Message> {
        let Talos {
            scroll,
            ore_harvest_units_incr_button,
            ore_harvest_units_decr_button,
            crystal_harvest_units_incr_button,
            crystal_harvest_units_decr_button,
            energy_harvest_units_incr_button,
            energy_harvest_units_decr_button,
            science_harvest_units_incr_button,
            science_harvest_units_decr_button,
            ore_harvest_unit_price,
            crystal_harvest_unit_price,
            energy_harvest_unit_price,
            science_harvest_unit_price,
            ..
        } = self;

        let resources = Row::new()
            .width(Length::Fill)
            .spacing(20)
            .push(Text::new("Ore:").size(48))
            .push(Text::new(self.ore.to_string()).size(48))
            .push(Text::new("Crystal:").size(48))
            .push(Text::new(self.crystal.to_string()).size(48))
            .push(Text::new("Energy:").size(48))
            .push(Text::new(self.energy.to_string()).size(48))
            .push(Text::new("Science:").size(48))
            .push(Text::new(self.science.to_string()).size(48));

        let resource_gathering = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(Text::new(format!("Ore: {}/s", self.ore_harvest_units)).size(48))
            .push(Text::new(format!("Crystel: {}/s", self.crystal_harvest_units)).size(48))
            .push(Text::new(format!("Energy: {}/s", self.energy_harvest_units)).size(48))
            .push(Text::new(format!("Science: {}/s", self.science_harvest_units)).size(48));

        let resource_row = |price, incr_state, decr_state, resource: Resource| {
            Row::new()
                .width(Length::Fill)
                .push(Text::new(format!("Price: {}", price)).size(48))
                .push(
                    Button::new(
                        decr_state,
                        Text::new("-")
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .size(16),
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .on_press(Message::DecrementPressed(resource)),
                )
                .push(
                    Button::new(
                        incr_state,
                        Text::new("+")
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .size(16),
                    )
                    .width(Length::Fill)
                    .padding(8)
                    .on_press(Message::IncrementPressed(resource)),
                )
        };

        let resource_gathering_mod = Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(resource_row(
                ore_harvest_unit_price,
                ore_harvest_units_incr_button,
                ore_harvest_units_decr_button,
                Resource::Ore,
            ))
            .push(resource_row(
                crystal_harvest_unit_price,
                crystal_harvest_units_incr_button,
                crystal_harvest_units_decr_button,
                Resource::Crystal,
            ))
            .push(resource_row(
                energy_harvest_unit_price,
                energy_harvest_units_incr_button,
                energy_harvest_units_decr_button,
                Resource::Energy,
            ))
            .push(resource_row(
                science_harvest_unit_price,
                science_harvest_units_incr_button,
                science_harvest_units_decr_button,
                Resource::Science,
            ));

        let columns = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(resource_gathering)
            .push(resource_gathering_mod);

        let scrollable_content = Scrollable::new(scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(resources)
            .push(columns);

        Container::new(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }
}
