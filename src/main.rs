#![windows_subsystem = "windows"]
use iced::mouse::Button;
use iced::widget::button::{Appearance, StyleSheet};
// use iced::widget::container::StyleSheet;
// #[derive(Debug, Clone, Copy)]
use iced::window::icon::Icon;
use iced::window::Action;
use iced::window::{change_icon, Id};
use iced::{command, Command};
use iced::{
    widget::{button, column, row, text, Container, Row, Text},
    Length, Pixels, Sandbox, Settings, Theme,
};
// pub enum Message {
//     IncrementPressed,
//     DecrementPressed,
// }

// struct Counter {
//     // The counter value
//     value: i32,
// }

// struct ButtonColor {
//     color: iced::Color,
// }

// impl button::StyleSheet for ButtonColor {
//     fn active(&self) -> button::Style {
//         button::Style {
//             background: Some(iced::Background::Color(self.color)),
//             ..Default::default()
//         }
//     }

//     type Style = ButtonColor;

//     fn hovered(&self, style: &Self::Style) -> button::Appearance {
//         let active = self.active(style);

//         button::Appearance {
//             shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
//             ..active
//         }
//     }

//     fn pressed(&self, style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             shadow_offset: iced::Vector::default(),
//             ..self.active(style)
//         }
//     }

//     fn disabled(&self, style: &Self::Style) -> button::Appearance {
//         let active = self.active(style);

//         button::Appearance {
//             shadow_offset: iced::Vector::default(),
//             background: active.background.map(|background| match background {
//                 iced::Background::Color(color) => iced::Background::Color(iced::Color {
//                     a: color.a * 0.5,
//                     ..color
//                 }),
//                 iced::Background::Gradient(gradient) => {
//                     iced::Background::Gradient(gradient.mul_alpha(0.5))
//                 }
//             }),
//             text_color: iced::Color {
//                 a: active.text_color.a * 0.5,
//                 ..active.text_color
//             },
//             ..active
//         }
//     }
//     // other methods in Stylesheet have a default impl
// }

#[derive(Debug, Clone, Copy)]
enum Message {
    add,
    remove,
}
struct App {
    counter: i32,
}

// struct ButtonColor {
//     color: iced::Color,
// }

// impl StyleSheet for ButtonColor {
//     fn active(&self) -> button::Style {
//         button::Style {
//             background: Some(iced::Background::Color(self.color)),
//             ..Default::default()
//         }
//     }

//     type Style = Button;
//     // other methods in Stylesheet have a default impl
// }

impl Sandbox for App {
    type Message = Message;

    // fn style(&self) -> iced::theme::Application {
    //     iced::theme::Application::custom()
    // }

    fn new() -> Self {
        App { counter: 0 }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        // change_icon::<Message>(
        //     Id::unique(),
        //     iced::window::icon::from_rgba(
        //         std::fs::read("C:\\Users\\103925pafr\\test\\iced_test\\assets\\Microsoft-Fluentui-Emoji-Flat-Test-Tube-Flat - Kopie.ico")
        //             .unwrap(),
        //         16,
        //         16,
        //     )
        //     .unwrap(),
        // );

        // column!(iced::Element::new(text("xxx")))
        let minus_button =
            button(text("remove").horizontal_alignment(iced::alignment::Horizontal::Center))
                .width(80)
                .on_press(Message::remove)
                .style(iced::theme::Button::Destructive);
        let plus_button =
            button(text("add").horizontal_alignment(iced::alignment::Horizontal::Center))
                .width(80)
                .on_press(Message::add)
                .style(iced::theme::Button::Positive);

        let counter_text = text(self.counter);

        let column = row(vec![
            minus_button.into(),
            counter_text.into(),
            plus_button.into(),
        ])
        // .height(Length::Fill)
        // .width(Length::Fill)
        // .align_items(iced::Alignment::Center)
        .spacing(50);

        // let container = Container::new(
        //     vec![minus_button.into(), counter_text.into(), plus_button.into()].into(),
        // )
        // .center_x()
        // .center_y()
        // .width(Length::Fill)
        // .height(Length::Fill);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

        // let container = Container::new(column)
        //     .center_x()
        //     .width(1000)
        //     .align_x(iced::alignment::Horizontal::Center);
        // container.into()

        // let row = Row::new()
        //     .push(column)
        //     .height(Length::Fill)
        //     .width(Length::Fill)
        //     .align_items(iced::alignment::Horizontal::Center.into());
        // // column.align_items(iced::Alignment::Center);
        // row.into()
        // column(vec![
        //     button::<Message::Remove>("Remove"),
        //     text(self.counter),
        //     button("Add"),
        // ])
        // .spacing(20)

        // iced::Element::from(vec![button("Remove"), text(self.counter), button("Add")])
        // iced::Element::new(text("xxx"))
    }

    fn title(&self) -> String {
        String::from("xxx")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::add => self.counter += 1,
            Message::remove => self.counter -= 1,
        }
    }
    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}
// impl SandBox for App {
// type Message = Message;

// fn view {
//     text("xxx")
// }

// fn title() -> String {
//     String::from("xxx")
// }

// fn new () -> Self {
//     App{}
// }

// }

fn set_icon<T>(settings: &mut Settings<T>)  {
    if let Ok(icon_file) =  std::fs::File::open("C:\\Users\\103925pafr\\test\\iced_test\\assets\\Microsoft-Fluentui-Emoji-Flat-Test-Tube-Flat - Kopie.ico"){
        if let Ok(icon_dir) = ico::IconDir::read(icon_file){
            if let Ok(image) = icon_dir.entries()[0].decode(){
                if let Ok(icon) = iced::window::icon::from_rgba(image.rgba_data().to_vec(), 512, 512) {
                    settings.window.icon = Some(icon);
                } 
            }
        }
    }

    // let icon_dir = ico::IconDir::read(file).unwrap();

    // let image = icon_dir.entries()[0].decode()
}

fn main() -> Result<(), iced::Error> {
    // iced::command::Action::Window::Command::single(command::Command(Action::ChangeIcon(id, icon)));
    // iced::window::icon::s

    let mut settings: Settings<()> = Settings::default();

    set_icon(&mut settings);

    App::run(settings)?;

    Ok(())
    // Counter.
}

// use iced::widget::{button, column, text, Column};

// impl Counter {
//     pub fn view(&self) -> Column<Message> {
//         // We use a column: a simple vertical layout
//         column![
//             // The increment button. We tell it to produce an
//             // `IncrementPressed` message when pressed
//             button("+").on_press(Message::IncrementPressed),
//             // We show the value of the counter here
//             text(self.value).size(50),
//             // The decrement button. We tell it to produce a
//             // `DecrementPressed` message when pressed
//             button("-").on_press(Message::DecrementPressed),
//         ]
//     }
// }

// impl Counter {
//     // ...

//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::IncrementPressed => {
//                 self.value += 1;
//             }
//             Message::DecrementPressed => {
//                 self.value -= 1;
//             }
//         }
//     }
// }
