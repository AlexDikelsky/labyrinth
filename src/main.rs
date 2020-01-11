extern crate termion;

use std::io;
use std::fs;

//This assumes only squares will be used, but I think thats fine
//These are for how many chars over you can see when going thourgh
//the maze
const X_SIGHT: usize = 8;
const Y_SIGHT: usize = 7;

const WALL_CHAR: char = 'X';
const OPEN_CHAR: char = '.';

fn main() {
    let filename = "testmaze.txt";

    let maze_raw = fs::read_to_string(filename)
        .expect("unable to read file");

    let s = fs::read_to_string(filename).expect("dsf");
    
    as_maze_vec(parse_string_maze(s));
    println!();
    as_maze(get_around(7, 7, parse_string_maze(maze_raw)));

    //println!("{:#?}", parse_string_maze(maze_raw));

}

fn parse_string_maze(maze_raw: String) -> Vec<Vec<bool>> {
    let mut vec_maze: Vec<Vec<bool>> = vec![vec![]];
    let mut row = 0;

    for byte_char in maze_raw.bytes() {
        match byte_char {

            //Newline
            10 => {row += 1; vec_maze.push(vec![])},

            //X
            88 => vec_maze[row].push(true),

            //O
            79 => vec_maze[row].push(false),
            _  => panic!("Invalid Character {}", byte_char),
        }
    }
    vec_maze.pop(); //This removes the empty list at the end
    return vec_maze
}

fn get_around(x_location: usize, y_location: usize, bool_maze: Vec<Vec<bool>>) -> [[bool; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1] {
    let mut result = [[true; X_SIGHT * 2 + 1]; Y_SIGHT * 2 + 1];

    for x in 0..(Y_SIGHT * 2 + 1) {
        for y in 0..(X_SIGHT * 2 + 1) {
            //println!("{} - {}, {} - {}", x, x_location, y, y_location);
            //println!("{}", x_location + x - X_SIGHT < bool_maze.len()); 
            if (
                (((x_location+x).checked_sub(X_SIGHT) != None) && ((y_location+y).checked_sub(Y_SIGHT) != None)) &&
                ((x_location + x - X_SIGHT < bool_maze.len()) &&  
                  y_location + y - Y_SIGHT < bool_maze[x].len()) 
                )
                {
                    result[x][y] = bool_maze[x_location + x - X_SIGHT][y_location + y - Y_SIGHT];
            }
        }
    }
    return result
}

fn as_maze_vec(maze: Vec<Vec<bool>>) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", match y {true => WALL_CHAR, false => OPEN_CHAR});
        }
        println!();
    }
}
fn as_maze(maze: [[bool; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1]) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", match y {true => WALL_CHAR, false => OPEN_CHAR});
        }
        println!();
    }
}
