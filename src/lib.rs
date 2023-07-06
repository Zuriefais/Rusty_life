pub mod lib {
    use hashbrown::*;
    use serde::{Deserialize, Serialize};
    

    #[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Debug)]
    pub struct Vec2 {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct World {
        pub cells: HashSet<Vec2>,
    }

    impl World {
        pub fn simulate(&self) {
            todo!()
        }

        pub fn draw(&self, width: i32, height: i32) {
            //clearscreen::clear().unwrap();
            let mut printed_world = "".to_string();
            for y in 0..height {
                let mut line = "\n".to_string();
                for x in 0..width {
                    let pos = Vec2 { x, y };
                    let s = match self.cells.get(&pos) {
                        Some(_) => "□",
                        None => "0",
                    };
                    line += s;
                }
                printed_world += &line;
            }
            println!("{}", printed_world);
        }
    }
}
