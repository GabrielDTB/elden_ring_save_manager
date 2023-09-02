mod character;

use character::*;
use std::fs::read;
use std::io::{stdin, Stdin};

enum MenuReturns {
    Open(Vec<u8>),
    Quit,
    None,
}

fn menu(stdin: &Stdin, saves: &Vec<save::View<Vec<u8>>>) -> MenuReturns {
    use MenuReturns::*;
    println!("Main menu"); // TODO improve message
    loop {
        let mut input = String::new();
        println!("Actions here"); // TODO improve message
        stdin.read_line(&mut input).unwrap(); // Is this unwrap safe?
        match &*input.trim() {
            "o" => return Open(open_file(stdin)),
            "q" => return Quit,
            "p1" => println!("{}", saves.get(0).unwrap().general().steam_id().read()),
            "p2" => {
                let save = saves.get(0).unwrap().general();
                let character = save.character_4();
                let name = character.name();
                let (_, slice, _) = unsafe { name.align_to::<u16>() };
                // let mut thing = Vec::new();
                // for pair in name.chunks(2) {
                //     thing.push(((pair[0] as u16) << 8) | pair[1] as u16);
                // }

                println!("{}", String::from_utf16(slice).unwrap());
            }

            "p3" => println!(
                "{}",
                saves.get(0).unwrap().general().character_4().level().read()
            ),
            "p4" => println!(
                "{}",
                saves
                    .get(0)
                    .unwrap()
                    .general()
                    .character_4()
                    .play_time()
                    .read()
            ),
            // "p5" => println!("{}", saves.get(0).unwrap().character_1().class().read()),
            // "f" => {
            //     let save = saves.get(0).unwrap();
            //     let data = [
            //         save.character_1(),
            //         save.character_2(),
            //         save.character_3(),
            //         save.character_4(),
            //         save.character_5(),
            //         save.character_6(),
            //         save.character_7(),
            //         save.character_8(),
            //         save.character_9(),
            //         save.character_10(),
            //     ];
            //     let characters = [
            //         data[0].block_1(),
            //         data[1].block_1(),
            //         data[2].block_1(),
            //         data[3].block_1(),
            //         data[4].block_1(),
            //         data[5].block_1(),
            //         data[6].block_1(),
            //         data[7].block_1(),
            //         data[8].block_1(),
            //         data[9].block_1(),
            //     ];
            //     let mut index = 0;
            //     while index < 2621440 {
            //         match [
            //             characters[0][index],
            //             characters[1][index],
            //             characters[2][index],
            //             characters[3][index],
            //             characters[4][index],
            //             characters[5][index],
            //             characters[6][index],
            //             characters[7][index],
            //             characters[8][index],
            //             characters[9][index],
            //         ] {
            //             // [9, 8, 2, 0, 7, 4, 3, 6, 1, 5] => println!("{index}"),
            //             [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] => println!("{index}"),
            //             _ => {}
            //         };
            //         index += 1;
            //     }
            // }
            _ => continue,
        }
    }
}

fn open_file(stdin: &Stdin) -> Vec<u8> {
    loop {
        // let mut input = String::new();
        // println!("Enter the file path"); // TODO improve message
        // stdin.read_line(&mut input).unwrap(); // Is this unwrap safe?
        // match read(input.trim()) {
        //     Ok(contents) => return contents,
        //     Err(e) => println!("{}", e), // TODO add error message
        // }
        return read("/home/gabe/ER0000.sl2").unwrap();
    }
}

fn main() {
    let mut saves = Vec::new();
    let stdin = stdin();
    loop {
        match menu(&stdin, &saves) {
            MenuReturns::Open(contents) => {
                saves.push(save::View::new(contents));
            }
            MenuReturns::Quit => break,
            MenuReturns::None => continue,
        }
    }
}
