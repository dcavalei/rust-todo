use std::collections::HashMap;
use std::path::Path;
use todo::container::{Entry, IContainer, persistency::Storage};
use todo::error::Result;

pub const fn help_message() -> &'static str {
    "
Usage: todo <option> [<argument>] ...
    Options:
        {-a|--add} <name> <content>
            Add a new entry

        {-d|--delete} <id>
            Delete an entry

        {-f|--find} <name>
            Find an entry by name

        --find-id <id>
            Find and entry by id

        --help
            Display this message
"
}

pub enum Options<'a> {
    Help,
    Add(Entry),
    Delete(u32),
    Find(&'a str),
    FindId(u32),
    Unknown(&'a str),
}

pub fn option_handler(opt: Options, container: &mut impl IContainer<Item=Entry>) -> Result<()> {
    match opt {
        Options::Help => {
            print!("{}", help_message());
            Ok(())
        }
        Options::Add(entry) => container.add(entry),
        Options::Delete(id) => container.delete(id),
        Options::Find(name) => {
            let entry = container.find(|entry| entry.name == name)
                .ok_or(format!("Error: NAME {} does not exist!", name))?;
            println!("FIND: {:?}", entry);
            Ok(())
        }
        Options::FindId(id) => {
            let entry = container.find_by_id(id)
                .ok_or(format!("Error: ID {} does not exist!", id))?;
            println!("FIND_ID: {:?}", entry);
            Ok(())
        }
        Options::Unknown(arg) => Err(format!("Unknown Option: {}", arg))?,
    }
}


// Some things might not make much sense from a architecture PoV, but the project main purpose is to learn what rust has
// to offer regarding syntax and std library features (no crates included).
fn main() -> Result<()>
{
    // TODO: Split in 2 types of options: pre-execution and execution
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        print!("{}", help_message());
        return Ok(());
    }

    let mut container = HashMap::<u32, Entry>::new();
    let mut next_id = 0_u32;
    // let mut storage = Storage::new()?;
    let mut storage = Storage::from_path(Path::new(".todo_storage"))?;

    let mut it = args.iter().skip(1);
    while let Some(arg) = it.next() {
        let opt: Options = match arg.as_str() {
            "--help" => Options::Help,
            "--add" | "-a" => {
                let name = it.next().ok_or("<name> expected: todo {-a|--add} <name> <content>")?;
                let content = it.next().ok_or("<content> expected: todo {-a|--add} <name> <content>")?;
                next_id += 1;
                Options::Add(Entry { id: next_id, name: name.clone(), content: content.clone() })
            }
            "--delete" | "-d" => {
                let id = it.next().ok_or("<id> expected: todo {-d|--delete} <id>")?;
                let id: u32 = id.trim().parse()
                    .or_else(|_| Err(format!("'{}' is not a valid <id>", id)))?; // this way format! is not expanded if not used
                Options::Delete(id)
            }
            "--find" | "-f" => {
                let name = it.next().ok_or("<name> expected: todo {-f|--find} <name>")?;
                Options::Find(name)
            }
            "--find-id" => {
                let id = it.next().expect("<id> expected: todo --find-id <id>");
                let id: u32 = id.trim().parse()
                    .or_else(|_| Err(format!("'{}' is not a valid <id>", id)))?;
                Options::FindId(id)
            }
            _ => Options::Unknown(arg)
        };
        option_handler(opt, &mut container)?;
    }
    println!("Container content: {:?}", container);
    println!("{:?}", storage);
    Ok(())
}
