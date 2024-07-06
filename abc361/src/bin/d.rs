use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: String,
        mut t: String,
    }

    let s = s.chars();
    let t = t.chars();
    dbg!(&s);
    dbg!(&t);

    /*
    bとwの数が等しければどのような順序にも対応できるかもしれない
    bbb
    bbw
    bwb
    bww
    wbb
    wbw
    wwb
    www

    bbb
    wbb
    bbw
    wbw
    bwb
    wwb
    bww
    www
     */

     // ←から順に確定させていく方法をとるとして、その方法で操作回数が最小値となるかわからない


     // memo
     // ・ビット演算に似た考えは利用できるか？
     // ・dpを使うにしても、確定的な情報がわからない。答えから探ることはできないかもしれない
}
