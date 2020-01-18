extern crate termion;

//use std::io;
use std::fs;

const STEP_LEN: usize = 1;

fn main() {
    let filename = "testmaze.txt";

    let maze_raw = fs::read_to_string(filename)
        .expect("unable to read file");

    let mut x = parse_string_maze(maze_raw);
    println!();
    let d = x.clone();
    as_maze(d);
    println!();

    //Down works
    as_maze(move_character(Terrain::Theseus, Direction::Left, 
    move_character(Terrain::Theseus, Direction::Left,
        (move_character(Terrain::Theseus, Direction::Up, x))
        )));
    //let mut x = get_around(7, 7, parse_string_maze(maze_raw));
    //let s = find_terr(x, Terrain::Theseus);
    //let x = s.0;
    //let a = (s.1).0;
    //let b = (s.1).1;

    //let s = x.clone();
    //let t = x.clone();
    //as_maze(get_around(a, b, x, 1, 5));
    //as_maze(get_around(0, 0, s, 4, 3));
    //as_maze(get_around(15, 15, x, 2, 2));
    //
    //  This one still crashes ↓
    //  Probably has something to do with 7 being a "high" number
    //as_maze(get_around(12, 12, t, 0, 7));
}

//{{{Notes
//  Can't use constant x_sights because when you enter the center of the
//  maze, your vision increases.

//const WALL_CHAR: char = 'X';
//const OPEN_CHAR: char = '.';
//}}}
//{{{Terrain 
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
//}}}
//{{{ Direction
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn motion(d: &Direction) -> ([usize; 2], char) { 
    //I think using arrays here is fine because only 2 dimentions will be needed
    //I have 'f' and 'b' to show which direction they are going because STEP_LEN
    //has to be a usize.
    match d {
        Direction::Right => ([0,STEP_LEN],  '+'),
        Direction::Down  => ([STEP_LEN, 0],  '+'),

        Direction::Up    => ([STEP_LEN,0],  '-'),
        Direction::Left  => ([0, STEP_LEN],  '-'),
    }
}
//}}}
//{{{
fn move_character(t: Terrain, d: Direction, mut maze_vec: Vec<Vec<Terrain>>) -> Vec<Vec<Terrain>> {
    let location = motion(&d);
    let value = find_terr(maze_vec, t);
    let mut maze_vec = value.0;
    let points = [(location.0)[0], (location.0)[1]];
    let x = (value.1).0;
    let y = (value.1).1;

    let destination = {
            if location.1 == '-' {
                [x - points[0], y - points[1]]
            } else {
                [x + points[0], y + points[1]]
            }
    };

    //println!("{}, {}, {}, {}, {}", 
    //         x + points[0] > 0 ,
    //         x + points[0] < maze_vec.len() ,
    //         y + points[1] > 0 ,
    //         y + points[1] < maze_vec[0].len() ,
    //         maze_vec[x + points[0]][y + points[1]] != Terrain::Wall  
    //         );

    //maze_vec[x + points[0]][y + points[1]] = t;
    //maze_vec[x - points[0]][y + points[1]] = t;

    println!("{:?}", destination);

    if destination[0] > 0 && 
        destination[0] < maze_vec.len() &&
        destination[1] > 0 &&
        destination[1] < maze_vec[0].len() &&
        maze_vec[destination[0]][destination[1]] == Terrain::Open
        {
            println!("s1df");
            let swap = maze_vec[destination[0]][destination[1]];
            maze_vec[destination[0]][destination[1]] = t;
            println!("sdf");

            maze_vec[x][y] = swap;
        }
    return maze_vec
}
//}}}
//{{{Maze generation
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
//}}}
//{{{Output
fn get_around(x_location: usize, y_location: usize, terr_maze: Vec<Vec<Terrain>>, x_sight: usize, y_sight: usize) -> Vec<Vec<Terrain>> {
    let mut result = vec![vec![Terrain::Wall; y_sight * 2 + 1]; x_sight * 2 + 1];

    //println!("{} = x_l, {} = y_l, {} = x_si, {} = y_si", x_location, y_location, x_sight, y_sight);

    for x in 0..(result.len()) {
        for y in 0..(result[x].len()) {
            //println!("{} - {}, {} - {}", x, x_location, y, y_location);
            //println!("{}", x_location + x - X_SIGHT < terr_maze.len()); 

            //println!("{}, {}, leny = {}, lenx = {}", x, y, terr_maze[y].len(), terr_maze[x].len());
            //println!("x calc = {}", (x_location as i32) + (x as i32) - (x_sight as i32));
            //println!("y calc = {}", (y_location as i32) + (y as i32) - (y_sight as i32));
            //println!("x ch = {}, y ch = {}", (x_location+x).checked_sub(x_sight) != None, ((y_location+y).checked_sub(y_sight) != None));

            //println!("x upper =  {}, len={}", x_location + x - x_sight, terr_maze.len());
            //println!("y upper =  {}, len={}", y_location + y - y_sight, terr_maze.len());
            //println!();
            if 
                ((x_location+x).checked_sub(x_sight) != None) && ((y_location+y).checked_sub(y_sight) != None) &&
                ( x_location + x - x_sight < terr_maze.len()) &&  
                  y_location + y - y_sight < terr_maze[y].len() 
                {
                    result[x][y] = terr_maze[x_location + x - x_sight][y_location + y - y_sight];
                    //println!("{}, {}", x_location + x - x_sight, y_location + y - y_sight);
            }
        }
    }
    return result
}

fn as_maze(maze: Vec<Vec<Terrain>>) -> Vec<Vec<Terrain>> {
    let readable = maze.clone();

    for x in readable.iter() {
        for y in x.iter() {
            print!("{}", get_terr_char(y));
        }
        println!();
    }

    return maze
}
//}}}
