use std::collections::HashMap;


pub struct PassportField {
    pub contraction: String,
    pub full_name: String,
    pub required: bool,
    pub validator: fn(&str) -> bool,
}

impl PassportField {
    pub fn validate(&self, value: &str) -> bool {
        (self.validator)(value)
    }
}


pub struct Passport {
    pub fields: HashMap<String, String>
}

impl Passport {
    fn new(passport_str:&str) -> Self {
        let mut fields: HashMap<String, String> = HashMap::new();
        let mut passport_string = String::from(passport_str);

        passport_string = passport_string
            .trim()
            .replace("\n", " ");
        let fields_str: Vec<&str> = passport_string.split(" ").collect();
        
        for field_str in fields_str {
            let chars: Vec<&str> = field_str.split(":").collect();
            fields.insert(String::from(chars[0]), String::from(chars[1]));
        }

        Self {
            fields,
        }
    }
}


pub struct PassportsParser {}

impl PassportsParser {
    /// Parses a string of passports separated by an empty line.
    pub fn parse(input: &str) -> Vec<Passport> {
        let mut passports: Vec<Passport> = Vec::new();
        let mut passport_str: String = String::new();

        for line in input.lines() {
            // Empty line -> new passport
            if line.is_empty() {
                if !passport_str.is_empty() {
                    passports.push(Passport::new(&passport_str));
                }

                passport_str = String::new();
                continue;
            }

            passport_str += " ";
            passport_str += line;
        }
        if !passport_str.is_empty() {
            passports.push(Passport::new(&passport_str));
        }
        passports
    }
}


pub struct PassportsValidator {
    passport_fields: Vec<PassportField>,
    passports: Vec<Passport>,
}

impl PassportsValidator {
    pub fn new(passports: Vec<Passport>, passport_fields: Vec<PassportField>) -> Self {
        Self {
            passport_fields,
            passports,
        }
    }

    pub fn validate_passports(&self) -> u32 {
        let mut valids: u32 = 0;

        for passport in &self.passports {
            if self.validate_passport(passport) {
                valids += 1;
            }
        }

        valids
    }

    fn validate_passport(&self, passport: &Passport) -> bool {
        for field in &self.passport_fields {
            if field.required  {
                if !passport.fields.contains_key(&field.contraction) || 
                    !field.validate(&passport.fields[&field.contraction]) {
                    return false;
                }
            }
        }
        return true;
    }
}
