use common::{AppState, Message};
use iced::{Element, Text};

#[no_mangle]
pub fn view(state: &mut AppState) -> Element<Message> {
    let text = Text::new("CHANGE ME");
    iced::Button::new(&mut state.button_state, text)
        .on_press(Message::Foo)
        .into()
}
