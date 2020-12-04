use std::{ops::Add, str::FromStr};
use util::read_input;

#[derive(Debug, Default, PartialEq)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_empty(&self) -> bool {
        self.byr.is_none()
            && self.iyr.is_none()
            && self.eyr.is_none()
            && self.hgt.is_none()
            && self.hcl.is_none()
            && self.ecl.is_none()
            && self.pid.is_none()
            && self.cid.is_none()
    }

    fn is_valid(&self) -> bool {
        match &self.byr {
            Some(byr) => {
                let byr_int = byr.parse::<usize>().unwrap();
                if byr.len() != 4 || byr_int < 1920 || byr_int > 2002 {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match &self.iyr {
            Some(iyr) => {
                let iyr_int = iyr.parse::<usize>().unwrap();
                if iyr.len() != 4 || iyr_int < 2010 || iyr_int > 2020 {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match &self.eyr {
            Some(eyr) => {
                let eyr_int = eyr.parse::<usize>().unwrap();
                if eyr.len() != 4 || eyr_int < 2020 || eyr_int > 2030 {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match &self.hgt {
            Some(hgt) => {
                let val = hgt[0..(hgt.len() - 2)].parse::<usize>().unwrap();
                let unit = &hgt[(hgt.len() - 2)..hgt.len()];
                match unit {
                    "cm" => {
                        if val < 150 || val > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if val < 59 || val > 76 {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
        match &self.hcl {
            Some(hcl) => {
                let mut chars = hcl.chars();
                if chars.next().unwrap() != '#' {
                    return false;
                }
                let mut counter = 0;
                for c in chars {
                    counter += 1;
                    if !(c.is_ascii_hexdigit() && !c.is_ascii_uppercase()) {
                        return false;
                    }
                }
                if counter != 6 {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match &self.ecl {
            Some(ecl) => {
                if !["amb", "blu", "brn", "grn", "gry", "hzl", "oth"].contains(&ecl.as_str()) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match &self.pid {
            Some(pid) => {
                if pid.len() != 9 {
                    return false;
                }
                match pid.parse::<usize>() {
                    Ok(_) => {}
                    Err(_) => {
                        return false;
                    }
                }
            }
            None => {
                return false;
            }
        }
        true
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        for kv in s.split(' ') {
            if kv.is_empty() {
                continue;
            }
            let mut split = kv.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            match key {
                "byr" => passport.byr = Some(value.to_string()),
                "iyr" => passport.iyr = Some(value.to_string()),
                "eyr" => passport.eyr = Some(value.to_string()),
                "hgt" => passport.hgt = Some(value.to_string()),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.to_string()),
                _ => {
                    return Err("Invalid key.".to_string());
                }
            }
        }
        Ok(passport)
    }
}

impl Add for &Passport {
    type Output = Passport;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            byr: self.byr.clone().or(other.byr.clone()),
            iyr: self.iyr.clone().or(other.iyr.clone()),
            eyr: self.eyr.clone().or(other.eyr.clone()),
            hgt: self.hgt.clone().or(other.hgt.clone()),
            hcl: self.hcl.clone().or(other.hcl.clone()),
            ecl: self.ecl.clone().or(other.ecl.clone()),
            pid: self.pid.clone().or(other.pid.clone()),
            cid: self.cid.clone().or(other.cid.clone()),
        }
    }
}

fn combine_passports(input: &[Passport]) -> Vec<Passport> {
    let mut result = Vec::new();
    let mut combined = Passport::default();
    for passport in input {
        if passport.is_empty() {
            if !combined.is_empty() {
                result.push(combined);
                combined = Passport::default();
            }
        }
        combined = &combined + &passport;
    }
    // Add the last passport.
    if !combined.is_empty() {
        result.push(combined);
    }
    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Passport>(&args[1]).collect::<Vec<Passport>>();
    let combined_input = combine_passports(&input);

    println!(
        "{}",
        combined_input
            .iter()
            .filter(|passport| passport.is_valid())
            .count()
    );
}

#[cfg(test)]
mod tests {
    use crate::{combine_passports, Passport};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let empty = Passport::from_str("").unwrap();
        assert!(empty.byr.is_none());
        assert!(empty.iyr.is_none());
        assert!(empty.eyr.is_none());
        assert!(empty.hgt.is_none());
        assert!(empty.hcl.is_none());
        assert!(empty.ecl.is_none());
        assert!(empty.pid.is_none());
        assert!(empty.cid.is_none());

        let non_empty = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd").unwrap();
        assert!(non_empty.byr.is_none());
        assert!(non_empty.iyr.is_none());
        assert_eq!(non_empty.eyr, Some("2020".to_string()));
        assert!(non_empty.hgt.is_none());
        assert_eq!(non_empty.hcl, Some("#fffffd".to_string()));
        assert_eq!(non_empty.ecl, Some("gry".to_string()));
        assert_eq!(non_empty.pid, Some("860033327".to_string()));
        assert!(non_empty.cid.is_none());
    }

    #[test]
    fn add() {
        let a = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd").unwrap();
        let b = Passport::from_str("byr:1937 iyr:2017 cid:147 hgt:183cm").unwrap();

        let combined = &a + &b;

        assert_eq!(combined.byr, Some("1937".to_string()));
        assert_eq!(combined.iyr, Some("2017".to_string()));
        assert_eq!(combined.eyr, Some("2020".to_string()));
        assert_eq!(combined.hgt, Some("183cm".to_string()));
        assert_eq!(combined.hcl, Some("#fffffd".to_string()));
        assert_eq!(combined.ecl, Some("gry".to_string()));
        assert_eq!(combined.pid, Some("860033327".to_string()));
        assert_eq!(combined.cid, Some("147".to_string()));
    }

    #[test]
    fn is_empty() {
        let empty = Passport::from_str("").unwrap();

        assert!(empty.is_empty());
    }

    #[test]
    fn is_valid() {
        let a = Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd").unwrap();
        let b = Passport::from_str("byr:1937 iyr:2017 cid:147 hgt:183cm").unwrap();

        let combined = &a + &b;

        assert!(combined.is_valid());
    }

    #[test]
    fn combine() {
        let input = [
            Passport::from_str("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd").unwrap(),
            Passport::from_str("byr:1937 iyr:2017 cid:147 hgt:183cm").unwrap(),
            Passport::from_str("").unwrap(),
            Passport::from_str("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884").unwrap(),
            Passport::from_str("hcl:#cfa07d byr:1929").unwrap(),
        ];

        let combined_inputs = combine_passports(&input);

        assert_eq!(combined_inputs.len(), 2);
        assert_eq!(combined_inputs[0], &input[0] + &input[1]);
        assert_eq!(combined_inputs[1], &input[3] + &input[4]);
    }
}
