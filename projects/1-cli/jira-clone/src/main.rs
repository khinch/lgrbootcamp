mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

use db::*;
use io_utils::*;
use models::Action;
use navigator::*;
use std::rc::Rc;

fn main() {
    let database = JiraDatabase::new(String::from("data/db.json"));
    let mut navigator = Navigator::new(Rc::new(database));

    loop {
        clearscreen::clear().unwrap();

        if let Some(current_page) = navigator.get_current_page() {
            if let Err(error) = current_page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
                navigator.handle_action(Action::NavigateToPreviousPage);
                continue;
            }

            let user_choice = get_user_input();

            match current_page.handle_input(&user_choice) {
                Ok(action) => match action {
                    Some(action) => {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error performing action: {}\nPress any key to continue...",
                                error
                            );
                            wait_for_key_press();
                            continue;
                        }
                    }
                    None => continue,
                },
                Err(error) => {
                    println!(
                        "Error handling input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                    continue;
                }
            }
        } else {
            break;
        }
    }
}
