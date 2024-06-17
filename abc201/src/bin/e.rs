use petgraph::{
    graph::{node_index, UnGraph},
    visit::{Dfs, VisitMap},
};
use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {n: usize, uvw: [(Usize1, Usize1, u64); n-1]}
    let mut g = UnGraph::<u64, u64, usize>::from_edges(uvw);

    let mut dfs = Dfs::new(&g, node_index(0));
    while let Some(nx) = dfs.next(&g) {
        let mut edges = g.neighbors(nx).detach();
        while let Some((edge, ny)) = edges.next(&g) {
            if dfs.discovered.is_visited(&ny) {
                continue;
            }
            g[ny] = g[nx] ^ g[edge];
        }
    }

    let mut cnt = [0; 60];
    for node in g.node_indices() {
        for i in 0..60 {
            if (g[node] >> i) & 1 == 1 {
                cnt[i] += 1;
            }
        }
    }

    let mut ans = 0;
    for (i, &x) in cnt.iter().enumerate() {
        ans = (ans + (1 << i) % MOD * x * (n - x)) % MOD;
    }

    println!("{}", ans % MOD);
}
