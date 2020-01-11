extern crate termion;

use std::io;
use std::fs;

//This assumes only squares will be used, but I think thats fine
//These are for how many chars over you can see when going thourgh
//the maze
const X_SIGHT: usize = 1;
const Y_SIGHT: usize = 1;

//const WALL_CHAR: char = 'X';
//const OPEN_CHAR: char = '.';

#[derive(Clone, Copy)]
enum Terrain {
    Wall,
    Open,
    Theseus,
    Minotaur,
}

fn get_terr_char(t: &Terrain) -> char {
    match t {
        Terrain::Wall => 'X',
        Terrain::Open => '.',
        Terrain::Theseus => '$',
        Terrain::Minotaur => '?',
    }
}

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

fn parse_string_maze(maze_raw: String) -> Vec<Vec<Terrain>> {
    let mut vec_maze: Vec<Vec<Terrain>> = vec![vec![]];
    let mut row = 0;

    for byte_char in maze_raw.bytes() {
        match byte_char {

            //Newline
            10 => {row += 1; vec_maze.push(vec![])},

            //X
            88 => vec_maze[row].push(Terrain::Wall),

            //O
            79 => vec_maze[row].push(Terrain::Open),
            _  => panic!("Invalid Character {}", byte_char),
        }
    }
    vec_maze.pop(); //This removes the empty list at the end
    return vec_maze
}

fn get_around(x_location: usize, y_location: usize, terr_maze: Vec<Vec<Terrain>>) -> [[Terrain; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1] {
    let mut result = [[Terrain::Wall; X_SIGHT * 2 + 1]; Y_SIGHT * 2 + 1];

    for x in 0..(Y_SIGHT * 2 + 1) {
        for y in 0..(X_SIGHT * 2 + 1) {
            //println!("{} - {}, {} - {}", x, x_location, y, y_location);
            //println!("{}", x_location + x - X_SIGHT < terr_maze.len()); 
            if (
                (((x_location+x).checked_sub(X_SIGHT) != None) && ((y_location+y).checked_sub(Y_SIGHT) != None)) &&
                ((x_location + x - X_SIGHT < terr_maze.len()) &&  
                  y_location + y - Y_SIGHT < terr_maze[x].len()) 
                )
                {
                    result[x][y] = terr_maze[x_location + x - X_SIGHT][y_location + y - Y_SIGHT];
            }
        }
    }
    return result
}

//I know I should use generics for these, but I'm not sure how
fn as_maze_vec(maze: Vec<Vec<Terrain>>) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", get_terr_char(y));
        }
        println!();
    }
}
fn as_maze(maze: [[Terrain; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1]) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", get_terr_char(y));
        }
        println!();
    }
}
