use std::io::{stdin, stdout, Write};

fn main() {
    let mut s = String::new();
    let mut state = false;
    loop{
        s.clear();
        println!("Current state is {}", state);
        print!(">>> ");
        stdout().flush().unwrap(); //Obbligo il SO a stampare subito
        stdin().read_line(&mut s).unwrap();
        let v: Vec<&str> = s.trim()
            .split(" ")
            .filter(|x|{*x != ""})
            .collect();
        println!("v: {:?}", v);
        let s = &v[..];
        match s{
            [] => println!("Empty line"),
            ["!", .. ] => println!("Comment"),
            ["inc", x] if x.parse::<i32>().is_ok() => println!("increment"),
            [x] => println!("Single element: {}", x),
            [x,y] => println!("Two elements: {} {}", x,y),
            x => println!("Unknow command '{:?}'", x),
        }

        //println!(">{}<", s.trim()); //Trim toglie gli spazi iniziali e finali
        /*
        match s.trim().to_ascii_lowercase().as_str(){
            "on" => state = true,
            "off" => state = false,
            "toggle" => state = !state,
            x => println!("unknow command '{}'", x), //Nome variabile non riconosciuto
            //_ => () //Ritorna una tupal nulla
        }

        */
    }
}
