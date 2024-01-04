use crate::core::messages::standard_messages;
use crate::core::report::logger;
use crate::core::structs::{Logger, ProcessInit};
use crate::core::utils::*;
use crate::modules::osint::osint_structs::*;
use crate::read_meow;

pub fn sample(data: SampleData, debug: bool) -> bool {
    let instance = ProcessInit {
        source: data.data,
        source_from: "sample_module",
        source_description: "Set sample rules",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn open_street_map_gen(term: OsintLocationOSM, debug: bool) -> bool {
    let link = format!(
        "https://www.openstreetmap.org/search?query={}#map={}/{}/{}",
        term.query, term.zoom, term.long, term.lati,
    );

    println!("\n💻 Link: {}\n", &link);

    let firefox_link = format!("firefox {}", link);

    let instance = ProcessInit {
        source: firefox_link.as_str(),
        source_from: "open_street_map_gen",
        source_description: "Create custom links to OpenStreetMap",
        debug: debug,
    };

    system_command_exec(instance)
}

pub fn city_geo_location(query: CityGeoLocation, debug: bool) -> bool {
    let path_location = read_meow("/var/witch_craft/witch_spells/embedded/config.meow", false);
    let paths: Vec<String> = vec![
        format!(
            "{}{}",
            path_location["PRIVATE_MODULES"], path_location["GEODATA_CITY_CODES"]
        ),
        format!(
            "{}{}",
            path_location["PRIVATE_MODULES"], path_location["GEODATA_COUNTRY_CODES"]
        ),
        format!(
            "{}{}",
            path_location["PRIVATE_MODULES"], path_location["GEODATA_WORLDCITIES"]
        ),
    ];

    for file in paths {
        match find_all_matching_lines(&file, query.city) {

            Ok(result) => {
            
                if !result.is_empty() {
            
                    for line in result {
            
                        let data = Logger {
                            source: "city_geo_location".to_string(),
                            source_from: "osint".to_string(),
                            source_description: "Look up geographic information on a city"
                                .to_string(),
                            status: 0.to_string(),
                            stdout: format!("{:?}", line),
                            stderr: "none".to_string(),
                            debug: debug,
                        };

                        match logger(data) {

                            Ok(_result) => {
                                // standard_messages("saved", "Log saved", "", "cute");
                            }
                            Err(_err) => println!("error:src.modules.osint.city_geo_location"),
                        }
                        standard_messages("flagged", "Found", &line, "cute");
                    }

                    return true;
                } else {
                    standard_messages("warning", "Pattern not found in any line.", "", "cute");
                    return false;
                }
            }
            Err(err) => {
                let message = format!("{}", err);
                standard_messages(
                    "error",
                    "Error while looking for city information",
                    &message,
                    "cute",
                );
                return false;
            }
        }
    }

    return false;
}

pub fn ip_geo_location(ip: IpGeoLocation, debug: bool) -> bool {
    // ip filter
    let mut ip_formatted = String::new();
    for symbol in ip.ip_string.chars() {
        if symbol == '.' {
            ip_formatted = format!("{}{}", ip_formatted, "");
        } else if symbol == ':' {
            ip_formatted = format!("{}{}", ip_formatted, "");
        } else {
            ip_formatted = format!("{}{}", ip_formatted, symbol);
        }
    }

    //convert ip to integer
    return true;
}

pub fn shell_osint(system_input: &mut Vec<String>) -> bool {
    let cmd_arg_name = system_input[2].as_str();

    match cmd_arg_name {
        "--link_map" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = OsintLocationOSM {
                path: &take_system_args(system_input, "--path"),
                query: &take_system_args(system_input, "--query"),
                zoom: &take_system_args(system_input, "--zoom"),
                long: &take_system_args(system_input, "--long"),
                lati: &take_system_args(system_input, "--lati"),
            };
            open_street_map_gen(instance, debug)
        }

        "--city_geo" => {
            let debug = take_system_args_debug(take_system_args(system_input, "--debug"));
            let instance = CityGeoLocation {
                city: &take_system_args(system_input, "--city"),
            };

            city_geo_location(instance, debug)
        }

        _ => {
            standard_messages("warning", "Invalid user input", "shell_lookup", "cute");
            standard_messages("warning", "Trying exec command", cmd_arg_name, "cute");
            return false;
        }
    }
}
