use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let count_map = Arc::new(Mutex::new(HashMap::new()));
    let input_vec = Arc::new(input.iter().map(|s| s.to_string()).collect::<Vec<_>>());

    let chunk_size = input_vec.len() / worker_count + 1;
    let chunks = input_vec.chunks(chunk_size).map(|chunk| chunk.to_vec());

    let mut handles = vec![];
    for chunk in chunks {
        let count_map = count_map.clone();
        let handle = thread::spawn(move || {
            for s in chunk {
                for c in s.chars() {
                    if c.is_numeric() || c.is_ascii_punctuation() {
                        continue;
                    }
                    let c = c.to_lowercase().next().unwrap_or(c);
                    let mut map = count_map.lock().unwrap();
                    *map.entry(c).or_insert(0) += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(count_map).unwrap().into_inner().unwrap()
}
