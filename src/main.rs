use json;
use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    titles: Vec<String>,
    error: Option<String>,
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::WebAccess]);
        subscribe(&[EventType::WebRequestResult, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::Key(k) => {
                eprintln!("{}", k);
                if k == Key::Down {
                    make_posts_web_request();
                }
            }
            Event::WebRequestResult(_status_code, _headers, body, _context) => {
                match parse_data(body) {
                    Ok(data) => {
                        should_render = true;
                        self.titles = data;
                    }
                    Err(e) => self.error = Some(format!("Failed to parse titles: {}", e)),
                }
            }
            _ => (),
        }

        should_render
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        if !self.titles.is_empty() {
            for title in self.titles.iter() {
                print_text(Text::new(title));
                println!();
            }
        } else {
            println!("Web request not made yet");
        }
    }
}

fn parse_data(body: Vec<u8>) -> Result<Vec<String>, String> {
    let mut vec = Vec::new();

    String::from_utf8(body)
        .map_err(|e| e.to_string())
        .and_then(|b| json::parse(&b).map_err(|e| e.to_string()))
        .and_then(|body| {
            let items = &body["items"];

            for item in items.members().take(3) {
                vec.push(item["name"].to_string());
            }

            Ok(vec)
        })
}

fn make_posts_web_request() {
    let context = BTreeMap::new();
    web_request(
        "https://lannonbr.com/posts.json",
        HttpVerb::Get,
        BTreeMap::new(),
        vec![],
        context,
    );
}

register_plugin!(State);
