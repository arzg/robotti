use oorandom::Rand32;
use std::convert::{TryFrom, TryInto};

pub struct State {
    mode: Mode,
    reminders: Vec<String>,
    rng: Rand32,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            mode: Mode::default(),
            reminders: Vec::new(),
            rng: {
                let mut rand_bytes = [0; 8];

                getrandom::getrandom(&mut rand_bytes)
                    .map_err(|_| anyhow::anyhow!("failed to initialise RNG"))?;

                Rand32::new(u64::from_be_bytes(rand_bytes))
            },
        })
    }

    pub fn handle_msg(&mut self, they_said: &str) -> Option<String> {
        let message = Message::from(they_said);

        RULES.iter().find_map(|rule| rule(self, &message))
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
    greeting,
    who,
    why,
    ladder_collection,
    create_reminder,
    set_reminder_value,
    list_reminders,
    start_fight,
    continue_fight,
    joke,
];

type Rule = fn(&mut State, &Message) -> Option<String>;

fn greeting(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral
        || (!message.contains_component("hello") && !message.contains_component(&"hi"))
    {
        return None;
    }

    Some("Greeting".to_string())
}

fn who(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || !message.contains_component("who") {
        return None;
    }

    Some("Glad you asked! I’m robotti, your personal assistant.".to_string())
}

fn why(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || !message.contains_component("why") {
        return None;
    }

    Some("Why not?".to_string())
}

fn ladder_collection(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || message.raw != "Thanks. I take pride in my ladder collection."
    {
        return None;
    }

    Some("It’s not that good tbh".to_string())
}

fn create_reminder(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || !message.contains_component("reminder") {
        return None;
    }

    state.mode = Mode::SettingReminder;

    Some("What would you like me to remind you of?".to_string())
}

fn set_reminder_value(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::SettingReminder {
        return None;
    }

    state.reminders.push(message.raw.to_string());
    state.mode = Mode::Neutral;

    Some("OK, I’ll remember that.".to_string())
}

fn list_reminders(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral
        || !message.contains_component("list")
        || !message.contains("reminder")
    {
        return None;
    }

    let num_reminders = state.reminders.len();

    if num_reminders == 0 {
        return Some("You have no reminders".to_string());
    }

    let mut response = format!(
        "You have {} {}:",
        num_reminders,
        if num_reminders == 1 {
            "reminder"
        } else {
            "reminders"
        }
    );

    for reminder in &state.reminders {
        response.push_str(&format!("\n{}", reminder));
    }

    Some(response)
}

fn start_fight(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || message.raw != "Initiating btot fight." {
        return None;
    }

    state.mode = Mode::Fight {
        num_times_said_kil: 0,
    };

    Some("KIL!".to_string())
}

fn continue_fight(state: &mut State, _: &Message) -> Option<String> {
    let num_times_said_kil = match state.mode {
        Mode::Fight {
            ref mut num_times_said_kil,
        } => num_times_said_kil,
        _ => return None,
    };

    if *num_times_said_kil < 5 {
        *num_times_said_kil += 1;
    } else {
        state.mode = Mode::Neutral;
    }

    Some("KIL!".to_string())
}

fn joke(state: &mut State, message: &Message) -> Option<String> {
    if state.mode != Mode::Neutral || !message.contains_component("joke") {
        return None;
    }

    const JOKES: &str = include_str!("jokes");
    let joke_lines: Vec<_> = JOKES.lines().collect();

    let rand_idx = state
        .rng
        .rand_range(0..joke_lines.len().try_into().unwrap());

    let random_joke = joke_lines[usize::try_from(rand_idx).unwrap()];

    Some(random_joke.to_string())
}

struct Message<'a> {
    raw: &'a str,
    components: Vec<String>,
}

impl Message<'_> {
    fn contains(&self, s: &str) -> bool {
        self.raw.contains(s)
    }

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
