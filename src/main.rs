extern crate termion;

//This assumes only squares will be used, but I think thats fine
//These are for how many chars over you can see when going thourgh
//the maze
const X_SIGHT: usize = 10;
const Y_SIGHT: usize = 10;

const WALL_CHAR: char = 'X';
const OPEN_CHAR: char = '.';

fn main() {
    let filename = "testmaze.txt";

    let maze_raw = fs::read_to_string(filename)
        .expect("unable to read file");
    
    //println!("Has contets:\n{}", maze_raw);
    //
    as_maze(get_around(8, 8, parse_string_maze(maze_raw)));

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
    return vec_maze
}

fn get_around(x_location: usize, y_location: usize, bool_maze: Vec<Vec<bool>>) -> [[bool; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1] {
    let mut result = [[true; X_SIGHT * 2 + 1]; Y_SIGHT * 2 + 1];

    for x in 0..(Y_SIGHT * 2 + 1) {
        for y in 0..(X_SIGHT * 2 + 1) {
            println!("{} - {}, {} - {}", x, x_location, y, y_location);
            if (
                (((x_location+x).checked_sub(X_SIGHT) != None) && ((y_location+y).checked_sub(Y_SIGHT) != None)) &&
                ((bool_maze.len() + 2 > x) && bool_maze[0].len() > y)
                )
                {
                    result[x][y] = bool_maze[x_location + x - X_SIGHT][y_location + y - Y_SIGHT];
            }
        }
    }
    return result
}

fn as_maze(maze: [[bool; X_SIGHT * 2+1]; Y_SIGHT * 2 + 1]) -> () {
    for x in maze.iter() {
        for y in x.iter() {
            print!("{}", match y {true => WALL_CHAR, false => OPEN_CHAR});
        }
        println!();
    }
}
