/* I have a vec! macro, how do I implement a HashMap macro? */

macro_rules! vec {
    ($($item:expr),*) => {
        {
            let mut vec = Vec::new();
            $(vec.push($item);)*
            vec
        }
    }
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
        use std::collections::HashMap;
        let mut hashmap = HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
        }
    }
}

/* Okay, it doesn't seem difficult, but what if there is an even easier way? */

macro_rules! ez_map {
    ($($key:expr => $value:expr),*) => {
        {
        
        let mut ez_map = ::std::collections::HashMap::from([$(($key, $value)),*]);
        ez_map
        }
    }
}
fn main() {
    let my_vec = vec![1, 2, 3];
    println!("This is my vec: {my_vec:?}");
    
    let my_hashmap = hashmap!{ 1 => 2, 3 => 4 };
    println!("This is my hashmap: {my_hashmap:?}");

    let my_ez_map = ez_map!{ 1 => 2, 3 => 4 };
    println!("This is my hashmap: {my_ez_map:?}");
}
