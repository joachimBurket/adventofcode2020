use std::fs;
use regex::Regex;
use day4::{PassportField, PassportsParser, PassportsValidator};


fn main() {
    const PASSPORTS_FILENAME: &str = "resources/passports";

    let passport_fields = get_passport_fields();

    let input = fs::read_to_string(PASSPORTS_FILENAME).unwrap();
    let passports = PassportsParser::parse(&input);

    let validator = PassportsValidator::new(passports, passport_fields);
    
    let passports_valid = validator.validate_passports();
    println!("There were {} passports valid", passports_valid);
}

fn get_passport_fields() -> Vec<PassportField> {
    let fields = vec![
        PassportField{
            contraction: String::from("byr"), full_name: String::from("Birth Year"), required: true,
            validator: |val: &str| {
                let number = val.parse::<i32>().unwrap_or(-1);
                number >= 1920 && number <= 2002
            }
        },
        PassportField{
            contraction: String::from("iyr"), full_name: String::from("Issue Year"), required: true,
            validator: |val: &str| {
                let number = val.parse::<i32>().unwrap_or(-1);
                number >= 2010 && number <= 2020
            }
        },
        PassportField{
            contraction: String::from("eyr"), full_name: String::from("Expiration Year"), required: true,
            validator: |val: &str| {
                let number = val.parse::<i32>().unwrap_or(-1);
                number >= 2020 && number <= 2030
            }
        },
        PassportField{
            contraction: String::from("hgt"), full_name: String::from("Height"), required: true,
            validator: |val: &str| {
                if val.ends_with("cm") {
                    let number = val[0..val.len()-2].parse::<i32>().unwrap_or(-1);
                    return number >= 150 && number <= 193;
                }
                else if val.ends_with("in") {
                    let number = val[0..val.len()-2].parse::<i32>().unwrap_or(-1);
                    return number >= 59 && number <= 76;
                }
                false
            }
        },
        PassportField{
            contraction: String::from("hcl"), full_name: String::from("Hair Color"), required: true,
            validator: |val: &str| {
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                re.is_match(val)
            }
        },
        PassportField{
            contraction: String::from("ecl"), full_name: String::from("Eye Color"), required: true,
            validator: |val: &str| {
                let colors = vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth");
                colors.contains(&val)
            }
        },
        PassportField{
            contraction: String::from("pid"), full_name: String::from("Passport ID"), required: true,
            validator: |val: &str| {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                re.is_match(val)
            }
        },
        PassportField{
            contraction: String::from("cid"), full_name: String::from("Country ID"), required: false,
            validator: |_val: &str| {
                true    // cid is not checked
            }
        },
    ];
    fields
}