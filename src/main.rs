extern crate termion;

use std::io;
use std::fs;

const STEP_LEN: usize = 1;
const SWORD_LEN: usize = 2;

//{{{Main
fn main() {
    //This reads in the maze file
    let filename = "testmaze.txt";

    let maze_raw = fs::read_to_string(filename)
        .expect("unable to read file");

    let mut current_maze = parse_string_maze(maze_raw);

    //Prepares the maze
    let mut found = false;
    let mut theseus = Character {
        loc           : find_terr(&current_maze, Terrain::Theseus).unwrap(),
        legal_terrain : vec![Terrain::Open],
        terr          : Terrain::Theseus,
        shape         : '$',
    };

    //Run the maze
    while !found {
        let action = read_action();

        println!("direction = {:?}", match action.1 { 
            Direction::Up => "Up", 
            Direction::Down => "Down", 
            Direction::Right => "Right",
            Direction::Left => "Left",
        });

        let mut current_maze = match action.0 {
            true  => stab(action.1, &mut current_maze, &theseus),
            false => move_character(action.1, &mut current_maze, &mut theseus),
        };
        //This is the clear incantation
        //println!("{}[2J", 27 as char);
        as_maze(get_around(theseus.loc.0, theseus.loc.1, &current_maze, 4, 4));
        current_maze = clear_terr(&mut current_maze, Terrain::Theseus, Terrain::Open);
    }
}
//}}}
//{{{ Structs & Enums
//{{{Character
struct Character {
    loc: (usize, usize),
    legal_terrain: Vec<Terrain>,
    terr: Terrain,
    shape: char,
}
//}}}
//{{{Terrain 
#[derive(Clone, Copy, PartialEq)]
enum Terrain {
    Wall,
    Open,
    Theseus,
    Minotaur,
    Sword,
}

fn get_terr_char(t: &Terrain) -> char {
    match t {
        Terrain::Wall => 'X',
        Terrain::Open => '.',
        Terrain::Theseus => '$',
        Terrain::Minotaur => '?',
        Terrain::Sword => '%',
    }
}
//}}}
//{{{ Direction
#[derive(Clone, Copy, PartialEq)]
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
//}}}
//{{{Map-changing functions
//{{{ Sword
fn stab(d: Direction, maze: &mut Vec<Vec<Terrain>>, character: &Character) -> Vec<Vec<Terrain>>{
    //Starts at 1 because don't replace yourself
    let mut i = 1;
    let legal_terrain: Vec<Terrain> = vec![Terrain::Open, Terrain::Minotaur];

    match d {
        Direction::Up    => { 
            while (character.loc.0).checked_sub(i) != None && 
            i < SWORD_LEN + 1 &&
            legal_terrain.contains(&maze[character.loc.0 - i][character.loc.1]) {
                maze[character.loc.0 - i][character.loc.1] = Terrain::Sword;
                i += 1;
            }
            maze.to_owned()
        }
        Direction::Left  => {
            while (character.loc.1).checked_sub(i) != None && i < SWORD_LEN + 1 &&
                legal_terrain.contains(&maze[character.loc.0][character.loc.1 - i]) {
                maze[character.loc.0][character.loc.1 - i] = Terrain::Sword;
                i += 1;
            }
            maze.to_owned()
        }
        Direction::Down  => {
            while character.loc.0 + i < maze[0].len() && i < SWORD_LEN + 1 &&
                legal_terrain.contains(&maze[character.loc.0 + i][character.loc.1]) {
                maze[character.loc.0 + i][character.loc.1] = Terrain::Sword;
                i += 1;
            }
            maze.to_owned()
        }
        Direction::Right => {
            while character.loc.1 + i < maze.len() && i < SWORD_LEN + 1 &&
                legal_terrain.contains(&maze[character.loc.0][character.loc.1 + i]) {
                maze[character.loc.0][character.loc.1 + i] = Terrain::Sword;
                i += 1;
            }
            maze.to_owned()
        }
    }
}
fn clear_terr<T>(two_d_vec: &mut Vec<Vec<T>>, item_to_remove: T, item_to_swap_to: T) -> Vec<Vec<T>> 
    where T: PartialEq + Clone + Copy
{
    for i in 0..two_d_vec.len() {
        for j in 0..two_d_vec[i].len() {
            if two_d_vec[i][j] == item_to_remove {
                //println!("{}, {}", i, j);
                two_d_vec[i][j] = item_to_swap_to.clone();
            }
        }
    }
    two_d_vec.to_owned()
}
    /*  This code doesn't work because it allows you to stab 'over' terrain {{{
     * match d {
        Direction::Up    => fill(
            vec![
                (Some(&character.loc.0), &character.loc.1.checked_sub(1)),
                (Some(&character.loc.0), &character.loc.1.checked_sub(2)),
            ],
            Terrain::Sword
        );
        Direction::Right => fill(
            vec![
                (&character.loc.0 + 1, &character.loc.1),
                (&character.loc.0 + 2, &character.loc.1),
            ],
            Terrain::Sword
        );
        Direction::Down  => fill(
            vec![
                (&character.loc.0, &character.loc.1 + 1),
                (&character.loc.0, &character.loc.1 + 2),
            ],
            Terrain::Sword
        );
        Direction::Left  => fill
            vec![
                (&character.loc.0.checked_sub(1), Some(&character.loc.1)),
                (&character.loc.0.checked_sub(2), Some(&character.loc.1)),
            ],
            Terrain::Sword
        );
    }
    //}}}
    */

//}}}
//{{{ Movement
fn move_character(d: Direction, maze: &mut Vec<Vec<Terrain>>, mut character: &mut Character) -> Vec<Vec<Terrain>> {
    let location = motion(&d);
    let points = [(location.0)[0], (location.0)[1]];
    let x = character.loc.0;
    let y = character.loc.1;

        if (x.checked_sub(points[0]) != None && y.checked_sub(points[1]) != None) || location.1 == '+' {
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

            //println!("dest = {:?}", destination);
            //println!("loc  = {:?}", (x, y));

            if destination[0] < maze.len() &&
                destination[1] < maze[0].len() &&
                character.legal_terrain.clone().contains(&maze[destination[0]][destination[1]])
                {
                    let swap = maze[destination[0]][destination[1]];
                    maze[destination[0]][destination[1]] = character.terr;
                    maze[x][y] = swap;
                    character.loc = (destination[0], destination[1]);
        }
    }
    return maze.to_owned()
}
//}}}
//}}}
//{{{ Input

fn find_terr(maze_vec: &Vec<Vec<Terrain>>, to_search: Terrain) -> Option<(usize, usize)> {
    let mut i = 0;
    while i < maze_vec.len() {
        let mut j = 0;
        while j < maze_vec[i].len() {
            if maze_vec[i][j] == to_search {
                return Some((i, j))
            }
            j += 1;
        }
        i += 1;
    }
    None
}

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
fn read_action() -> (bool, Direction) {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        match guess.chars().next().unwrap() {
            'u' => {return (false, Direction::Up)},
            'd' => {return (false, Direction::Down)},
            'l' => {return (false, Direction::Left)},
            'r' => {return (false, Direction::Right)},
            's' => {
                println!("Choose a direction to attack:"); 
                return (true, read_action().1)
                     }
            _   => {println!("Please enter \n`u' for up,\n`d' for down,\n`l' for left,\n`r' for right");},
        }
    }
}
//}}}
//{{{Output
fn get_around(x_location: usize, y_location: usize, terr_maze: &Vec<Vec<Terrain>>, x_sight: usize, y_sight: usize) -> Vec<Vec<Terrain>> {
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
