use std::io;
use std::result::Result;
use std::boxed::Box;
use std::string::String;
use std::collections::HashMap;


// ======================== //
// Cli callback return type //
// ======================== //

type CliResult = Result<String,String>;


// ============== //
// Cli State Type //
// ============== //

struct Cli<'tether, Data> {
    data: Data,
    callbacks: HashMap<String, Box<Fn(&mut Data, &[&str]) -> CliResult + 'tether>>
}


// ================== //
// Cli Implementation //
// ================== //
// -------
// - new -
// -------
//   instantiate new Cli State
// -----------------
// - (add/del)_cmd -
// -----------------
//   add or remove callback function from callback HashMap
// -----------
// - process -
// -----------
//   takes command line arguments and passes them to callback function
//   - first arg:    callback HashMap key
//   - rest of args: arguments to callback function (all strings)
// -------
// - run -
// -------
//   runs the cli
//   - reads input from STDIN in a loop
//   - passes input (on crg ret) to process to get and run callback function
impl <'tether, Data> Cli<'tether, Data> {
    pub fn new(data: Data) -> Cli<'tether, Data > {
        Cli { data: data, callbacks: HashMap::new() }
    }

    pub fn add_cmd<Func>(&mut self, name: &str, callback: Func)
    where Func: Fn(&mut Data, &[&str]) -> CliResult+ 'tether {
        self.callbacks.insert(name.to_string(), Box::new(callback));
        println!("Added command {}", name);
    }

    pub fn del_cmd(&mut self, name: &str) {
        match self.callbacks.get(name) {
            Some(_) => {
                self.callbacks.remove(name);
                println!("Removed command {}", name);
            },
            None => println!("Command {} does not exist", name)
        }
    }

    pub fn process(&mut self, line: &str) -> CliResult {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 0{
            return Ok("".to_string())
        }
        match self.callbacks.get(parts[0]) {
            Some(callback) => callback(&mut self.data, &parts[1..]),
            None => Err("No such command".to_string())
        }
    }

    pub fn run(&mut self) {
        let mut buff = String::new();
        while io::stdin().read_line(&mut buff).expect("error") > 0 {
            {
                let line = buff.trim_left();
                let res = self.process(line);
                println!("{:?}", res);
            }
            buff.clear();
        }
    }
}


// ================= //
// Logging functions //
// ================= //
// - very handy
// - without this nothing would be piped to STDOUT

pub fn ok<Content: ToString>(s: Content) -> CliResult {
    Ok(s.to_string())
}

pub fn err<Content: ToString>(s: Content) -> CliResult {
    Err(s.to_string())
}








// ============= //
// Main Function //
// ============= //
// - for example only

fn main() {
    println!("This is the interactive prompt");
    println!("==============================");
    struct Data {
        answer: i32,
        location: i32
    };
    let mut cli = Cli::new(Data{answer: 12, location: 0});

    cli.add_cmd("select_i32", |data, args| {
        if args.len() == 0 { return err("Usage: go <data> <args>") }
        let mut found: bool = false;
        for (i, arg) in args.iter().enumerate() {
            match arg.parse::<i32>() {
                Ok(int) => {
                    found = true;
                    data.answer = int;
                    data.location = i as i32;
                },
                Err(e) => continue
            }
        }
        println!("got {:?}", args);
        if found {
            println!("found first i32 at {} in args", data.location);
        } else {
            println!("Did not find an i32 in args list.");
        }
        ok(data.answer)
    });

    cli.add_cmd("show", |data, _| {
       ok(data.answer)
    });

    cli.run();
}