use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(12)) class_name="home-icons">
                    <a class="icon" href="https://github.com/trivernis" target="_blank">
                        <img src="assets/github-logo.png"/>
                    </a>
                    <a class="icon" href="https://discord.gg/ZxzM2bTeXU" target="_blank">
                        <img src="assets/discord-logo.png"/>
                    </a>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(8)) class_name="home-greeting">
                    <h1>{"Welcome to my website"}</h1>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(8)) class_name="home-description">
                    <p>{r#"
                    I'm a german software developer working on way too many projects at once.
                    I primarily use rust because it's just the best programming language.
                    "#}</p>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12), ItemLayout::ItM(8)) class_name="github-stats">
                    <img src="https://github-readme-stats.vercel.app/api?username=trivernis&show_icons=true&theme=tokyonight"/>
                    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=trivernis&hide=html&show_icons=true&theme=tokyonight" />
                </Item>
            </Container>
        }
    }
}
