use std::collections::HashMap;
use std::io;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn request_user_rest_command(command_map: &HashMap<String, PowerStates>) -> &PowerStates {
    println!("\nWill you like to rest your computer?");
    println!(
        "Enter any of the following commands: \"off\", \"sleep\", \"reboot\", \"shutdown\", \"hibernate\""
    );
    let user_input = read_user_input();
    if let Some(command) = command_map.get(&user_input) {
        command
    } else {
        println!("Sorry, I don't recognize that command 🤨");
        request_user_rest_command(command_map)
    }
}

fn reboot_computer() {
    println!("\nOkie dokie!🫡 Rebooting your computer now....");
    println!("Stopping all running applications...");
    sleep(Duration::from_secs(3));
    println!("Saving all your work and shutting down to restart...");
    sleep(Duration::from_secs(5));
    println!("Computer has restarted!");
}

fn shut_down_messages() {
    println!("\nOkie dokie!🫡 Shutting down your computer now....");
    println!("Stopping all running applications...");
    sleep(Duration::from_secs(3));
    println!("Saving all your work and shutting down...");
    sleep(Duration::from_secs(2));
    println!("Computer is shut down. Goodbye!");
}

fn off_computer() -> bool {
    println!(
        "\nTo resume or shut it down, please enter: \"1\" to resume or \"2\" to shutdown completely."
    );
    let user_choice = read_user_input();
    match user_choice.as_str() {
        "1" => {
            println!("Resuming computer...");
            sleep(Duration::from_secs(3));
            return true;
        }
        "2" => {
            shut_down_messages();
            return false;
        }
        _ => {
            println!("Invalid choice. Please try again.");
            off_computer()
        }
    }
}

fn sleep_computer() {
    println!("\nOkie dokie!🫡 Putting computer to sleep...");
    sleep(Duration::from_secs(3));
    println!("Sleeping 😴...");
    println!("Computer is now in sleep mode. Press any key to wake it up.");
    read_user_input();
}

fn hibernate_computer() {
    println!("\nOkie dokie!🫡 Putting computer into hibernate mode...");
    println!("Saving all your work and hibernating computer...");
    sleep(Duration::from_secs(5));
    println!("Going into a deep sleep in...");
    println!("3...");
    sleep(Duration::from_secs(1));
    println!("2...");
    sleep(Duration::from_secs(1));
    println!("1...\n");
    println!("💤💤💤💤💤💤💤💤💤💤");
    sleep(Duration::from_secs(10));
    println!("Computer is now in hibernate mode. Press any key to wake it up.");
    read_user_input();
}

fn main() {
    let command_mapping = HashMap::from([
        ("off".to_string(), PowerStates::Off),
        ("sleep".to_string(), PowerStates::Sleep),
        ("reboot".to_string(), PowerStates::Reboot),
        ("shutdown".to_string(), PowerStates::Shutdown),
        ("hibernate".to_string(), PowerStates::Hibernate),
    ]);

    println!("Computer is starting up! Welcome user!");
    println!("Please enter your name:");

    let username = read_user_input();

    loop {
        println!("\nWelcome {username}! Glad your back!");

        println!("««Resuming machine task! This will take 10 seconds»»");
        println!("Processing................................................");
        sleep(Duration::from_secs(10));

        println!("Machine task completed successfully!");

        let user_command = request_user_rest_command(&command_mapping);
        match user_command {
            PowerStates::Off => {
                println!("Okie dokie!🫡 Turning off your computer now....");
                println!("Screen and system is going to off mode in 3 seconds...");
                sleep(Duration::from_secs(3));
                println!("Computer is off!");
                let can_resume = off_computer();
                if !can_resume {
                    break;
                }
            }
            PowerStates::Sleep => sleep_computer(),
            PowerStates::Reboot => reboot_computer(),
            PowerStates::Shutdown => {
                shut_down_messages();
                break;
            }
            PowerStates::Hibernate => hibernate_computer(),
        }
    }
}
