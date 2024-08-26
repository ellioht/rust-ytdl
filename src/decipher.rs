use anyhow::{Result, Context};
use regex::Regex;
use std::collections::HashMap;
use reqwest::Url;

pub struct SigDecipher {
    url: String,
    player: String,
    func_regex: Regex,
    actions_regex: Regex,
}

impl SigDecipher {
    pub fn new(url: String, player: String) -> Self {
        SigDecipher {
            url,
            player,
            func_regex: Regex::new(r"(.{2}):function\(.*?\)\{(.*?)\}").unwrap(),
            actions_regex: Regex::new(r";.{2}\.(.{2})\(.*?,(.*?)\)").unwrap(),
        }
    }

    pub fn decipher(&self) -> Result<String> {
        let args: HashMap<String, String> = self.parse_query_string(&self.url);
        let functions = self.get_functions()?;
        let mut signature: Vec<char> = args.get("s")
            .context("Missing 's' parameter in query string")?
            .chars()
            .collect();

        for action in self.actions_regex.captures_iter(&self.player) {
            let func = action.get(1).context("Missing function name in action")?.as_str();
            let param = action.get(2).context("Missing parameter in action")?.as_str().parse::<usize>()
                .context("Failed to parse parameter as usize")?;

            match func {
                f if f == functions[0] => reverse(&mut signature),
                f if f == functions[1] => splice(&mut signature, param),
                f if f == functions[2] => swap(&mut signature, param),
                _ => (),
            }
        }

        let mut url_components = Url::parse(&args["url"]).context("Failed to parse URL")?;
        if let Some(sp) = args.get("sp") {
            url_components.query_pairs_mut().append_pair(sp, &signature.iter().collect::<String>());
        } else {
            url_components.query_pairs_mut().append_pair("signature", &signature.iter().collect::<String>());
        }

        Ok(url_components.to_string())
    }

    fn parse_query_string(&self, url: &str) -> HashMap<String, String> {
        HashMap::new()
    }

    fn get_functions(&self) -> Result<Vec<&str>> {
        Ok(vec!["func1", "func2", "func3"])
    }
}

fn reverse(signature: &mut Vec<char>) {
    signature.reverse();
}

fn splice(signature: &mut Vec<char>, index: usize) {
    signature.drain(index..);
}

fn swap(signature: &mut Vec<char>, index: usize) {
    signature.swap(0, index);
}