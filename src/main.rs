use rusty_life::lib::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use text_block_macros::text_block;
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");
    let json = read_file("world.json");
    let mut world: World = match serde_json::from_str(&json) {
        Ok(w) => w,
        Err(e) => panic!("{}", e),
    };
    let text = text_block! {
        "abc"
        "def"
        "ghi"
    };
    world.draw(20, 20);
    println!("{}", text);
    simulate_world(&mut world);
    world.draw(20, 20);
    simulate_world(&mut world);
    world.draw(20, 20);
}

fn simulate_world(world: &mut World) {
    let mut cells_add: Vec<Vec2> = vec![];
    let mut cells_remove: Vec<Vec2> = vec![];

    for cell in world.cells.iter() {
        // die simulation
        println!("{:?}", cell.clone());
        let mut i = 0;
        if world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x,
            y: cell.y + 1,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x - 1,
            y: cell.y + 1,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x - 1,
            y: cell.y,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x - 1,
            y: cell.y - 1,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x,
            y: cell.y - 1,
        }) != None
        {
            i += 1;
        }
        if world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y - 1,
        }) != None
        {
            i += 1;
        }

        if i < 4 && i > 1 || i == 0 {
            cells_remove.push(cell.clone());
        }
        println!("{}", i);

        // spawn simulation

        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            cells_add.push(Vec2 {
                x: cell.x + 1,
                y: cell.y,
            });
        }

        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }

        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }

        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }
        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }
        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }
        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }
        if (world.cells.get(&Vec2 {
            x: cell.x + 1,
            y: cell.y + 1,
        }) != None)
            && world.cells.get(&Vec2 {
                x: cell.x + 1,
                y: cell.y - 1,
            }) != None
        {
            //  cells_add.push(Vec2 { x: cell.x+1, y: cell.y });
        }
    }

    for cell in cells_remove {
        world.cells.remove(&cell);
        println!("{:?}", cell.clone());
    }

    for cell in cells_add {
        world.cells.insert(cell);
    }
}

fn read_file(file_name: &str) -> String {
    let mut file = match File::open(&file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", file_name, why),
        Ok(_) => print!("{} contains:\n{}", file_name, s),
    }
    s
}

fn write_to_file(str: &str, file_name: String) {
    let mut file = match File::create(&file_name) {
        Err(why) => panic!("couldn't create {}: {}", &file_name, why),
        Ok(file) => file,
    };

    match file.write_all(str.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", &file_name, why),
        Ok(_) => println!("successfully wrote to {}", file_name),
    }
}
