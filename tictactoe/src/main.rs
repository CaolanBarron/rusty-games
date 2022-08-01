use tictactoe::{TicTacToe, Position, Shape};
use yew::prelude::*;
use weblog::console_log;
enum Msg {
    SetX(Position),
    SetO(Position),
}

struct Board {
    game: TicTacToe,
}

impl Component for Board {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: TicTacToe::new(3, 3),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetX(pos) => self.game.set_square(pos.x, pos.y, Shape::X),
            Msg::SetO(pos) => self.game.set_square(pos.x, pos.y, Shape::O),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        if self.game.winner != None {
            console_log!(self.game.winner.unwrap().to_string());
        }

        let mut positions = Vec::new();

        for y in 1..=self.game.size {
            for x in 1..=self.game.size {
                positions.push(Position{x, y})
            }
        }

        let cb = ctx.link();
        
        html!{
            <div class = "root">
                <h1>{"Tic Tac Toe"}</h1>
                <h1>{"Rust"}</h1>
                {if self.game.winner != None{
                    html!{
                        <h2>{self.game.display_victor()}</h2>
                    }
                 }else {
                    html!{
                        <h2>{"No winner"}</h2>
                    }
                 }
                }
            <div class = "board-base">

            <div class = "board">
            {
                positions.into_iter().map(|pos| {
                    html!{
                        <button class="square" 
                            onclick={cb.callback(move |_| Msg::SetX(pos))} 
                            oncontextmenu={ cb.callback(move |_| Msg::SetO(pos))} >
                            {self.game.get_square(pos.x, pos.y)}
                        </button>
                   }
                }).collect::<Html>()
            }
            </div>
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Board>();
}
