mod market;
mod order;
mod user;
mod helper;

use crate::user::User;
use order::{OrderType, Order};
use market::{Stock};

use std::time::SystemTime;

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue};
//use yew::prelude::*;
//use yew::Renderer;
/* 
#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
*/
fn main() {
    struct Options {
        verbose: bool,
    }
    let mut is_verbose = false;
    let mut is_help = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("OME command-line utility. ");
        parser.refer(&mut is_verbose).add_option(
            &["-v"],
            StoreTrue,
            "Verbose, display debugging statements",
        );
        parser.refer(&mut is_help).add_option(
            &["-h"],
            StoreTrue,
            "Help, display options and arguments",
        );
        parser.parse_args_or_exit();
    }

    let _user = User {
        name: String::from("kahshiuh"),
        balance: 100f64,
        user_id: String::from("12345"),
        order: vec![],
    };
    let _me = Order {
        user_id: String::from("Hello"),
        order_id: String::from("aksdhkasd"),
        is_fufilled: false,
        price: 100f64,
        timestamp: SystemTime::now(),
        order_type: OrderType::Market,
        amount: 10,
        fufiller_id: None,
        is_buy: true,
    };
    //yew::Renderer::<App>::new().render();
}
