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
                if (Rule::GREETING.applies_to)(self.mode, &message) {
                    Some((Rule::GREETING.gen_reply)(self, &message))
                } else if (Rule::WHO.applies_to)(self.mode, &message) {
                    Some((Rule::WHO.gen_reply)(self, &message))
                } else if (Rule::WHY.applies_to)(self.mode, &message) {
                    Some((Rule::WHY.gen_reply)(self, &message))
                } else if (Rule::CREATE_REMINDER.applies_to)(self.mode, &message) {
                    Some((Rule::CREATE_REMINDER.gen_reply)(self, &message))
                } else if (Rule::START_FIGHT.applies_to)(self.mode, &message) {
                    Some((Rule::START_FIGHT.gen_reply)(self, &message))
                } else {
                    None
                }
            }
            Mode::SettingReminder => Some((Rule::SET_REMINDER.gen_reply)(self, &message)),
            Mode::Fight => Some((Rule::CONTINUE_FIGHT.gen_reply)(self, &message)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
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

struct Rule {
    applies_to: fn(Mode, &Message) -> bool,
    gen_reply: fn(&mut State, &Message) -> String,
}

impl Rule {
    const GREETING: Self = Self {
        applies_to: |mode, message| {
            mode == Mode::Neutral && message.contains_component("hello")
                || message.contains_component(&"hi")
        },
        gen_reply: |_, _| "Greeting".to_string(),
    };

    const WHO: Self = Self {
        applies_to: |mode, message| mode == Mode::Neutral && message.contains_component("who"),
        gen_reply: |_, _| "Glad you asked! I’m robotti, your personal assistant.".to_string(),
    };

    const WHY: Self = Self {
        applies_to: |mode, message| mode == Mode::Neutral && message.contains_component("why"),
        gen_reply: |_, _| "Why not?".to_string(),
    };

    const CREATE_REMINDER: Self = Self {
        applies_to: |mode, message| mode == Mode::Neutral && message.contains_component("reminder"),
        gen_reply: |state, _| {
            state.mode = Mode::SettingReminder;
            "What would you like me to remind you of?".to_string()
        },
    };

    const SET_REMINDER: Self = Self {
        applies_to: |mode, _| mode == Mode::SettingReminder,
        gen_reply: |state, message| {
            state.reminders.push(message.raw.to_string());
            state.mode = Mode::Neutral;

            "OK, I’ll remember that.".to_string()
        },
    };

    const START_FIGHT: Self = Self {
        applies_to: |mode, message| mode == Mode::Neutral && message.raw == "Initiating btot fight",
        gen_reply: |state, _| {
            state.mode = Mode::Fight;
            "KIL!".to_string()
        },
    };

    const CONTINUE_FIGHT: Self = Self {
        applies_to: |mode, _| mode == Mode::Fight,
        gen_reply: |_, _| "KIL!".to_string(),
    };
}

struct Message<'a> {
    raw: &'a str,
    components: Vec<String>,
}

impl Message<'_> {
    fn contains_component(&self, component: &str) -> bool {
        self.components.iter().any(|c| c == component)
    }
}

impl<'a> From<&'a str> for Message<'a> {
    fn from(s: &'a str) -> Self {
        let lowercase = s.to_lowercase();

        let components = lowercase
            .split(|c: char| !c.is_alphanumeric())
            .map(str::to_string)
            .collect();

        Self { raw: s, components }
    }
}
