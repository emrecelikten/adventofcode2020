use std::collections::HashMap;
use std::convert::From;
use std::ops::RangeInclusive;

fn read_file(filename: &str) -> Vec<String> {
    let lines = std::fs::read_to_string(filename).unwrap();

    lines.split("\n\n").map(|e| e.replace('\n', " ")).collect()
}

#[derive(Debug)]
struct Passport<'a> {
    data: HashMap<&'a str, &'a str>,
}

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        self.data.keys().count() >= 7 && FIELDS.iter().all(|e| self.data.contains_key(e))
    }

    fn check_in_range(num: &str, range: RangeInclusive<u32>) -> bool {
        num.parse().map_or(false, |n| range.contains(&n))
    }

    fn validate_hgt(height: &str) -> bool {
        height.strip_suffix("cm").map_or(false, |s| Passport::check_in_range(s, 150..=193)) ||
            height.strip_suffix("in").map_or(false, |s| Passport::check_in_range(s, 59..=76))
    }

    fn validate_hcl(hcl: &str) -> bool {
        hcl.strip_prefix("#").map_or(false, |e| e.chars().all(|e| (e >= '0' && e <= '9') || (e >= 'a' && e <= 'f')))
    }

    fn validate_ecl(ecl: &str) -> bool {
        ECLS.contains(&ecl)
    }

    fn validate_pid(pid: &str) -> bool {
        pid.len() == 9 && pid.chars().all(|e| e.is_numeric())
    }

    fn is_valid_strict(&self) -> bool {
        self.is_valid() &&
            Passport::check_in_range(self.data.get("byr").unwrap(), 1920..=2002) &&
            Passport::check_in_range(self.data.get("iyr").unwrap(), 2010..=2020) &&
            Passport::check_in_range(self.data.get("eyr").unwrap(), 2020..=2030) &&
            Passport::validate_hgt(self.data.get("hgt").unwrap()) &&
            Passport::validate_hcl(self.data.get("hcl").unwrap()) &&
            Passport::validate_ecl(self.data.get("ecl").unwrap()) &&
            Passport::validate_pid(self.data.get("pid").unwrap())
    }
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(line: &'a str) -> Self {
        let mut data = HashMap::new();

        let entries = line.split([' ', '\n'].as_ref()).filter(|e| !e.is_empty());

        for entry in entries {
            let splitted: Vec<&str> = entry.split(":").collect();
            data.insert(splitted[0], splitted[1]);
        }

        Passport { data }
    }
}

fn main() {
    let data = read_file("input");

    let mut count = 0;
    let mut strict_count = 0;
    for line in &data {
        let p = Passport::from(line.as_str());
        if p.is_valid() { count += 1; }
        if p.is_valid_strict() { strict_count += 1; }
    }
    println!("{}/{}", count, data.len());
    println!("Strict: {}/{}", strict_count, data.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        let l1 = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let p1 = Passport::from(l1);
        assert_eq!(p1.data.get("ecl").unwrap(), &"gry");
        assert_eq!(p1.data.get("hgt").unwrap(), &"183cm");
    }

    #[test]
    fn test_is_valid() {
        let ls = vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
                      "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
                      "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
                      "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"];

        let ps: Vec<Passport> = ls.into_iter().map(Passport::from).collect();

        assert!(ps[0].is_valid());
        assert!(!ps[1].is_valid());
        assert!(ps[2].is_valid());
        assert!(!ps[3].is_valid());
    }

    #[test]
    fn test_is_valid_strict() {
        let ls = vec!["eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                      "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
                      "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                      "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
                      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f",
                      "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                      "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
                      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"];

        let ps: Vec<Passport> = ls.into_iter().map(Passport::from).collect();

        assert!(!ps[0].is_valid_strict());
        assert!(!ps[1].is_valid_strict());
        assert!(!ps[2].is_valid_strict());
        assert!(!ps[3].is_valid_strict());
        assert!(ps[4].is_valid_strict());
        assert!(ps[5].is_valid_strict());
        assert!(ps[6].is_valid_strict());
        assert!(ps[7].is_valid_strict());
    }
}