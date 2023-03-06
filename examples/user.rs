use indexed_vector::{BTreeIndexedVector, HashIndexedVector, IndexedVector};

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct User {
    name: String,
    age: u16,
}

fn main() {
    let users = vec![
        User {
            name: "Tom".to_owned(),
            age: 20,
        },
        User {
            name: "Jane".to_owned(),
            age: 20,
        },
        User {
            name: "Ivan".to_owned(),
            age: 30,
        },
    ];

    let hash_vec = HashIndexedVector::new(users.clone(), |user: &User| user.age);
    // Tom and Jane
    dbg!(hash_vec.search(&20).collect::<Vec<_>>());
    // Ivan
    dbg!(hash_vec.search(&30).collect::<Vec<_>>());

    let btree_vec = BTreeIndexedVector::new(users, |user: &User| user.age);
    // Tom, Jane and Ivan
    dbg!(btree_vec.search_range(10..40).collect::<Vec<_>>());
}
