mod market;
mod order;
mod user;

use crate::order::Order;
use crate::user::User;
use order::{OrderType, OrderTypeContainer};

use std::time::SystemTime;

extern crate argparse;

use argparse::{ArgumentParser, Store, StoreConst, StoreFalse, StoreTrue};

fn main() {
    let _user = User {
        name: String::from("kahshiuh"),
        balance: 100f64,
        order: vec![],
    };
    let _me = Order {
        user_id: String::from("Hello"),
        is_fufilled: false,
        price: 100f64,
        timestamp: SystemTime::now(),
        order_type: OrderTypeContainer {
            order_type: OrderType::Market,
            is_buy: true,
        },
    };
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
    if is_verbose {
        println!("HELLO")
    }
}
