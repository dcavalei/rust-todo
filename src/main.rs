use std::collections::{HashMap};
use std::io;

enum Options<'a> {
    Help,
    Add(Entry),
    Delete(u32),
    Find(&'a str),
    FindId(u32),
    Unknown(&'a str),
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Entry {
    id: u32,
    name: String,
    content: String,
}

impl Entry {}

const fn help_message() -> &'static str {
    "
Usage: todo --<option> <argument> [...]

    Options:
        --add               <name> <content>
"
}

fn option_handler(opt: Options, container: &mut impl IContainer<Item=Entry>) -> io::Result<()> {
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
            println!("{:?}", entry);
            Ok(())
        }
        Options::FindId(id) => {
            let entry = container.find_by_id(id)
                .ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Error: ID {} does not exist!", id)))?;
            println!("{:?}", entry);
            Ok(())
        }
        Options::Unknown(arg) => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unknown Option: {}", arg))),
    }
}

trait IContainer {
    type Item;
    fn add(&mut self, obj: Self::Item) -> io::Result<()>;
    fn delete(&mut self, id: u32) -> io::Result<()>;
    fn find<P>(&self, predicate: P) -> Option<&Self::Item>
        where P: Fn(&Self::Item) -> bool;
    fn find_mut<P>(&mut self, predicate: P) -> Option<&mut Self::Item>
        where P: Fn(&Self::Item) -> bool;
    fn find_by_id(&self, id: u32) -> Option<&Self::Item>;
    fn find_mut_by_id(&mut self, id: u32) -> Option<&mut Self::Item>;
}

impl IContainer for HashMap<u32, Entry> {
    type Item = Entry;

    fn add(&mut self, obj: Self::Item) -> io::Result<()> {
        match self.entry(obj.id) {
            std::collections::hash_map::Entry::Occupied(_) => Err(io::Error::new(io::ErrorKind::AlreadyExists, "Entry already exists!")),
            std::collections::hash_map::Entry::Vacant(v) => {
                let value = v.insert(obj);
                println!("ADDED: {:?}", value);
                Ok(())
            }
        }
    }

    fn delete(&mut self, id: u32) -> io::Result<()>
    {
        let entry = self.remove(&id).ok_or(
            io::Error::new(io::ErrorKind::InvalidInput, format!("Error: ID {} does not exist!", id))
        )?;
        println!("REMOVED: {:?}", entry);
        Ok(())
    }

    fn find<P>(&self, predicate: P) -> Option<&Self::Item>
        where P: Fn(&Self::Item) -> bool
    {
        self.values().find(|value| predicate(value))
    }

    fn find_mut<P>(&mut self, predicate: P) -> Option<&mut Self::Item>
        where P: Fn(&Self::Item) -> bool
    {
        self.values_mut().find(|value| predicate(value))
    }

    fn find_by_id(&self, id: u32) -> Option<&Self::Item> {
        self.get(&id)
    }

    fn find_mut_by_id(&mut self, id: u32) -> Option<&mut Self::Item> {
        self.get_mut(&id)
    }
}

fn main() -> io::Result<()> {
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
