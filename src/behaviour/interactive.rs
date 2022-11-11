use std::{
    fs::File
};

use super::player::Player;

use dialoguer::{theme::ColorfulTheme, Select, Input, };

pub fn perform() {
    loop {
        let commands = vec!["Play file", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&commands)
            .default(0)
            .interact()
            .expect("error");
        println!("Selection: {}", commands[selection]);
        if selection == commands.len() - 1 {
            break;
        }

        let file_path = Input::<String>::new()
            .with_prompt("Binary to play")
            .interact_text()
            .expect("err");

        let width = Input::<usize>::new()
            .with_prompt("Width")
            .interact_text()
            .expect("msg");

        let height = Input::<usize>::new()
            .with_prompt("Height")
            .interact_text()
            .expect("msg");
        
        let framerate = Input::<u64>::new()
            .with_prompt("Framerate")
            .interact_text()
            .expect("msg");

        let f = File::open(file_path)
            .expect("msg");
        
        let player =Player::new(height, width, framerate, f);

        player.play().expect("msg");
    }
}
