extern crate rand; // import an external crate called rand 
extern crate termion;
use std::{env, thread, time}; // import thread and time standard libraries 
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;


// let yes_no: bool = true; // weird way to define a variable but there we go 
fn census(_world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;
    for i in 0..74 {
        for j in 0..74 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(world: [[u8; 75]; 75]) -> [[u8; 75]; 75] {
    let mut newworld = [[0u8; 75]; 75];

    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;

            if i > 0 {
                count += world[i-1][j];
            }
            if j > 0 {
                count += world[i][j-1];
            }
            if i < 74 {
                count += world[i+1][j];
            }
            if j < 74 {
                count += world[i][j+1];
            }
            if i > 0 && j < 74 {
                count += world[i-1][j+1];
            }
            if i < 74 && j > 0 {
                count += world[i+1][j-1];
            }
            if i > 0 && j > 0 {
                count += world[i-1][j-1];
            }
            if i < 74 && j < 74 {
                count += world[i+1][j+1];
            }

            newworld[i][j] == 0;

            if world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1;
            }

            if world[i][j] == 0 && (count == 3) {
                newworld[i][j] = 1;
            }
        }
    }
    newworld
}
fn main() {
    let mut world = [[0u8; 75]; 75]; // populate with a zero value, which is unsigned 8 bit
    let mut generations = 0; // mut means mutable, meaning we are gonig to change it 

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {         // if we don't pass arguments to the program, do it randomly 
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            } 
        }
    }
}