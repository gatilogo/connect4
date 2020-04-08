use yew::prelude::*;

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    difficulty: Difficulty,
    player1_name: String,
    player2_name: String
}

pub enum Msg {
    GotPlayer1Name(String),
    GotPlayer2Name(String),
    GotDifficulty(Difficulty),
    StartGame
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Component for Connect4Computer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4Computer {
            link,
            difficulty: Difficulty::Easy,
            player1_name: "".into(),
            player2_name: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayer1Name(name) => self.player1_name = name,
            Msg::GotPlayer2Name(name) => self.player2_name = name,
            Msg::GotDifficulty(difficulty) => self.difficulty = difficulty,
            Msg::StartGame => {

            }
        }
        true
    }
    
    // VIEW
    fn view(&self) -> Html {
        html! {
            <div>
                <h1><b>{"Enter Your Name"}</b></h1>
                <div>
                    <input
                        type="text"
                        value=&self.player1_name
                        oninput=self.link.callback(|e: InputData| Msg::GotPlayer1Name(e.value))
                        placeholder="Your Name"/>
                </div>
                <h1><b>{"Enter Difficulty"}</b></h1>
                <div>
                    <select>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Easy))>{"Easy"}</option>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Medium))>{"Medium"}</option>
                        <option onclick=self.link.callback(|_| Msg::GotDifficulty(Difficulty::Hard))>{"Hard"}</option>
                    </select>
                </div>
                <br></br>
                <button onclick=self.link.callback(|_| Msg::StartGame)>{"Start Game"}</button>
            </div>
        }
    }
}