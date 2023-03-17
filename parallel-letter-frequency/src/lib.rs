use std::hash::Hash;
use std::sync::mpsc;

use std::sync::Arc;
use std::thread;
use std::collections::HashMap;

pub fn frequency(input:  &'a [&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count == 0 {
        panic!();
    }
    let (tx, rx) = mpsc::channel();

    // do work
    let mut handles = vec![];
    let per = input.len() / worker_count;
    for i in 0..worker_count { 

        let tx = tx.clone();
        let work = & input[(i * per)..((i + 1) * per)];
        
        let handle = thread::spawn(move || {
            for w in work.iter().cloned() { 
          
            }    
        });

        handles.push(handle);
    }


    // create hashmap from recieved 
    let mut ret = HashMap::new();
    for received in rx { 
        *ret.entry(received).or_insert(0) += 1;
    }

    // wait til finished
    handles.into_iter().for_each(|h| h.join().unwrap());
    ret
}
