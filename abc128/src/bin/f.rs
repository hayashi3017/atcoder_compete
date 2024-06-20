use core::fmt;

fn main() {
    // ビット演算の練習

    dbg!(100i8 << 0);
    dbg!(100i8 << 1);
    dbg!(100i8 << 2);
    dbg!(100i8 << 3);
    // dbg!(100i8 << -1i8);

    bin_debug(100 << 0);
    bin_debug(100 << 1);
    bin_debug(100 << 2);
    bin_debug(100 << 3);
    bin_debug(100);
    bin_debug(0);
    bin_debug(1);
    bin_debug(2);
    bin_debug(3);
}

fn bin_debug(n: isize) {
    println!("{}: {:b}", n, n);
}

// struct V(Vec<u32>);

// impl fmt::Binary for V {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let vec = &self.0;

//         for (count, n) in vec.iter().enumerate() {
//             if count != 0 {
//                 write!(f, " ")?;
//             }
//             write!(f, "{:b}", n)?;
//         }

//         Ok(())
//     }
// }
