#[allow(unused_must_use)]
#[allow(unused_variables)]

mod data;

extern crate structopt;

use crate::data::dababy::{DABABY, URL};
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
    print_dababy(message.clone(), opts.blue);
    run_audio(&message, URL);
}

fn print_dababy(message: String, blue: bool) {
    println!("\n{}{}", " ".repeat(20), "-".repeat(message.chars().count() + 4));
    println!("{}| {} |", " ".repeat(20), message);
    println!("{}{}", " ".repeat(20), "-".repeat(message.chars().count() + 4));
    println!(r"{}\ /", " ".repeat(21 + message.chars().count() / 2));
    println!(r"{}\ /", " ".repeat(21 + message.chars().count() / 2));
    if blue { println!("{}", DABABY.blue()); } else { println!("{}", DABABY); }
}

fn run_audio(message: &String, url: &'static str){
    if message.to_lowercase().contains("go") {
        let resp = match reqwest::blocking::get(url) {
            Ok(res) => res,
            Err(x) => {return ();}
        };
        let mut cursor = Cursor::new(resp.bytes().unwrap());
        println!("{}", "'GO' SPOTTED IN THE TEXT".red());
        let (_strm, strm_hnd) = OutputStream::try_default().unwrap();
        let src = Decoder::new(cursor).unwrap();
        strm_hnd.play_raw(src.convert_samples()).unwrap();
        sleep(Duration::from_secs(4));
    }
}
