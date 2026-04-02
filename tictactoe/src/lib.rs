use iced::advanced::text::IntoFragment;
use iced::widget::{Button, Container, button, column, row, text};
use iced::{Color, Length};

pub fn app() -> iced::Result {
    iced::run(State::update, State::view)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Message {
    NewGame,
    Change { id: usize },
}

struct State {
    board: [Field; 9],
    current: Field,
    game_end_status: Option<Field>,
    ai: bool,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Field {
    X,
    O,
    #[default]
    Blank,
}

impl IntoFragment<'static> for Field {
    fn into_fragment(self) -> std::borrow::Cow<'static, str> {
        match self {
            Field::X => "X",
            Field::O => "O",
            _ => "",
        }
        .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            board: [Field::Blank; 9],
            current: Field::X,
            game_end_status: None,
            ai: false,
        }
    }
}

impl State {
    fn check_win(&self) -> Option<Field> {
        let wins = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for &line in &wins {
            let [a, b, c] = line;
            if self.board[a] != Field::Blank
                && self.board[a] == self.board[b]
                && self.board[a] == self.board[c]
            {
                return Some(self.board[a]);
            }
        }

        None
    }

    fn update(&mut self, msg: Message) {
        println!("msg: {:?}", msg);

        match msg {
            Message::Change { id } => {
                if self.board[id] != Field::Blank || self.game_end_status.is_some() {
                    return;
                }

                self.board[id] = self.current;

                if let Some(p) = self.check_win() {
                    println!("won: {}", p.into_fragment());
                    self.game_end_status = Some(p);
                } else if !self.board.contains(&Field::Blank) {
                    println!("draw");
                    self.game_end_status = Some(Field::Blank);
                }

                self.current = match self.current {
                    Field::X => Field::O,
                    _ => Field::X,
                };

                if self.ai {

                }
            }

            Message::NewGame => {
                self.board = [Field::Blank; 9];
                self.game_end_status = None;
                self.current = Field::X;
            }
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let make_btn = |id: usize| -> Button<'_, Message> {
            button(text(self.board[id]).size(40).center())
                .width(60)
                .height(60)
                .on_press(Message::Change { id })
                .style(|_, _| iced::widget::button::Style {
                    background: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
                    text_color: Color::from_rgb8(0xe, 0xe, 0xe).into(),
                    ..Default::default()
                })
        };

        let grid = if let Some(w) = self.game_end_status {
            let status = match w {
                Field::X => "won X",
                Field::O => "won O",
                Field::Blank => "draw",
            };

            column![
                row![
                    text(status)
                        .size(20)
                        .center()
                        .width(120)
                        .height(60)
                        .style(|_| iced::advanced::widget::text::Style {
                            color: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
                        }),
                ],
                row![make_btn(0), make_btn(1), make_btn(2)].spacing(10),
                row![make_btn(3), make_btn(4), make_btn(5)].spacing(10),
                row![make_btn(6), make_btn(7), make_btn(8)].spacing(10),
                row![
                    button(text("new game").size(20).center())
                        .width(120)
                        .height(60)
                        .on_press(Message::NewGame)
                        .style(|_, _| iced::widget::button::Style {
                            background: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
                            text_color: Color::from_rgb8(0xe, 0xe, 0xe).into(),
                            ..Default::default()
                        }),
                ]
            ]
            .spacing(10)
        } else {
            column![
                row![
                    text(format!("current: {}", self.current.into_fragment()))
                        .size(20)
                        .center()
                        .width(120)
                        .height(60)
                        .style(|_| iced::advanced::widget::text::Style {
                            color: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
                        }),
                ],
                row![make_btn(0), make_btn(1), make_btn(2)].spacing(10),
                row![make_btn(3), make_btn(4), make_btn(5)].spacing(10),
                row![make_btn(6), make_btn(7), make_btn(8)].spacing(10),
                row![text("").width(60).height(60)],
            ]
            .spacing(10)
        };

        Container::new(grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| iced::widget::container::Style {
                background: Some(Color::from_rgb8(0x1e, 0x1e, 0x2e).into()),
                text_color: None,
                ..Default::default()
            })
    }
}
