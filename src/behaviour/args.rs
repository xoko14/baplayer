use std::fs::File;

use super::player::Player;


pub fn perform(args: &Vec<String>){
    let width = args[2].parse::<usize>().expect("Could not parse width");
    let height = args[3].parse::<usize>().expect("Could not parse heigth");
    let framerate = args[4].parse::<u64>().expect("Could not parse framerate");
    let file_path = &args[1];

    let file = File::open(file_path).expect("Could not open file");
    let player = Player::new(height, width, framerate, file);
    player.play().expect("Error playing file")
}