use super::*;
use std::{
    sync::{
	Arc,
	RwLock,
    },
    thread,
};
use semaphore::Semaphore;

#[derive(Debug)]
pub struct Cache(RwLock<Map<NonZeroU8, usize>>);

impl Cache
{
    pub fn new() -> Self
    {
	Self(RwLock::new(Map::new()))
    }
    pub fn get_or_insert_with_owned<F>(self: Arc<Self>, key: u8, with: F) -> usize
    where F: FnOnce() -> usize
    {
	match Arc::try_unwrap(self)
	{
	    Ok(this) => {
		debug_assert!(key>0);
		let key = unsafe {NonZeroU8::new_unchecked(key)};

		let map = this.0.into_inner().unwrap();
		if let Some(&value) = map.get(&key) {
		    value
		} else {
		    // No need to insert, the cache will be dropped after this
		    with()
		}
	    },
	    Err(this) => this.get_or_insert_with(key, with)
	}
    }
    #[inline] pub fn insert(&self, key: u8, value: usize) -> usize
    {
	debug_assert!(key>0);
	let key = unsafe {NonZeroU8::new_unchecked(key)};
	self.0.write().unwrap().insert(key, value);
	value
    }
    pub fn get(&self, key: u8) -> Option<usize>
    {
	debug_assert!(key>0);
	let key = unsafe {NonZeroU8::new_unchecked(key)};
	self.0.read().unwrap().get(&key).copied()
    }
    pub fn get_or_insert_with<F>(&self, key: u8, with: F) -> usize
    where F: FnOnce() -> usize
    {
	debug_assert!(key>0);
	let key = unsafe {NonZeroU8::new_unchecked(key)};

	{
	    let read = self.0.read().unwrap();
	    if let Some(&value) = read.get(&key) {
		return value;
	    }
	}
	// Insert needed
	{
	    let value = with();
	    let mut lock = self.0.write().unwrap();
	    lock.insert(key, value);
	    value
	}
    }
}

#[derive(Debug)]
enum Deffered
{
    Known(usize),
    Yielded(thread::JoinHandle<usize>),
}

impl Deffered
{
    #[inline] pub fn into_value(self) -> usize
    {
	match self {
	    Self::Known(v) => v,
	    Self::Yielded(v) => v.join().unwrap(),
	}
    }
}

fn rec_part2(map: Arc<Adaptors>, cache: Arc<Cache>, lock: Semaphore<()>, max: u8, f: u8) -> usize
{
    if f == max {
	return 1;
    }
    debug_assert!(f < max);

    match iterate_adaptor_chain(&map, None, f).map(|next| {
	
	if next == max {
	    Deffered::Known(1)
	} else {
	    match cache.get(next) {
		Some(value) => Deffered::Known(value),
		None => {
		    let map = Arc::clone(&map);
		    let cache = Arc::clone(&cache);
		    let lock = lock.clone();
		    if let Ok(_guard) = lock.try_access() {
			Deffered::Yielded(thread::spawn(move || {
			    cache.clone().insert(next, rec_part2(map, cache, lock, max, next))
			}))
		    } else {
			Deffered::Known(cache.clone().insert(next, rec_part2(map, cache, lock, max, next)))
		    }
		}
	    }
	}
    }).map(Deffered::into_value)
	.sum::<usize>() {
	    #[cold] 0 => panic!("eh"),
	    x => x,
	}
}

fn gensem() -> Semaphore<()>
{
    Semaphore::new(num_cpus::get()+1, ())
}

#[inline] pub fn solve(a: Adaptors, max: u8) -> usize
{
    rec_part2(Arc::new(a), Arc::new(Cache::new()), gensem(), max, 0)
}
