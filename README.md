## comment

`tips:` means something hot.
`error:` means encounting some rust syntax error.


## reference

- [レッドコーダーが教える、競プロ・AtCoder上達のガイドライン【中級編：目指せ水色コーダー！】 #新人プログラマ応援 - Qiita](https://qiita.com/e869120/items/eb50fdaece12be418faa#0-%E3%81%AF%E3%81%98%E3%82%81%E3%81%AB)
- [たのしい探索アルゴリズムの世界【前編：全探索、bit全探索から半分全列挙まで】 #新人プログラマ応援 - Qiita](https://qiita.com/e869120/items/25cb52ba47be0fd418d6#2-%E3%81%99%E3%81%B9%E3%81%A6%E3%81%AE%E5%9F%BA%E6%9C%AC%E5%85%A8%E6%8E%A2%E7%B4%A2)
- 


- [Rust petgraph で競技プログラミングのグラフ理論系頻出アルゴリズムを学ぶ #AtCoder - Qiita](https://qiita.com/hossie/items/ff0e9be89f22dea41aea)


## pow

https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d4921fb028a42e4cbd587181519a38a0


## add test case
[cargo-compete/README-ja.md at master · qryxip/cargo-compete](https://github.com/qryxip/cargo-compete/blob/master/README-ja.md#cargo-compete-retrieve-testcases--cargo-compete-download)
`cargo compete download --overwrite --full`で全テストケースをダウンロードするには、下記手順によりDropbox apiを利用するためのAccess tokenを準備しておく必要がある。

1. Dropbox Appを作成する　＞　[test_for_atcoder - Dropbox](https://www.dropbox.com/developers/apps/info/e5jdwrkonjzxpam) 
2. 作成したDropbox Appのコンソール上でAccess tokenを生成する　＞　`Generated access token`
3. 生成したAccess tokenをcargo-competeが読み込めるように配置する　＞　`~/.local/share/cargo-compete/tokens/dropbox.json`

`cargo compete download --overwrite --full`で全テストケースをダウンロードした場合、
[Extend::SystemTestCases](https://github.com/qryxip/cargo-compete/blob/master/README-ja.md#extendsystemtestcases)となっており、
実テストデータファイルは`~/.cache/cargo-compete/system-test-cases/atcoder.jp/contests/`配下に保存されている模様。
