use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SugResponse {
    data: Option<Vec<Meaning>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meaning {
    k: String,
    v: String,
}

/// 百度单词翻译
#[tauri::command]
pub async fn translate_word(word: String) -> Result<Vec<String>, String> {
    let client = Client::new();
    let url = "https://fanyi.baidu.com/sug";

    let params = [("kw", word.clone())];

    let res = client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    let parsed: SugResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    if let Some(data) = parsed.data {
        Ok(data
            .into_iter()
            .map(|m| format!("{}: {}", m.k, m.v))
            .collect())
    } else {
        Err("no_result".to_string())
    }
}

/// 有道句子翻译
#[tauri::command]
pub async fn translate_sentence(sentence: String) -> Result<String, String> {
    let client = Client::new();
    let url = "https://m.youdao.com/translate";

    let params = [("inputtext", sentence.as_str()), ("type", "auto")];

    let res = client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("#translateResult li").unwrap();

    if let Some(element) = document.select(&selector).next() {
        Ok(element.text().collect::<String>())
    } else {
        Err("no_result".to_string())
    }
}
