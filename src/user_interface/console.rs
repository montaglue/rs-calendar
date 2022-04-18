use chrono::Utc;
use repl_rs::{Command, Parameter, Result, Value};
use repl_rs::{Convert, Repl};
use std::collections::HashMap;

use crate::data::Data;
use crate::prediction;
use crate::world::Time;

use super::UI;

fn new_task(args: HashMap<String, Value>, context: &mut Data) -> Result<Option<String>> {
    let name = args["name"].convert()?;
    let description = args["description"].convert()?;
    let _date: String = args["date"].convert()?;
    let _stage: String = args["stage"].convert()?;

    context.create_task(name, description);

    Ok(None)
}

fn change_task(args: HashMap<String, Value>, context: &mut Data) -> Result<Option<String>> {
    let id: usize = args["id"].convert()?;
    let name: String = args["name"].convert()?;
    let description: String = args["description"].convert()?;
    let _stage: String = args["stage"].convert()?;


    context.change_task(id, move |task| {
        task.name = name;
        task.description = description;
    });

    Ok(None)
}

fn remove_task(args: HashMap<String, Value>, context: &mut Data) -> Result<Option<String>> {
    let id: usize = args["id"].convert()?;

    context.remove_task(id);

    Ok(None)
}

fn change_flow(args: HashMap<String, Value>, context: &mut Data) -> Result<Option<String>> {
    let flow: String = args["flow"].convert()?;

    context.change_flow(flow);

    Ok(None)
}

fn predict(_: HashMap<String, Value>, context: &mut Data) -> Result<Option<String>> {
    Ok(if let Some(wm) = prediction::predict(context, Time::from(Utc::now())) {
        Some(format!("{:?}", wm))
    } else {
        None
    })
}

fn exit(_: HashMap<String, Value>, _: &mut Data) -> Result<Option<String>> {
    panic!()
}

#[derive(Default)]
pub struct ConsoleUI {}

impl ConsoleUI {
    fn load_data(&mut self) {
        // start every time with empty data, yet [todo!]
    }
}

impl UI for ConsoleUI {
    fn run(mut self: Box<Self>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        self.load_data();

        let mut repl = Repl::new(Data::default())
            .use_completion(true)
            .add_command(
                Command::new("task", new_task)
                    .with_parameter(Parameter::new("name").set_required(true)?)?
                    .with_parameter(Parameter::new("description").set_required(true)?)?
                    .with_parameter(Parameter::new("time").set_required(true)?)?
                    .with_parameter(Parameter::new("stage").set_required(true)?)?
            )
            .add_command(
                Command::new("flow", change_flow)
                    .with_parameter(Parameter::new("name").set_required(true)?)?
            )
            .add_command(
                Command::new("predict", predict)
                    .with_parameter(Parameter::new("time").set_required(true)?)?
            )
            .add_command(
                Command::new("remove", remove_task)
                    .with_parameter(Parameter::new("name").set_required(true)?)?
            )
            .add_command(
                Command::new("change", change_task)
                    .with_parameter(Parameter::new("name").set_required(true)?)?
                    .with_parameter(Parameter::new("description").set_required(true)?)?
                    .with_parameter(Parameter::new("time").set_required(true)?)?
                    .with_parameter(Parameter::new("stage").set_required(true)?)?
            )
            .add_command(
                Command::new("exit", exit)
            );
        
        repl.run()?;

        Ok(())
    }
}
