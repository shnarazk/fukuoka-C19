use {hyper::Client, hyper_tls::HttpsConnector, regex::Regex};

#[derive(Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct CovidInstance {
    pub num: u32,
    // pref_num: u32,
    // pref: String,
    pub date: String,
    pub location: String,
    pub age: String,
    pub gender: String,
}

impl TryFrom<&str> for CovidInstance {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let csv_line: Regex = Regex::new(
            // 176230,400009,福岡県,2022/02/11,金,久留米市,20代,男性,,,
            // r"([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*)"
            // "221246,400009,福岡県,2022/02/24,木,久留米市,20代,男性"
            r"([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*)",
        )
        .expect("");
        csv_line.captures(s).map_or_else(
            || {
                eprintln!(" - fail to parse data: {}", s);
                Err(())
            },
            |csv| {
                if let Ok(num) = csv[1].parse::<u32>() {
                    Ok(CovidInstance {
                        num,
                        date: csv[4].to_string(),
                        location: csv[6].to_string(),
                        age: csv[7].to_string(),
                        gender: csv[8].to_string(),
                    })
                } else {
                    eprintln!(" - fail to convert data: {}", s);
                    Err(())
                }
            },
        )
    }
}

pub async fn load_csv() -> hyper::Result<Vec<CovidInstance>> {
    let base = "https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients";
    let target = Regex::new("https://fukuokakenblob[^\"]+\\.csv").expect("wrong regex");
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let res = client.get(base.parse().expect("wrong url")).await?;
    let buf = hyper::body::to_bytes(res).await?;
    let str = String::from_utf8_lossy(buf.as_ref());
    for l in str.lines() {
        if let Some(url) = target.captures(l) {
            if !url[0].contains("_new") {
                continue;
            }
            eprintln!("start downloading {}...", &url[0]);
            let res = client.get(url[0].parse().expect("wrong url")).await?;
            let buf = hyper::body::to_bytes(res).await?;
            eprintln!("done.");
            return Ok(String::from_utf8_lossy(buf.as_ref())
                .split('\n')
                .skip(1)
                .filter(|s| !s.is_empty())
                .flat_map(CovidInstance::try_from)
                .collect::<Vec<CovidInstance>>());
        }
    }
    Ok(vec![])
}
