use yew::prelude::*;

use monaco::{api::{CodeEditorOptions, TextModel}, sys::editor::BuiltinTheme, yew::CodeEditor};
use std::rc::Rc;

use crate::convert::convert;
use crate::example::example;

pub enum Msg {
    Run(),
    LoadExample(),
}

pub struct App {
    option_verilog: Rc<CodeEditorOptions>,
    model_verilog: TextModel,

    option_spinal: Rc<CodeEditorOptions>,
    model_spinal: TextModel,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let options_verilog = CodeEditorOptions::default()
            .with_language("verilog".to_owned())
            .with_value(String::new())
            .with_builtin_theme(BuiltinTheme::VsDark);
        let models_verilog = TextModel::create("", Some("verilog"), None).unwrap();

        let options_spinal = CodeEditorOptions::default()
            .with_language("scala".to_owned())
            .with_value(String::new())
            .with_builtin_theme(BuiltinTheme::VsDark);
        let models_spinal = TextModel::create("", Some("scala"), None).unwrap();

        Self {
            option_verilog: Rc::new(options_verilog),
            model_verilog: models_verilog,

            option_spinal: Rc::new(options_spinal),
            model_spinal: models_spinal,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run() => {
                let spinal_text = convert(&self.model_verilog.get_value());
                self.model_spinal = TextModel::create(&spinal_text, Some("scala"), None).unwrap();
                true
            },
            Msg::LoadExample() => {
                self.model_verilog = TextModel::create(&(example()), Some("verilog"), None).unwrap();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //let link = ctx.link();

        let window = web_sys::window().expect("should have a window in this context");
        let win_height = window.inner_height().unwrap().as_f64().unwrap()-64.0;
        //console::log_1(&format!("height2 {:?}",win_height).into());

        html! {
            <div style={"height:".to_owned()+&win_height.to_string()+"px"}>
                <button onclick={ctx.link().callback(|_| Msg::Run())}>{"run"}</button>
                <button onclick={ctx.link().callback(|_| Msg::LoadExample())}>{"load example"}</button>
                <CodeEditor options={ self.option_verilog.to_sys_options() } model={Some(self.model_verilog.clone())} classes="monacoleft"/>
                <CodeEditor options={ self.option_spinal.to_sys_options() } model={Some(self.model_spinal.clone())} classes="monacoright"/>
            </div>
        }
    }
}

#[test]
fn test() {
    println!("{}", convert(&example()))
}
