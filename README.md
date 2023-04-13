# Fukuoka COVID-19 stats viewer

Swift版から移植。2022年9月27日で更新停止したので意味なくなった。

- [dioxus](https://github.com/DioxusLabs/dioxus)
  - [the guide](https://dioxuslabs.com/guide/)
- [福岡県新型コロナウイルス感染症陽性者発表情報](https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients)

## Flow

```mermaid
flowchart TD
A[hyper::Client::get] -- html --> B[parse and get];
B -- csv data --> C[regex::capture];
C --csv::CovidPatient --> D[calculate stats.];
D -- dioxus::Element --> E[Dioxus];
```

![スクリーンショット 2023-04-14 1 21 16](https://user-images.githubusercontent.com/997855/231823208-4357d42a-4752-4b90-8b71-2adfce49890d.png)
![スクリーンショット 2023-04-14 1 21 32](https://user-images.githubusercontent.com/997855/231823216-2a5fdfd6-e5e9-4e3e-9950-5895802c809b.png)
![スクリーンショット 2023-04-14 1 21 48](https://user-images.githubusercontent.com/997855/231823221-003d1729-f51c-4789-bf10-2de6f42566e6.png)

## Note

2022-04-14

数日前からデータが取れなくなった。多分32万行を超えて何かの上限になったのだろう。。

2022-02-15のデータが壊れとるがな！第1フィールドが数値にならないので気いつけや。

```
 - fail to convert data: あｑ,400009,福岡県,2022/02/15,火,糟屋郡,80代,女性
 ```
