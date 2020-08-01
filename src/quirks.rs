use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quirks {
    quirks: Vec<Quirk>,
}

impl Quirks {
    pub fn from_cfg(cfg: &crate::Config) -> Self {
        let quirks = cfg
            .quirks
            .as_ref()
            .map_or(Vec::new(), |qs| Self::quirks(qs));

        Self { quirks }
    }

    pub fn check_quirk<'a>(&self, text: &'a str) -> Option<&'a str> {
        let mut changed_text = text;
        let mut found_one = false;
        while let Some(quirk) = self.quirks.iter().find(|q| q.call(changed_text).is_some()) {
            changed_text = quirk.call(changed_text).unwrap();
            found_one = true;
        }

        if found_one {
            Some(changed_text)
        } else {
            None
        }
    }

    fn quirks(qs: &[String]) -> Vec<Quirk> {
        let mut qs: Vec<_> = qs
            .iter()
            .map(|quirk| Quirk::from_str(quirk))
            .flatten()
            .collect();
        qs.dedup_by(|a, b| a == b);
        qs
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub enum Quirk {
    SingleQuoted,
    Quoted,
    MultipicityXsuffix,
}

impl Quirk {
    fn from_str(q: &str) -> Option<Self> {
        match q {
            "single-quoted" => Some(Self::SingleQuoted),
            "quoted" => Some(Self::Quoted),
            "multipicity-x-suffix" => Some(Self::MultipicityXsuffix),
            // "dash-free-compound-words" => Some(Self::DashFreeCompoundWords),
            _ => None,
        }
    }

    pub fn call<'a>(&self, text: &'a str) -> Option<&'a str> {
        match self {
            Self::Quoted => {
                if text.len() > 2
                    && text.chars().next() == Some('"')
                    && text.chars().last() == Some('"')
                {
                    Some(&text[1..text.len() - 1])
                } else {
                    None
                }
            }
            Self::SingleQuoted => {
                if text.len() > 2
                    && text.chars().next() == Some('\'')
                    && text.chars().last() == Some('\'')
                {
                    Some(&text[1..text.len() - 1])
                } else {
                    None
                }
            }
            Self::MultipicityXsuffix => {
                if text.len() > 1 && text.chars().last() == Some('x') {
                    Some(&text[..text.len() - 1])
                } else {
                    None
                }
            }
        }
    }
}
