const PRIME_1: u128 = 5003;
const PRIME_2: u128 = 5281;
const INITIAL_BASE_SIZE: usize = 53;

struct Prime(usize);

#[derive(Clone, PartialEq, Eq, Debug)]
struct HashTableItem {
    key: String,
    value: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum HashItemType {
    Removed,
    Item(HashTableItem),
}

#[derive(Debug)]
struct HashTable {
    size: usize,
    count: usize,
    base_size: usize,
    items: Vec<Option<HashItemType>>,
}

impl HashTableItem {
    fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
}

impl HashTable {
    fn new() -> Self {
        HashTable::new_sized(INITIAL_BASE_SIZE)
    }

    fn new_sized(base_size: usize) -> Self {
        Self {
            size: base_size,
            count: 0,
            base_size: Prime(base_size).next_prime(),
            items: vec![None; base_size],
        }
    }

    fn hash(text: &str, prime: u128, m: u128) -> usize {
        let mut hash: u128 = 0;
        let text_len = text.len();
        for i in 0..text_len {
            hash += (prime.pow((text_len - (i + 1)) as u32)
                * (text.chars().nth(i).unwrap() as u128)) as u128;
            hash = hash % m;
        }

        hash as usize
    }

    fn get_hash(text: &str, num_buckets: usize, attempt: usize) -> usize {
        let hash_a = HashTable::hash(text, PRIME_1, num_buckets as u128);
        let hash_b = HashTable::hash(text, PRIME_2, num_buckets as u128);

        (hash_a + (attempt * (hash_b + 1))) % num_buckets
    }

    fn resize(&mut self, base_size: usize) {
        if base_size < INITIAL_BASE_SIZE {
            return;
        }

        let mut new_hash_table = HashTable::new_sized(base_size);

        for i in 0..self.size {
            let item = &self.items[i];
            if let Some(HashItemType::Item(val)) = item {
                new_hash_table.insert(&val.key, &val.value);
            }
        }

        self.base_size = new_hash_table.base_size;
        self.count = new_hash_table.count;
        self.size = new_hash_table.size;
    }

    #[inline]
    fn resize_up(&mut self) {
        self.resize(self.base_size * 2);
    }

    #[inline]
    fn resize_down(&mut self) {
        self.resize(self.base_size / 2);
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let load = self.count * 100 / self.size;
        if load > 70 {
            self.resize_up();
        }

        let item = HashTableItem::new(key, value);
        let mut index = HashTable::get_hash(&item.key, self.size, 0);
        let mut current_item = &self.items[index];

        let mut i = 1;

        while current_item.is_some() {
            if let Some(HashItemType::Item(val)) = current_item {
                if val.key.eq(key) {
                    self.items[index] = Some(HashItemType::Item(item));
                    return;
                }
            }

            index = HashTable::get_hash(&item.key, self.size, i);
            current_item = &self.items[index];
            i += 1;
        }

        self.items[index] = Some(HashItemType::Item(item));
        self.count += 1;
    }

    pub fn search(&self, key: &str) -> Option<&str> {
        let mut index = HashTable::get_hash(key, self.size, 0);
        let mut item = &self.items[index];

        let mut i = 1;

        while item.is_some() {
            if let Some(HashItemType::Item(val)) = item {
                if val.key.as_str().eq(key) {
                    return Some(&val.value);
                }
            }

            index = HashTable::get_hash(key, self.size, i);
            item = &self.items[index];
            i += 1;
        }

        return None;
    }

    pub fn delete(&mut self, key: &str) {
        let load = self.count * 100 / self.size;
        if load < 10 {
            self.resize_down();
        }

        let mut index = HashTable::get_hash(key, self.size, 0);
        let mut item = &self.items[index];

        let mut i = 1;

        while item.is_some() {
            if let Some(HashItemType::Item(val)) = item {
                if val.key.as_str().eq(key) {
                    self.items[index] = Some(HashItemType::Removed);
                }
            }
            index = HashTable::get_hash(key, self.size, i);
            item = &self.items[index];
            i += 1;
        }

        self.count -= 1;
    }
}

impl Prime {
    fn is_prime(&self) -> Option<bool> {
        if self.0 < 2 {
            return None;
        }

        if self.0 < 4 {
            return Some(true);
        }

        if self.0 % 2 == 0 {
            return Some(false);
        }

        for i in (3..=self.0.isqrt()).step_by(2) {
            if self.0 % i == 0 {
                return Some(false);
            }
        }

        return Some(true);
    }

    fn next_prime(&mut self) -> usize {
        while self.is_prime() != Some(true) {
            self.0 += 1;
        }

        self.0
    }
}

fn main() {
    let mut hash_table = HashTable::new();
    hash_table.insert("farzin", "vatani");
    hash_table.insert("me", "v2");
    assert!(hash_table.search("farzin") == Some("vatani"));
    assert!(hash_table.search("me") == Some("v2"));
    hash_table.delete("farzin");
    assert!(hash_table.search("farzin") == None);
    assert!(hash_table.search("ali") == None);
    assert!(hash_table.search("me") == Some("v2"));
}
