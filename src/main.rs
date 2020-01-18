extern crate termion;

//use std::io;
use std::fs;

//This assumes only squares will be used, but I think thats fine
//These are for how many chars over you can see when going thourgh
//the maze
//  THis was a bad idea, it will need to change once you reach 
//  the center section
//const X_SIGHT: usize = 16;
//const Y_SIGHT: usize = 16;
const STEP_LEN: usize = 1;

//const WALL_CHAR: char = 'X';
//const OPEN_CHAR: char = '.';

#[derive(Clone, Copy, PartialEq)]
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

enum Direction {
    Up,
    Right,
}
fn motion(d: &Direction) -> [usize; 2] { 
    //I think const 2 in the declaration is fine here because I think I won't need another dimension
    match d {
        Direction::Up    => [0,STEP_LEN],
        Direction::Right => [STEP_LEN,0],
        //Not including these, since in a sense they are special cases of Up and Right
        //  I also don't want to have to store this stuff as signed integers unless needed
        //Down  => [0,-STEP_LEN],
        //Left  => [-STEP_LEN,0],
    }
}

fn find_terr(maze_vec: Vec<Vec<Terrain>>, to_search: Terrain) -> (Vec<Vec<Terrain>>, (usize, usize)) {
    let mut i = 0;
    while i < maze_vec.len() {
        let mut j = 0;
        while j < maze_vec[i].len() {
            if maze_vec[i][j] == to_search {
                return (maze_vec, (i, j))
            }
            j += 1;
        }
        i += 1;
    }
    panic!("This character is not on the map");
}

//fn move_character(t: Terrain, d: Direction, for_or_back: bool, mut maze_vec: Vec<Vec<Terrain>>) -> Vec<Vec<Terrain>> {
//    let points = motion(&d);
//    let value = find_terr(maze_vec, t);
//    let mut maze_vec = value.0;
//    let x = (value.1).0;
//    let y = (value.1).1;
//
//    println!("{}, {}, {}, {}, {}", 
//             x + points[0] > 0 ,
//             x + points[0] < maze_vec.len() ,
//             y + points[1] > 0 ,
//             y + points[1] < maze_vec[0].len() ,
//             maze_vec[x + points[0]][y + points[1]] != Terrain::Wall  
//             );
//
//    if for_or_back {
//        if x + points[0] > 0 && x + points[0] < maze_vec.len() &&
//           y + points[1] > 0 && y + points[1] < maze_vec[0].len() &&
//           maze_vec[x + points[0]][y + points[1]] != Terrain::Wall {
//            maze_vec[x + points[0]][y + points[1]] = t;
//            maze_vec[x][y] = Terrain::Open;
//    }
//    else {
//        if x.checked_sub(points[0]) != None && x - points[0] < maze_vec.len() &&
//           y.checked_sub(points[1]) != None && y - points[1] < maze_vec[0].len() &&
//           maze_vec[x - points[0]][y - points[1]] != Terrain::Wall {
//            maze_vec[x - points[0]][y - points[1]] = t;
//            maze_vec[x][y] = Terrain::Open;
//           }
//        }
//    }
//    return maze_vec
//}



fn main() {
    let filename = "testmaze.txt";

    let maze_raw = fs::read_to_string(filename)
        .expect("unable to read file");

    let s = fs::read_to_string(filename).expect("dsf");
    
    let mut x = parse_string_maze(s);
    println!();
    //let mut x = get_around(7, 7, parse_string_maze(maze_raw));
    let s = find_terr(x, Terrain::Theseus);
    let x = s.0;
    let a = (s.1).0;
    let b = (s.1).1;

    as_maze(get_around(a, b, x, 3, 4));

    //as_maze_vec(move_character(Terrain::Theseus, 
    //                           Direction::Up,
    //                           false,
    //                           x));
}

//fn move_char(x_start, y_start, direction)

fn parse_string_maze(maze_raw: String) -> Vec<Vec<Terrain>> {
    let mut vec_maze: Vec<Vec<Terrain>> = vec![vec![]];
    let mut row = 0;

    for byte_char in maze_raw.chars() {
        match byte_char {

            //Newline
            '\n' => {row += 1; vec_maze.push(vec![])},

            'X' => vec_maze[row].push(Terrain::Wall),
            'O'  => vec_maze[row].push(Terrain::Open),
            '$'  => vec_maze[row].push(Terrain::Theseus),
            '?'  => vec_maze[row].push(Terrain::Minotaur),

            _  => panic!("Invalid Character {}", byte_char),
        }
    }
    vec_maze.pop(); //This removes the empty list at the end
    return vec_maze
}

fn get_around(x_location: usize, y_location: usize, terr_maze: Vec<Vec<Terrain>>, x_sight: usize, y_sight: usize) -> Vec<Vec<Terrain>> {
    let mut result = vec![vec![Terrain::Wall; x_sight * 2 + 1]; y_sight * 2 + 1];


    for x in 0..(y_sight * 2 + 1) {
        for y in 0..(x_sight * 2 + 1) {
            //println!("{} - {}, {} - {}", x, x_location, y, y_location);
            //println!("{}", x_location + x - X_SIGHT < terr_maze.len()); 

            println!("{}, {}, leny = {}, lenx = {}", x, y, terr_maze[y].len(), terr_maze[x].len());
            println!("x calc = {}", (x_location as i32) + (x as i32) - (x_sight as i32));
            println!("y calc = {}", (y_location as i32) + (y as i32) - (y_sight as i32));
            println!("x ch = {}, y ch = {}", (x_location+x).checked_sub(x_sight) != None, ((y_location+y).checked_sub(y_sight) != None));

            if 
                ((x_location+x).checked_sub(x_sight) != None) && ((y_location+y).checked_sub(y_sight) != None) &&
                (x_location + x + x_sight < terr_maze.len()) &&  
                  y_location + y + y_sight < terr_maze[y].len() 
                {
                    result[x][y] = terr_maze[x_location + x + x_sight][y_location + y + y_sight];
            }
        }
    }
    return result
}

//I know I should use generics for these, but I'm not sure how
fn as_maze(maze: Vec<Vec<Terrain>>) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", get_terr_char(y));
        }
        println!();
    }
}
//fn as_maze(maze: [[Terrain; X_SIGHT * 2+1]; y_sight * 2 + 1]) -> () {
//    for x in maze.iter() {
//        for y in x.iter() {
//            print!("{}", get_terr_char(y));
//        }
//        println!();
//    }
//}
