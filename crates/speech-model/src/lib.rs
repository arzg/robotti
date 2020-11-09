#[derive(Default)]
pub struct State {
    mode: Mode,
    reminders: Vec<String>,
}

impl State {
    pub fn handle_msg(&mut self, they_said: &str) -> Option<String> {
        let message = Message::from(they_said);

        match self.mode {
            Mode::Neutral => {
                if message.contains_component(&"hello") || message.contains_component(&"hi") {
                    Some("Greetings!".to_string())
                } else if message.contains_component(&"who") {
                    Some("Glad you asked! I’m robotti, your personal assistant.".to_string())
                } else if message.contains_component("why") {
                    Some("Why not?".to_string())
                } else if message.contains_component(&"reminder") {
                    self.mode = Mode::SettingReminder;
                    Some("What would you like me to remind you of?".to_string())
                } else if they_said == "Initiating btot fight" {
                    self.mode = Mode::Fight;
                    Some("KIL!".to_string())
                } else {
                    None
                }
            }
            Mode::SettingReminder => {
                self.reminders.push(they_said.to_string());
                self.mode = Mode::Neutral;

                Some("OK, I’ll remember that.".to_string())
            }
            Mode::Fight => Some("KIL!".to_string()),
        }
    }
}

enum Mode {
    Neutral,
    SettingReminder,
    Fight,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Neutral
    }
}

struct Message {
    lowercase: String,
    components: Vec<String>,
}

impl Message {
    fn contains_component(&self, component: &str) -> bool {
        self.components.iter().any(|c| c == component)
    }
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        let lowercase = s.to_lowercase();

        let components = lowercase
            .split(|c: char| !c.is_alphanumeric())
            .map(str::to_string)
            .collect();

        Self {
            lowercase,
            components,
        }
    }
}
