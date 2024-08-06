use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    a.sort();
    let grouped: Vec<Vec<usize>> = a
        .iter()
        .group_by(|&&x| x)
        .into_iter()
        .map(|(_, group)| group.cloned().collect())
        .collect();
    dbg!(&a);
    let mut cnt = 0;
    for v in grouped {
        let mut a = v.iter().combinations(3);
        // このようにCombinationsをVec<T>に戻すことができる
        // let b: Vec<Vec<_>> = a.collect();
        if let Some(pair) = a.next() {}
    }

    // 途中です。
    let mut it = (1..6).combinations(3);
    dbg!(&it);
    dbg!(it.next());
    dbg!(it.next());
}

// tips: HashMapを用いて配列を等しい値同士にグループ化する
// let arr = vec![1, 1, 1, 1, 2, 2, 2];
// let mut groups: HashMap<i32, Vec<i32>> = HashMap::new();

// for &value in &arr {
//     groups.entry(value).or_insert_with(Vec::new).push(value);
// }

// for (key, group) in groups {
//     println!("{:?}: {:?}", key, group);
// }
