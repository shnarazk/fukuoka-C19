# Fukuoka COVID-19 stats viewer

Swift版から移植。

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

## Note

2022-02-15のデータが壊れとるがな！第1フィールドが数値にならないので気いつけや。

```
 - fail to convert data: あｑ,400009,福岡県,2022/02/15,火,糟屋郡,80代,女性
 ```
