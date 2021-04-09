#[allow(unused_must_use)]
#[allow(unused_variables)]

mod data;

extern crate structopt;

use crate::data::dababy::{DABABY, SUS_URL, LETS_GO_URL, SUS};
use std::{
    thread::sleep,
    time::Duration,
    io::{Read, Cursor}
};
use rodio::source::Source;
use structopt::StructOpt;
use colored::Colorize;
use reqwest::Error;
use rodio::{OutputStream, Decoder};
use rand::{thread_rng, Rng};
use std::borrow::Borrow;

#[derive(StructOpt)]
struct Opts {
    #[structopt(default_value = "created by cytolytic!", multiple = true)]
    /// LET'S GOOOOO
    message: Vec<String>,

    #[structopt(short = "b", long = "blue")]
    /// DABABY IN SUPER SAYAN BLUE
    blue: bool,
}

fn main() {
    let opts = Opts::from_args();
    let mut message = String::new();
    opts.message.iter().for_each(|x| {
        &message.push_str(x.as_str());
        &message.push_str(" ");
    });
    if message.contains("sus") {
        print_dababy(message.clone(), opts.blue, true);
        run_audio(&message, SUS_URL);
        return;
    }
    print_dababy(message.clone(), opts.blue, false);
    run_audio(&message, LETS_GO_URL);
}

fn print_dababy(message: String, blue: bool, sus:bool) {
    println!("\n{}{}", " ".repeat(20), "-".repeat(message.chars().count() + 4));
    println!("{}| {} |", " ".repeat(20), message);
    println!("{}{}", " ".repeat(20), "-".repeat(message.chars().count() + 4));
    println!(r"{}\ /", " ".repeat(21 + message.chars().count() / 2));
    println!(r"{}\ /", " ".repeat(21 + message.chars().count() / 2));
    if sus {
        println!("{}", SUS.red());
    }
    else if blue { println!("{}", DABABY.blue()); } else { println!("{}", DABABY); }
}

fn run_audio(message: &String, url: &'static str){
    let get_audio = |sus: bool| {
        let resp = match reqwest::blocking::get(url) {
            Ok(res) => res,
            Err(x) => {return ();}
        };
        let mut cursor = Cursor::new(resp.bytes().unwrap());
        let (_strm, strm_hnd) = OutputStream::try_default().unwrap();
        let src = Decoder::new(cursor).unwrap();
        strm_hnd.play_raw(src.convert_samples()).unwrap();
        let mut timer = 0;
        if sus {
            loop {
                println!("{}", "DU ".repeat(thread_rng().gen_range(1..20)).as_str().red());
                sleep(Duration::from_secs(1));
                if timer >= 72 {break;}
                timer += 1;
            }
        }
    };
    if message.to_lowercase().contains("go") {
        println!("{}", "'GO' SPOTTED IN THE TEXT".red());
        get_audio(false);
    } else if message.contains("sus") {
        println!("{}", "IMPOSTOR SUS".red());
        get_audio(true);
    }
}
