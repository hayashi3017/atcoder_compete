use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize, usize);n]
    }

    if ab.iter().fold(0, |xor, &(a, b)| {
        // tips: proconio::marker::Usize1を使ってインデックスを調整してもよい。ここでは a - 1, b - 1を計算している
        // https://qiita.com/hossie/items/90a450598d22ade3dd00#c-%E5%85%A5%E5%8A%9B%E6%99%82%E3%81%AB%E3%82%A4%E3%83%B3%E3%83%87%E3%83%83%E3%82%AF%E3%82%B9%E3%82%92-1%E6%B8%9B%E3%82%89%E3%81%99
        return xor ^ a - 1 ^ b - 1;
    }) != 0usize
    {
        println!("First");
    } else {
        println!("Second");
    }
}
