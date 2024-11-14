use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub type Priority = u32;
// A struct to hold the path and its popularity
#[derive(Clone, PartialEq, Eq)]
struct Item<T>
where
    T: PartialEq + Eq,
{
    value: T,
    popularity: Priority,
}

// Implement Ord and PartialOrd so BinaryHeap can sort by popularity in reverse order
impl<T> Ord for Item<T>
where
    T: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.popularity.cmp(&other.popularity).reverse()
    }
}

impl<T> PartialOrd for Item<T>
where
    T: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PopularitySet<T>
where
    T: PartialEq + Eq,
{
    items: HashMap<T, Priority>,
    popularity_queue: BinaryHeap<Item<T>>,
}

impl<T> PopularitySet<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            popularity_queue: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, value: T, popularity: Priority) {
        if let Some(existing_popularity) = self.items.get(&value) {
            // Update only if the new popularity is higher
            if *existing_popularity < popularity {
                self.items.insert(value.clone(), popularity);
                self.popularity_queue.push(Item { value, popularity });
            }
        } else {
            // Insert new item
            self.items.insert(value.clone(), popularity);
            self.popularity_queue.push(Item { value, popularity });
        }
    }

    pub fn insert_many(&mut self, values:Vec<(T,Priority)>){
        for item in values.into_iter(){
            self.insert(item.0, item.1);
        }
    }

    /**
     * Returns the most popular item in the queue unless the queue is empty
     */
    pub fn pop(&mut self) -> Option<T> {
        self.pop_with_popularity().map(|x| Some(x.0))?
    }

    /**
     * Returns the most popular item in the queue along with its popularity unless the queue is empty
     */
    pub fn pop_with_popularity(&mut self) -> Option<(T, Priority)> {
        while let Some(Item { value, popularity }) = self.popularity_queue.pop() {
            // Check if the item in the queue is still the most popular in the map
            if let Some(&current_popularity) = self.items.get(&value) {
                if current_popularity == popularity {
                    // Remove from map and return
                    self.items.remove(&value);
                    return Some((value, popularity));
                }
            }
        }
        None
    }

    pub fn as_vec(&self) -> Vec<(T,Priority)>{
        self.items.iter().map(|x| (x.0.clone(),x.1.clone())).collect()
    }

    pub fn as_partial_vec(&self) -> Vec<T>{
        self.items.iter().map(|x|x.0.clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
