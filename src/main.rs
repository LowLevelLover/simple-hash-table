const PRIME_1: u128 = 5003;
const PRIME_2: u128 = 5281;

#[derive(Clone, PartialEq, Eq)]
struct HashTableItem {
    key: String,
    value: String,
}

#[derive(Clone, PartialEq, Eq)]
enum HashItemType {
    Removed,
    Item(HashTableItem),
}

struct HashTable {
    size: usize,
    count: usize,
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
        Self {
            size: 53,
            count: 0,
            items: vec![None; 53],
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

    pub fn insert(&mut self, key: &str, value: &str) {
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
