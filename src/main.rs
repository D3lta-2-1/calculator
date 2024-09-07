  #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fmt::{Display, Formatter};
use eframe::egui;
use eframe::egui::Button;

#[derive(PartialEq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    None,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Mul => write!(f, "*"),
            Operation::Div => write!(f, "/"),
            Operation::None => write!(f, ">"),
        }
    }
}


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).


    let viewport = egui::ViewportBuilder {
        title: Some("calculator".to_string()),
        inner_size: Some(egui::vec2(200.0, 250.0)),
        resizable: Some(false),
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        viewport,
        vsync: true,
        ..Default::default()
    };


    let mut number = None;
    let mut last_result = None;
    let mut operation = Operation::None;


    let add_digit_to_number = |number: &mut Option<i64>, digit: i64| {
        *number = Some(number.unwrap_or(0) * 10 + digit);
    };

    let done = |number : &mut Option<i64>, last_result : &mut Option<i64>, operation : &mut Operation|{
        if number.is_none() { return; }
        match operation {
            Operation::Add => *last_result = Some(last_result.unwrap() + number.unwrap()),
            Operation::Sub => *last_result = Some(last_result.unwrap() - number.unwrap()),
            Operation::Mul => *last_result = Some(last_result.unwrap() * number.unwrap()),
            Operation::Div => if number.unwrap() != 0 { *last_result = Some(last_result.unwrap() / number.unwrap()); }

            Operation::None => (),
        }
        *number = None;
        *operation = Operation::None;
    };


    eframe::run_simple_native("calculator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            let create_button = |name: &str| Button::new(name).min_size(egui::vec2(40.0, 40.0));

            ui.heading(if last_result.is_some() {
                format!("{}", last_result.unwrap())
            } else {
                "".to_string()
            });
            ui.heading(format!("{operation} {}", if number.is_some() {
                format!("{}", number.unwrap())
            } else {
                "".to_string()
            }));
            let but0 = create_button("0");
            let but1 = create_button("1");
            let but2 = create_button("2");
            let but3 = create_button("3");
            let but4 = create_button("4");
            let but5 = create_button("5");
            let but6 = create_button("6");
            let but7 = create_button("7");
            let but8 = create_button("8");
            let but9 = create_button("9");


            ui.horizontal(|ui|{
                if ui.add(create_button("+")).clicked() {
                    if number.is_none() {
                        if operation != Operation::None { operation = Operation::Add; }
                        return;
                    }

                    if last_result.is_some() && operation != Operation::None { done(&mut number, &mut last_result, &mut operation); }
                    else { last_result = number; }

                    operation = Operation::Add;
                    number = None;
                }

                if ui.add(but7).clicked() { add_digit_to_number(&mut number,7); }
                if ui.add(but8).clicked() { add_digit_to_number(&mut number, 8); }
                if ui.add(but9).clicked() { add_digit_to_number(&mut number,9); }
            });

            ui.horizontal(|ui|{
                if ui.add(create_button("-")).clicked() {
                    if number.is_none() {
                        if operation != Operation::None { operation = Operation::Sub; }
                        return;
                    }

                    if last_result.is_some() && operation != Operation::None { done(&mut number, &mut last_result, &mut operation); }
                    else { last_result = number; }

                    operation = Operation::Sub;
                    number = None;
                }


                if ui.add(but4).clicked() { add_digit_to_number(&mut number,4); }
                if ui.add(but5).clicked() { add_digit_to_number(&mut number,5); }
                if ui.add(but6).clicked() { add_digit_to_number(&mut number,6); }
            });

            ui.horizontal(|ui|{
                if ui.add(create_button("*")).clicked() {
                    if number.is_none() {
                        if operation != Operation::None { operation = Operation::Mul; }
                        return;
                    }

                    if last_result.is_some() && operation != Operation::None { done(&mut number, &mut last_result, &mut operation); }
                    else { last_result = number; }

                    operation = Operation::Mul;
                    number = None;
                }

                if ui.add(but1).clicked() { add_digit_to_number(&mut number,1); }
                if ui.add(but2).clicked() { add_digit_to_number(&mut number,2); }
                if ui.add(but3).clicked() { add_digit_to_number(&mut number,3); }
            });

            ui.horizontal(|ui|{
                if ui.add(create_button("/")).clicked() {
                    if number.is_none() {
                        if operation != Operation::None { operation = Operation::Div; }
                        return;
                    }

                    if last_result.is_some() && operation != Operation::None { done(&mut number, &mut last_result, &mut operation); }
                    else { last_result = number; }

                    operation = Operation::Div;
                    number = None;
                }


                if ui.add(but0).clicked() { add_digit_to_number(&mut number,0); }

                if ui.add(create_button("=")).clicked() { done(&mut number, &mut last_result, &mut operation) }

                if ui.add(create_button("CE")).clicked() {
                    operation = Operation::None;
                    last_result = None;
                    number = None;
                }
            });

        });
    })
}
