## test case

https://www.dropbox.com/sh/nx3tnilzqz7df8a/AABh1dWn9MpTS1XX_t3_2nHfa/tessoku-book?dl=0&subfolder_nav_tracking=1


## add test case
[cargo-compete/README-ja.md at master · qryxip/cargo-compete](https://github.com/qryxip/cargo-compete/blob/master/README-ja.md#cargo-compete-retrieve-testcases--cargo-compete-download)
`cargo compete download --overwrite --full`で全テストケースをダウンロードするには、下記手順によりDropbox apiを利用するためのAccess tokenを準備しておく必要がある。

1. Dropbox Appを作成する　＞　[test_for_atcoder - Dropbox](https://www.dropbox.com/developers/apps/info/e5jdwrkonjzxpam) 
2. 作成したDropbox Appのコンソール上でAccess tokenを生成する　＞　`Generated access token`
3. 生成したAccess tokenをcargo-competeが読み込めるように配置する　＞　`~/.local/share/cargo-compete/tokens/dropbox.json`

`cargo compete download --overwrite --full`で全テストケースをダウンロードした場合、
[Extend::SystemTestCases](https://github.com/qryxip/cargo-compete/blob/master/README-ja.md#extendsystemtestcases)となっており、
実テストデータファイルは`~/.cache/cargo-compete/system-test-cases/atcoder.jp/contests/`配下に保存されている模様。

Access tokenは寿命が短いので、下記手順で更新する。

1. Dropbox Appのコンソール上でAccess tokenを生成する　＞　`Generated access token`
2. `sync-dropbox-api.sh`のAccess tokenを更新する
3. `bash ./sync-dropbox-api.sh`を流す
