pub mod persistency;
use crate::error::Result;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Entry {
    pub id: u32,
    pub name: String,
    pub content: String,
}

pub trait IContainer {
    type Item;
    fn add(&mut self, obj: Self::Item) -> Result<()>;
    fn delete(&mut self, id: u32) -> Result<()>;
    fn find<P>(&self, predicate: P) -> Option<&Self::Item>
        where P: Fn(&Self::Item) -> bool;
    fn find_mut<P>(&mut self, predicate: P) -> Option<&mut Self::Item>
        where P: Fn(&Self::Item) -> bool;
    fn find_by_id(&self, id: u32) -> Option<&Self::Item>;
    fn find_mut_by_id(&mut self, id: u32) -> Option<&mut Self::Item>;
}

impl IContainer for HashMap<u32, Entry> {
    type Item = Entry;

    fn add(&mut self, obj: Self::Item) -> Result<()>
    {
        match self.entry(obj.id) {
            std::collections::hash_map::Entry::Occupied(_) => Err("Entry already exists!")?,
            std::collections::hash_map::Entry::Vacant(v) => {
                let value = v.insert(obj);
                println!("ADDED: {:?}", value);
                Ok(())
            }
        }
    }

    fn delete(&mut self, id: u32) -> Result<()>
    {
        let entry = self.remove(&id)
            .ok_or_else(|| format!("Error: ID {} does not exist!", id))?;
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

    fn find_by_id(&self, id: u32) -> Option<&Self::Item>
    {
        self.get(&id)
    }

    fn find_mut_by_id(&mut self, id: u32) -> Option<&mut Self::Item>
    {
        self.get_mut(&id)
    }
}
