pub struct Rule{
    rule_str: String,
    survive: Vec<u8>,
    birth: Vec<u8>
}

impl Rule {

    pub fn new(rule: String) -> Result<Rule, String> {
        let mut survive: Vec<u8> = Vec::new();
        let mut birth: Vec<u8> = Vec::new();

        let mut found_slash = false;

        for c in rule.chars() {
            match c {
                '/' if !found_slash => found_slash = true,
                e @ '0'...'8' if !found_slash => survive.push(e.to_digit(10).unwrap() as u8),
                e @ '0'...'8' if found_slash => birth.push(e.to_digit(10).unwrap() as u8),
                e => return Err(format!("Invalid char '{}' in rule \"{}\"", e, rule))
            }
        }

        let rule = Rule {
            rule_str: rule,
            survive: survive,
            birth: birth
        };

        Ok(rule)
    }

    pub fn apply(&self, cell: bool, neighbours: u8) -> bool {
        match cell {
            true if contains(&self.survive, neighbours) => true,
            false if contains(&self.birth, neighbours) => true,
            _ => false
        }
    }

    pub fn get_string(&self) -> &str {
        &self.rule_str
    }

}

fn contains<T>(v: &Vec<T>, value: T) -> bool
where T: PartialEq<T> {

    for e in v {
        if *e == value {
            return true;
        }
    }
    false
}
