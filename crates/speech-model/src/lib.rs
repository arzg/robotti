#[derive(Default)]
pub struct State {
    mode: Mode,
    reminders: Vec<String>,
}

impl State {
    pub fn handle_msg(&mut self, they_said: &str) -> Option<String> {
        let message = Message::from(they_said);

        RULES
            .iter()
            .find(|rule| (rule.applies_to)(self.mode, &message))
            .map(|rule| (rule.gen_reply)(self, &message))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mode {
    Neutral,
    SettingReminder,
    Fight { num_times_said_kil: u32 },
}

impl Default for Mode {
    fn default() -> Self {
        Self::Neutral
    }
}

const RULES: &[Rule] = &[
    Rule::GREETING,
    Rule::WHO,
    Rule::WHY,
    Rule::CREATE_REMINDER,
    Rule::START_FIGHT,
    Rule::CONTINUE_FIGHT,
    Rule::SET_REMINDER,
];

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
        applies_to: |mode, message| {
            mode == Mode::Neutral && message.raw == "Initiating btot fight."
        },
        gen_reply: |state, _| {
            state.mode = Mode::Fight {
                num_times_said_kil: 0,
            };

            "KIL!".to_string()
        },
    };

    const CONTINUE_FIGHT: Self = Self {
        applies_to: |mode, _| matches!(mode, Mode::Fight { .. }),
        gen_reply: |state, _| {
            let num_times_said_kil = if let Mode::Fight {
                ref mut num_times_said_kil,
            } = state.mode
            {
                num_times_said_kil
            } else {
                unreachable!()
            };

            if *num_times_said_kil < 5 {
                *num_times_said_kil += 1;
            } else {
                state.mode = Mode::Neutral;
            }

            "KIL!".to_string()
        },
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
