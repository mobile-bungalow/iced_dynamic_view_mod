use dymod::dymod;
use iced::{executor, Application, Command, Element, Settings};

pub fn main() {
    Hello::run(Settings::default()).unwrap();
}

dymod! {
    #[path = "../view_mod/lib.rs"]
    pub mod view_mod {
        fn view(app_state: &mut common::AppState) -> Element<common::Message>;
    }
}

struct Hello(common::AppState);

impl Application for Hello {
    type Executor = executor::Default;
    type Message = common::Message;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (
            Hello(common::AppState {
                button_state: iced::button::State::new(),
            }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        dbg!(message);
        view_mod::reload();
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        view_mod::view(&mut self.0)
    }
}
