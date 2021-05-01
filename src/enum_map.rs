extern crate strum;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//An object of type EnumMap<E,T> is a function f from E to T where E is a finite enum such that
//it's efficient to change the value of f of an individual member of E.
#[derive(Clone)]
pub struct EnumMap<E, T>
where
    E: std::hash::Hash + IntoEnumIterator + Eq + PartialEq + Copy,
    T: Clone,
{
    map: HashMap<E, T>,
}

impl<E: std::hash::Hash + IntoEnumIterator + Eq + PartialEq + Copy, T: Clone> EnumMap<E, T> {
    pub fn new<M>(mapping: M) -> EnumMap<E, T>
    where
        M: Fn(E) -> T,
    {
        let mut enum_map = EnumMap {
            map: HashMap::new(),
        };
        for key in E::iter() {
            enum_map.map.insert(key, mapping(key));
        }

        return enum_map;
    }

    pub fn get(&self, key: E) -> T {
        return self.map.get(&key).unwrap().clone();
    }
    pub fn set(&mut self, key: E, value: T) -> () {
        self.map.insert(key, value);
    }
}
