use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Container, button, checkbox, column, container, row, text},
};
use rayon::prelude::*;

pub fn run() {
    iced::run(State::update, State::view).unwrap();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Message {
    NewGame,
    StartGame,
    SetSize(usize),
    UseAi(bool),
    ChangeField(usize),
}

#[derive(Clone, Debug)]
enum State {
    NewGame {
        size: usize,
        ai: bool,
    },

    InGame {
        board: Vec<Field>,
        current: Field,
        ai: bool,
        win_lines: Vec<Vec<usize>>,
    },

    AfterGame {
        board: Vec<Field>,
        won: Field,
    },
}

impl Default for State {
    fn default() -> Self {
        Self::NewGame { size: 3, ai: false }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Field {
    X,
    O,
    #[default]
    Blank,
}

impl State {
    fn update(&mut self, msg: Message) {
        println!("msg: {:?}", msg);

        match self {
            Self::NewGame { size, ai } => match msg {
                Message::SetSize(s) => *size = s,

                Message::UseAi(v) => *ai = v,

                Message::StartGame => {
                    *self = State::InGame {
                        board: vec![Field::Blank; *size * *size],
                        current: Field::X,
                        ai: *ai,
                        win_lines: generate_win_lines(*size),
                    };
                }

                _ => {}
            },

            Self::InGame {
                board,
                current,
                ai,
                win_lines,
            } => match msg {
                Message::ChangeField(id) => {
                    let win = |board: &[Field], win_lines: &[Vec<usize>]| -> Option<Field> {
                        if let Some(p) = check_win(board, win_lines) {
                            println!("won: {:?}", p);
                            Some(p)
                        } else if !board.contains(&Field::Blank) {
                            println!("draw");
                            Some(Field::Blank)
                        } else {
                            None
                        }
                    };

                    if board[id] != Field::Blank {
                        return;
                    }

                    board[id] = *current;

                    if let Some(p) = win(board, win_lines) {
                        *self = State::AfterGame {
                            won: p,
                            board: board.to_vec(),
                        };
                        return;
                    }

                    if *ai {
                        let ai_move = ai_find_best_pos(board, win_lines).unwrap();
                        board[ai_move] = Field::O;

                        if let Some(p) = win(board, win_lines) {
                            *self = State::AfterGame {
                                won: p,
                                board: board.to_vec(),
                            };
                            return;
                        }
                    } else {
                        *current = match current {
                            Field::X => Field::O,
                            _ => Field::X,
                        };
                    }
                }

                Message::NewGame => *self = Self::default(),

                _ => {}
            },

            Self::AfterGame { .. } => match msg {
                Message::NewGame => *self = Self::default(),
                _ => {}
            },
        }

        println!("game state: {:?}", self);
    }

    fn view(&self) -> Element<'_, Message> {
        let btn_style = button::Style {
            background: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
            text_color: Color::from_rgb8(0xe, 0xe, 0xe).into(),
            ..Default::default()
        };

        let text_style = text::Style {
            color: Some(Color::from_rgb8(0xee, 0xee, 0xee).into()),
        };

        let view_ui = |top_text: &'static str,
                       button_text: &'static str,
                       board: &[Field]|
         -> Element<Message> {
            let make_btn = |id: usize| -> Button<'_, Message> {
                let field_text = match board[id] {
                    Field::X => "X",
                    Field::O => "O",
                    _ => "",
                };

                button(text(field_text).size(40).center())
                    .width(60)
                    .height(60)
                    .on_press(Message::ChangeField(id))
                    .style(move |_, _| btn_style)
            };

            let n = (board.len() as f64).sqrt() as usize;

            let btn_board: Vec<Element<Message>> = (0..n).fold(Vec::new(), |mut acc, y| {
                let btn_board: Vec<Element<Message>> = (0..n).fold(Vec::new(), |mut acc, x| {
                    acc.push(make_btn((y * n) + x).into());
                    acc
                });

                acc.push(row(btn_board).spacing(10).into());
                acc
            });

            column![
                text(top_text).size(35).style(move |_| text_style),
                column(btn_board).spacing(10),
                button(button_text)
                    .style(move |_, _| btn_style)
                    .padding(16)
                    .on_press(Message::NewGame),
            ]
            .align_x(Alignment::Center)
            .spacing(40)
            .into()
        };

        let element: Element<Message> = match self {
            Self::NewGame { size, ai } => {
                let plus = button("+")
                    .on_press(Message::SetSize(size + 1))
                    .style(move |_, _| btn_style)
                    .padding(18);

                let minus = button("-")
                    .style(move |_, _| btn_style)
                    .on_press(Message::SetSize(if *size > 3 { size - 1 } else { *size }))
                    .padding(18);

                column![
                    row![
                        plus,
                        text(format!("board size: {}", size)).style(move |_| text_style),
                        minus
                    ]
                    .spacing(20)
                    .align_y(Alignment::Center),
                    row![
                        text("use ai:").style(move |_| text_style),
                        checkbox(*ai).on_toggle(|v| Message::UseAi(v))
                    ]
                    .spacing(10),
                    button("Start Game")
                        .style(move |_, _| btn_style)
                        .padding(16)
                        .on_press(Message::StartGame)
                ]
                .spacing(40)
                .padding(20)
                .align_x(Alignment::Center)
                .into()
            }

            Self::InGame { board, .. } => view_ui("   ", "Restart Game", board),

            Self::AfterGame { board, won } => {
                let won = match won {
                    Field::X => "Won X",
                    Field::O => "Won O",
                    _ => "Draw",
                };

                view_ui(won, "New Game", board)
            }
        };

        Container::new(element)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color::from_rgb8(0x1e, 0x1e, 0x2e).into()),
                text_color: None,
                ..Default::default()
            })
            .into()
    }
}

fn generate_win_lines(n: usize) -> Vec<Vec<usize>> {
    let mut lines = Vec::new();

    for r in 0..n {
        let mut row = Vec::new();

        for c in 0..n {
            row.push(r * n + c);
        }

        lines.push(row);
    }

    for c in 0..n {
        let mut col = Vec::new();

        for r in 0..n {
            col.push(r * n + c);
        }

        lines.push(col);
    }

    let mut diag1 = Vec::new();

    for i in 0..n {
        diag1.push(i * n + i);
    }

    lines.push(diag1);

    let mut diag2 = Vec::new();

    for i in 0..n {
        diag2.push(i * n + (n - 1 - i));
    }

    lines.push(diag2);

    lines
}

fn check_win(board: &[Field], win_lines: &[Vec<usize>]) -> Option<Field> {
    win_lines.iter().find_map(|line| {
        let mut iter = line.iter().map(|&i| board[i]);

        let first = iter.next()?;
        if first == Field::Blank {
            return None;
        }

        if iter.all(|f| f == first) {
            Some(first)
        } else {
            None
        }
    })
}

fn ai_find_best_pos(board: &[Field], win_lines: &Vec<Vec<usize>>) -> Option<usize> {
    let n = (board.len() as f64).sqrt() as usize;

    let max_depth = match n {
        3 => 8,
        4 => 7,
        5 => 6,
        6 => 5,
        _ => 4,
    };

    let root_moves: Vec<usize> = board
        .iter()
        .enumerate()
        .filter(|(_, f)| **f == Field::Blank)
        .map(|(i, _)| i)
        .collect();

    let results: Vec<(i64, usize)> = root_moves
        .into_par_iter()
        .map(|mv| {
            let mut new_board = board.to_vec();
            new_board[mv] = Field::O;

            let (score, _) = minimax(
                &mut new_board,
                n,
                false,
                1,
                i64::MIN,
                i64::MAX,
                max_depth,
                win_lines,
            );

            (score, mv)
        })
        .collect();

    results
        .into_iter()
        .max_by_key(|(score, _)| *score)
        .map(|(_, mv)| mv)
}

fn minimax(
    board: &mut [Field],
    n: usize,
    is_maximizing: bool,
    depth: i64,
    mut alpha: i64,
    mut beta: i64,
    max_depth: i64,
    win_lines: &Vec<Vec<usize>>,
) -> (i64, Option<usize>) {
    if let Some(winner) = check_win(board, win_lines) {
        let result = match winner {
            Field::X => (-10 + depth, None),
            Field::O => (10 - depth, None),
            _ => (0, None),
        };

        return result;
    }

    if depth >= max_depth {
        let result = (evaluate(board, win_lines), None);
        return result;
    }

    if !board.contains(&Field::Blank) {
        return (0, None);
    }

    let mut best_move = None;

    if is_maximizing {
        let mut best_score = i64::MIN;

        for i in 0..board.len() {
            if board[i] != Field::Blank {
                continue;
            }

            board[i] = Field::O;
            let (score, _) = minimax(
                board,
                n,
                false,
                depth + 1,
                alpha,
                beta,
                max_depth,
                win_lines,
            );
            board[i] = Field::Blank;

            if score > best_score {
                best_score = score;
                best_move = Some(i);
            }

            alpha = alpha.max(best_score);
            if alpha >= beta {
                break;
            }
        }

        (best_score, best_move)
    } else {
        let mut best_score = i64::MAX;

        for i in 0..board.len() {
            if board[i] != Field::Blank {
                continue;
            }

            board[i] = Field::X;
            let (score, _) = minimax(board, n, true, depth + 1, alpha, beta, max_depth, win_lines);
            board[i] = Field::Blank;

            if score < best_score {
                best_score = score;
                best_move = Some(i);
            }

            beta = beta.min(best_score);
            if alpha >= beta {
                break;
            }
        }

        (best_score, best_move)
    }
}

fn evaluate(board: &[Field], win_lines: &Vec<Vec<usize>>) -> i64 {
    let mut score = 0;

    for line in win_lines {
        let mut x = 0;
        let mut o = 0;

        for &i in line {
            match board[i] {
                Field::X => x += 1,
                Field::O => o += 1,
                _ => {}
            }
        }

        let len = line.len();

        if x == 0 {
            match o {
                n if n == len => score += 1000,
                n if n == len - 1 => score += 50,
                n if n == len - 2 => score += 10,
                _ => {}
            }
        }

        if o == 0 {
            match x {
                n if n == len => score -= 1000,
                n if n == len - 1 => score -= 50,
                n if n == len - 2 => score -= 10,
                _ => {}
            }
        }
    }

    score
}
