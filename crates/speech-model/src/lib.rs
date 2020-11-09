#[derive(Default)]
pub struct State {
    mode: Mode,
    reminders: Vec<String>,
}

impl State {
    pub fn handle_msg(&mut self, they_said: &str) -> Option<String> {
        let they_said = they_said.to_lowercase();
        let components: Vec<_> = they_said.split(|c: char| !c.is_alphanumeric()).collect();

        match self.mode {
            Mode::Neutral => {
                if components.contains(&"hello") || components.contains(&"hi") {
                    Some("Greetings!".to_string())
                } else if components.contains(&"who") {
                    Some("Glad you asked! I’m robotti, your personal assistant.".to_string())
                } else if components.contains(&"why") {
                    Some("Why not?".to_string())
                } else if components.contains(&"reminder") {
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
