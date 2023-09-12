use crate::core::report::*;
use crate::core::structs::{CommandCall, CommandResult};
use chrono;
use colored::*;
use std::env;
use std::io::Error;
use std::process::{Command, Output};

pub fn system_command_call(cmd: CommandCall) -> Result<CommandResult, Error> {
    let output: Output = Command::new(cmd.command).args(cmd.args).output()?;

    let status_code = output.status.code().unwrap_or(-1).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let result = CommandResult {
        status: status_code,
        stdout: stdout,
        stderr: stderr,
    };

    Ok(result)
}

pub fn system_string_to_vec_builder(build_string: String) -> Result<Vec<String>, Error> {
    let sys_args: Vec<String> = build_string.split_whitespace().map(String::from).collect();
    Ok(sys_args)
}

pub fn system_command_exec(command_str: &str, debug: bool) -> bool {
    let command_string = command_str.to_string();

    match system_string_to_vec_builder(command_string.clone()) {
        Ok(res_command_call) => {
            let command_call: CommandCall = CommandCall {
                command: res_command_call[0].as_str(),
                args: &res_command_call[1..],
            };

            match system_command_call(command_call) {
                Ok(res) => {
                    if debug == true {
                        let (x, y, z) = (res.status, res.stdout, res.stderr);
                        println!("{}, {}, {}", x, y, z);

                        match write_report(command_string.clone(), x, y, z, debug) {
                            Ok(()) => {
                                system_text("[REPORT] :: Report created", "green");
                            }

                            Err(e) => {
                                eprintln!("[REPORT_ERROR] :: While executing command at → system_command_call: {}", e );
                            }
                        }

                        return true;
                    }
                    println!("{}", res.stdout);
                    return true;
                }
                Err(e) => {
                    eprintln!(
                        "[ERROR] :: While executing command at → system_command_call: {}",
                        e
                    );
                }
            }
        }
        Err(e) => eprintln!(
            "[ERROR] :: While executing command at → system_string_to_vec_builder: {}",
            e
        ),
    }

    return false;
}

pub fn system_command_deep_exec(command_str: &str, _debug: bool) -> Result<CommandResult, Error> {
    let command_string = command_str.to_string();
    let mut result = CommandResult {
        status: "none".to_string(),
        stdout: "none".to_string(),
        stderr: "none".to_string(),
    };

    match system_string_to_vec_builder(command_string) {
        Ok(res_command_call) => {
            let command_call: CommandCall = CommandCall {
                command: res_command_call[0].as_str(),
                args: &res_command_call[1..],
            };

            match system_command_call(command_call) {
                Ok(res) => result = res,
                Err(e) => eprintln!(
                    "[ERROR] :: While executing command at → system_command_call: {}",
                    e
                ),
            }
        }
        Err(e) => eprintln!(
            "[ERROR] :: While executing command at → system_string_to_vec_builder: {}",
            e
        ),
    }

    Ok(result)
}

pub fn system_text(text: &str, color: &str) -> bool {
    let mut lines = text.lines();
    loop {
        match lines.next() {
            Some(line) => {
                if color == "black" {
                    println!("{}", line.black())
                } else if color == "red" {
                    println!("{}", line.red())
                } else if color == "green" {
                    println!("{}", line.green())
                } else if color == "yellow" {
                    println!("{}", line.yellow())
                } else if color == "blue" {
                    println!("{}", line.blue())
                } else if color == "purple" {
                    println!("{}", line.purple())
                } else if color == "cyan" {
                    println!("{}", line.cyan())
                } else {
                    println!("{}", line.white())
                }
            }
            None => break,
        }
    }
    println!("");
    return true;
}

pub fn system_time() -> String {
    format!("{:?}", chrono::offset::Local::now())
}

pub fn system_exec_shell(debug: bool) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if debug == true {
        println!("{:?}", args)
    }
    return args;
}

pub fn gsv(data: Vec<String>, parameter_name: &str) -> String {
    if data.len() <= 2 {
        return "[ERROR] :: Invalid vector size at → gsv".to_string();
    }

    let mut count = 2;
    while count < data.len() {
        if data[count] == parameter_name {
            return data[count + 1].to_string();
        }
        count += 1;
    }

    return format!(
        "[NOT_FOUND] :: Parameter name not found at → P: {}, Vec: {:?}",
        parameter_name, data
    );
}

pub fn gsv_debug(debug: String) -> bool {
    if debug == "true" {
        return true;
    } else {
        return false;
    }
}