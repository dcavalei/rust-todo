use std::collections::{HashMap};
use std::io;
use todo::{Entry};

mod core {
    use std::io;
    use todo::{Entry, IContainer};

    pub enum Options<'a> {
        Help,
        Add(Entry),
        Delete(u32),
        Find(&'a str),
        FindId(u32),
        Unknown(&'a str),
    }

    pub const fn help_message() -> &'static str {
        "
Usage: todo --<option> <argument> [...]

    Options:
        --add               <name> <content>
"
    }

    pub fn option_handler(opt: Options, container: &mut impl IContainer<Item=Entry>) -> io::Result<()> {
        match opt {
            Options::Help => {
                print!("{}", help_message());
                Ok(())
            }
            Options::Add(entry) => container.add(entry),
            Options::Delete(id) => container.delete(id),
            Options::Find(name) => {
                let entry = container.find(|entry| entry.name == name)
                    .ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Error: NAME {} does not exist!", name)))?;
                println!("FIND: {:?}", entry);
                Ok(())
            }
            Options::FindId(id) => {
                let entry = container.find_by_id(id)
                    .ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Error: ID {} does not exist!", id)))?;
                println!("FIND_ID: {:?}", entry);
                Ok(())
            }
            Options::Unknown(arg) => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unknown Option: {}", arg))),
        }
    }
}

use core::*; // Disclaimer: I'm not brain damaged

// Some things might not make much sense from a architecture PoV, but the project main purpose is to learn what rust has
// to offer regarding syntax and std library features (no crates included).
fn main() -> io::Result<()>
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print!("{}", help_message());
        return Ok(());
    }

    let mut container = HashMap::<u32, Entry>::new();
    let mut next_id = 0_u32;

    let mut it = args.iter().skip(1);
    while let Some(arg) = it.next() {
        let opt: Options =
            if arg.eq("--help") {
                Options::Help
            } else if arg.eq("--add") || arg.eq("-a") {
                let name = it.next().expect("NAME expected");
                let content = it.next().expect("CONTENT expected");

                next_id += 1;
                Options::Add(Entry { id: next_id, name: name.clone(), content: content.clone() })
            } else if arg.eq("--delete") || arg.eq("-d") {
                let id = it.next().expect("ID expected");
                let id: u32 = id.trim().parse()
                    .unwrap_or_else(|_| panic!("Error: {} is not a valid ID", id));

                Options::Delete(id)
            } else if arg.eq("--find") || arg.eq("-f") {
                let name = it.next().expect("NAME expected");

                Options::Find(name)
            } else if arg.eq("--find-id") {
                let id = it.next().expect("ID expected");
                let id: u32 = id.trim().parse()
                    .unwrap_or_else(|_| panic!("Error: {} is not a valid ID", id));

                Options::FindId(id)
            } else {
                Options::Unknown(arg)
            };
        option_handler(opt, &mut container)?;
    }
    println!("Container content: {:?}", container);
    Ok(())
}
