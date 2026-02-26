#[derive(Default, PartialEq)]
pub enum TicketSource {
    #[default]
    Terminal,
    External,
}

#[derive(Default)]
pub struct State {
    pub source: TicketSource,
    pub terminal_code: String,
    pub online_code: String,
}
