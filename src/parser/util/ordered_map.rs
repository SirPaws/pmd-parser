use std::borrow::Borrow;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Debug, PartialEq, Clone)]
pub struct OrderedMap<K: Eq + Clone + Hash, V> {
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K: Eq + Clone + Hash, V> OrderedMap<K, V> 
    where 
{
    pub fn new() -> Self {
        Self { map: HashMap::new(), order: vec![] }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.order.push(key.clone());
        self.map.insert(key, value);
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where 
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        self.map.get(key)
    }
    
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V> 
        where 
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        self.map.get_mut(key)
    }

    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool 
        where 
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        self.map.contains_key(key)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        self.map.keys()
    } 
}

pub struct Iter<'a, K: Eq + Clone + Hash, V> {
    map: &'a OrderedMap<K, V>,
    index: usize,
}

pub struct IterMut<'a, K: Eq + Clone + Hash, V> {
    map: &'a mut OrderedMap<K, V>,
    index: usize,
}


impl<'a, K: Eq + Clone + Hash, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.map.order.len() {
            None
        } else {
            let key = &self.map.order[self.index];
            self.index = self.index + 1;
            if let Some(value) = self.map.map.get(key) {
                Some((key, value))
            } else {
                None
            }
        }
    }    
}

impl<'a, K: Eq + Clone + Hash, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.map.order.len() {
            None
        } else {
            let key = &self.map.order[self.index];
            self.index = self.index + 1;
            if let Some(value) = self.map.map.get_mut(key) {
                let value = unsafe { std::mem::transmute(&mut *value) };
                let key   = unsafe { std::mem::transmute(& *key) };
                Some((key, value))
            } else {
                None
            }
        }
    }    
}

impl<'a, K: Eq + Clone + Hash, V> IntoIterator for &'a OrderedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { map: self, index: 0 }
    }
}

impl<'a, K: Eq + Clone + Hash, V> IntoIterator for &'a mut OrderedMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { map: self, index: 0 }
    }
}
