use crate::shared::WfcError;
use crate::vec2i;
use crate::vec2i::Vec2i;
use serde::Deserialize;
use serde_yaml::from_str;
use vec2i::{DOWN, LEFT, RIGHT, UP};

#[derive(Debug, Deserialize)]
pub struct Ruleset {
    rules: Vec<Rule>,
}
#[derive(Debug, Deserialize)]
struct Rule {
    field: i8,
    allowed_up: Vec<i8>,
    allowed_right: Vec<i8>,
    allowed_down: Vec<i8>,
    allowed_left: Vec<i8>,
}

impl Ruleset {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        from_str(yaml)
    }

    pub fn all_fields(&self) -> Vec<i8> {
        self.rules.iter().map(|rule| rule.field).collect()
    }

    pub fn get_allowed_fields(&self, field: i8, direction: Vec2i) -> crate::Result<&Vec<i8>> {
        let rule = self.rules.iter().find(|rule| rule.field == field);
        if let Some(rule) = rule {
            return match direction {
                UP => Ok(&rule.allowed_up),
                RIGHT => Ok(&rule.allowed_right),
                DOWN => Ok(&rule.allowed_down),
                LEFT => Ok(&rule.allowed_left),
                _ => Err(WfcError::new(format!(
                    "get_allowed_fields -> Invalid direction: {:?}",
                    direction,
                ))),
            };
        }
        Err(WfcError::new(format!(
            "get_allowed_fields -> No rule found for field: {}",
            field
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULESET_YAML: &str = r#"
    rules:
      - field: 1
        allowed_up: [2, 3]
        allowed_right: [4, 5]
        allowed_down: [6, 7]
        allowed_left: [8, 9]
      - field: 2
        allowed_up: [1]
        allowed_right: [3]
        allowed_down: [4]
        allowed_left: [5]
    "#;

    #[test]
    fn test_ruleset_parsing() {
        let ruleset = Ruleset::from_yaml(RULESET_YAML).unwrap();
        assert_eq!(ruleset.rules.len(), 2);
        assert_eq!(ruleset.rules[0].field, 1);
        assert_eq!(ruleset.rules[1].field, 2);
        assert_eq!(ruleset.rules[0].allowed_up, vec![2, 3]);
        assert_eq!(ruleset.rules[0].allowed_right, vec![4, 5]);
        assert_eq!(ruleset.rules[0].allowed_down, vec![6, 7]);
        assert_eq!(ruleset.rules[0].allowed_left, vec![8, 9]);
        assert_eq!(ruleset.rules[1].allowed_up, vec![1]);
        assert_eq!(ruleset.rules[1].allowed_right, vec![3]);
        assert_eq!(ruleset.rules[1].allowed_down, vec![4]);
        assert_eq!(ruleset.rules[1].allowed_left, vec![5]);
        //println!("parsed ruleset: {:#?}", ruleset);
    }

    #[test]
    fn test_ruleset_all_fields() {
        let ruleset = Ruleset::from_yaml(RULESET_YAML).unwrap();
        let all_fields = ruleset.all_fields();
        assert_eq!(all_fields.len(), 2);
        assert_eq!(all_fields, vec![1, 2]);
    }

    #[test]
    fn test_get_allowed_fields() {
        let ruleset = Ruleset::from_yaml(RULESET_YAML).unwrap();
        let allowed_up = ruleset.get_allowed_fields(1, UP).unwrap();
        assert_eq!(allowed_up, &vec![2, 3]);
        let allowed_right = ruleset.get_allowed_fields(1, RIGHT).unwrap();
        assert_eq!(allowed_right, &vec![4, 5]);
        let allowed_down = ruleset.get_allowed_fields(1, DOWN).unwrap();
        assert_eq!(allowed_down, &vec![6, 7]);
        let allowed_left = ruleset.get_allowed_fields(1, LEFT).unwrap();
        assert_eq!(allowed_left, &vec![8, 9]);
    }
}
