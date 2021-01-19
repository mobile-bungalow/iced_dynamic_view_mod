#[derive(Copy, Clone, Debug)]
pub enum Message {
    Foo,
    Bar,
}

#[derive(Copy, Clone, Debug)]
pub struct AppState {
    pub button_state: iced_native::button::State,
}
