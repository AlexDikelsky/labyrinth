extern crate termion;

use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::fs;
use std::env;

const STEP_LEN: usize = 1;
const SWORD_LEN: usize = 2;

//{{{Main
fn main() {
    //This reads in the maze file
    let filename = "min_test.txt";

    let args: Vec<String> = env::args().collect();

    let mut maze_raw = String::new();

    let mut print_debug = false;
    if args.len() > 1 {
        maze_raw = fs::read_to_string(args[1].clone())
            .expect("Type `cargo run <file>` to choose a maze");

        if args.len() > 2 {
            print_debug = true;
        }
    } else {
        maze_raw = fs::read_to_string("testmaze.txt")
            .expect("Please supply a maze filename");
    }



    let mut current_maze = parse_string_maze(maze_raw);

    //Prepares the maze
    let mut found = false;
    let mut min_slain = false;
    let mut theseus = Character {
        loc           : find_terr(&current_maze, Terrain::Theseus).unwrap(),
        legal_terrain : vec![Terrain::Open],
        terr          : Terrain::Theseus,
        shape         : '$',
    };
    let start_point = find_terr(&current_maze, Terrain::Theseus).unwrap();
    let mut minotaur = Character {
        loc           : find_terr(&current_maze, Terrain::Minotaur).unwrap(),
        legal_terrain : vec![Terrain::Open, Terrain::Theseus],
        terr          : Terrain::Minotaur,
        shape         : '?',
    };


    //Run the maze
    let mut turn = 0;
    while !found {
        turn += 1;
        //The turn % n thing is here so the minotaur doesn't go too fast

        current_maze = match (closer_than_n(&minotaur, &theseus, 4) && (turn % 5 != 0)) && !min_slain {
            true => {
                let m = move_character(best_direction(&minotaur, &theseus), &mut current_maze, &mut minotaur);
                match m {
                    Some(t) => t,
                    None    => go_in_valid_direction(&mut current_maze, &mut minotaur),
                }
            },
            false => current_maze,
        };

        if print_debug {println!("Before Movement ↓");}
        as_maze(get_around(theseus.loc.0, theseus.loc.1, &current_maze, 4, 4));

        current_maze = clear_terr(&mut current_maze, Terrain::Sword, Terrain::Open);

        if closer_than_n(&theseus, &minotaur, 1) {
            println!("You were eaten by the Minotaur");
            break;
        }

        let action = read_action();

        let mut current_maze = match action.0 {
            true  => stab(action.1, &mut current_maze, &theseus),
            false => {
                let m = move_character(action.1, &mut current_maze, &mut theseus);
                match m {
                    Some(t) => t,
                    None => current_maze.clone(),
                }
            }
        };


        //as_maze(get_around(theseus.loc.0, theseus.loc.1, &current_maze, 4, 4));
        
        if print_debug {println!("Best direction: {:?}", best_direction(&theseus, &minotaur));}
        //This is the clear incantation
        if !print_debug {println!("{}[2J", 27 as char);}

        //as_maze(get_around(theseus.loc.0, theseus.loc.1, &current_maze, 4, 4));
        //  I thing this print is too late because it makes the min seem further
        //  away than it really is
        if print_debug {println!("After movement: up");}

        if find_terr(&current_maze, Terrain::Minotaur) == None && !min_slain {
            println!("You have slain the minotaur!");
            println!("Now return to the entrance you came from");
            //Changing the loc to be off the board is probably not the best way to solve this
            //problem
            minotaur.loc = (usize::max_value(), usize::max_value());
            min_slain = true;
        }
        if theseus.loc == start_point && min_slain {
            println!("You win!");
            break;
        }
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

fn get_terr_char(t: &Terrain) -> colored::ColoredString {
    match t {
        Terrain::Wall => "X".bright_white(),
        Terrain::Open => " ".white(),
        Terrain::Theseus => "T".blue().bold(),
        Terrain::Minotaur => "M".red().bold(),
        Terrain::Sword => "%".yellow(),
    }
}
//}}}
//{{{ Direction
#[derive(Clone, Copy, PartialEq, Debug)]
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
fn move_character(d: Direction, maze: &mut Vec<Vec<Terrain>>, mut character: &mut Character) -> Option<Vec<Vec<Terrain>>> {
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

            //{{{
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
            //}}}

        if destination[0] < maze.len() &&
            destination[1] < maze[0].len() &&
            character.legal_terrain.clone().contains(&maze[destination[0]][destination[1]])
            {
                let swap = maze[destination[0]][destination[1]];
                maze[destination[0]][destination[1]] = character.terr;
                maze[x][y] = swap;
                character.loc = (destination[0], destination[1]);
                return Some(maze.to_owned());
        }
    }
    None
}
//}}}
//{{{ Minotaur
fn closer_than_n(this: &Character, other: &Character, n: usize) -> bool {
    if ((this.loc.0 as isize) - (other.loc.0 as isize)).abs() < (n as isize) &&
            ((this.loc.1 as isize) - (other.loc.1 as isize)).abs() < (n as isize) {
        return true
    }
    return false
}

//This function tries all 4 directions to see if 
fn go_in_valid_direction(mut current_maze: &mut Vec<Vec<Terrain>>, mut character: &mut Character) -> Vec<Vec<Terrain>> {
    let mut rng = thread_rng();
    let mut list = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    list.shuffle(&mut rng);
    for d in list.iter() {
        let test_maze = move_character(*d, &mut current_maze, &mut character);
        if test_maze == None {
            continue
        } else {
            return test_maze.unwrap().to_owned()
        }
    }
    return current_maze.to_owned()
}

fn best_direction(this: &Character, other: &Character) -> Direction {
    let left_right: isize = (this.loc.1 as isize) - (other.loc.1 as isize);
    let up_down: isize = (this.loc.0 as isize) - (other.loc.0 as isize);
    let mut result = Direction::Left;
    //kprintln!("x dist = {}, y dist = {}", left_right, up_down

    if left_right.abs() > up_down.abs() {
        if left_right < 0 {
            result = Direction::Right;
        } else {
            result = Direction::Left;
        }
    } else {
        if up_down < 0 {
            result = Direction::Down;
        } else {
            result = Direction::Up;
        }
    }
    return result
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
            print!("{} ", get_terr_char(y));
        }
        println!();
    }

    return maze
}
//}}}
