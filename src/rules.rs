use crate::shared::WfcError;
use crate::vec2i;
use crate::vec2i::Vec2i;
use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use vec2i::{DOWN, LEFT, RIGHT, UP};

/// Ruleset is a struct that contains a list of rules for the WFC algorithm.
#[derive(Debug, Deserialize)]
pub struct Ruleset {
    rules: Vec<Rule>,
}
#[derive(Debug, Deserialize)]
struct Rule {
    field: i8,
    allowed_up: HashMap<i8, f32>,
    allowed_right: HashMap<i8, f32>,
    allowed_down: HashMap<i8, f32>,
    allowed_left: HashMap<i8, f32>,
}

impl Ruleset {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        from_str(yaml)
    }

    pub fn all_fields(&self) -> Vec<i8> {
        self.rules.iter().map(|rule| rule.field).collect()
    }

    pub fn get_allowed_fields(
        &self,
        field: i8,
        direction: Vec2i,
    ) -> crate::Result<&HashMap<i8, f32>> {
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
        allowed_up:
            1: 0.5
            2: 1.0
        allowed_right:
            1: 1.0
            3: 1.0
        allowed_down:
            2: 1.0
            4: 0.9
        allowed_left:
            1: 0.4
            3: 1.0
      - field: 2
        allowed_up:
            1: 1.0
            2: 1.0
        allowed_right:
            1: 1.0
            3: 1.0
        allowed_down:
            2: 1.0
            4: 0.9
        allowed_left:
            1: 0.5
            3: 1.0
    "#;

    #[test]
    fn test_ruleset_parsing() {
        let ruleset = Ruleset::from_yaml(RULESET_YAML).unwrap();
        assert_eq!(ruleset.rules.len(), 2);
        assert_eq!(ruleset.rules[0].field, 1);
        assert_eq!(ruleset.rules[1].field, 2);
        assert_eq!(ruleset.rules[0].allowed_up.get(&1), Some(&0.5));
        assert_eq!(ruleset.rules[0].allowed_up.get(&2), Some(&1.0));
        assert_eq!(ruleset.rules[0].allowed_right.get(&1), Some(&1.0));
        assert_eq!(ruleset.rules[0].allowed_right.get(&3), Some(&1.0));
        assert_eq!(ruleset.rules[0].allowed_down.get(&2), Some(&1.0));
        assert_eq!(ruleset.rules[0].allowed_down.get(&4), Some(&0.9));
        assert_eq!(ruleset.rules[0].allowed_left.get(&1), Some(&0.4));
        assert_eq!(ruleset.rules[0].allowed_left.get(&3), Some(&1.0));
        assert_eq!(ruleset.rules[1].allowed_down.get(&4), Some(&0.9));
        assert_eq!(ruleset.rules[1].allowed_left.get(&1), Some(&0.5));
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
        assert_eq!(allowed_up, &HashMap::from([(1, 0.5), (2, 1.0)]));
        let allowed_right = ruleset.get_allowed_fields(1, RIGHT).unwrap();
        assert_eq!(allowed_right, &HashMap::from([(1, 1.0), (3, 1.0)]));
        let allowed_down = ruleset.get_allowed_fields(1, DOWN).unwrap();
        assert_eq!(allowed_down, &HashMap::from([(2, 1.0), (4, 0.9)]));
        let allowed_left = ruleset.get_allowed_fields(1, LEFT).unwrap();
        assert_eq!(allowed_left, &HashMap::from([(1, 0.4), (3, 1.0)]));
    }
}
