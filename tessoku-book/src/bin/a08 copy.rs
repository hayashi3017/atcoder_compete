use ndarray::*;
use proconio::{fastout, input};
use std::usize;

#[fastout]
fn main() {
    input! {
      h: usize,
      w: usize,
      x: [[i32; w]; h],
      q: usize,
      poss: [[i32; 4]; q],
    }
    // tips: ndarrayのfrom_shape_vecによるvec生成
    // 例：　https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a8102e1986ab5f1aa41b1f6c91bfc685
    // see: [ndarray/README-quick-start.md at master · rust-ndarray/ndarray](https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md)
    let vec = Array::from_shape_vec((h, w), x.concat()).unwrap();
    for pos in poss {
        let area = vec.slice(s![pos[0] - 1..pos[2], pos[1] - 1..pos[3]]);
        println!("{:?}", area.sum());
    }
}
