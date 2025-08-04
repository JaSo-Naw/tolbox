use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tokio::process::Command;

#[tauri::command]
pub async fn download_bilibili(bv: String, title: String, path: String) -> Result<String, String> {
    let url = format!("https://www.bilibili.com/video/{}", bv);
    let encoded_title = urlencoding::encode(&title);

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0")
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| e.to_string())?;

    let referer = format!(
        "https://search.bilibili.com/all?keyword={}&from_source=webtop_search&spm_id_from=333.1007&search_source=5",
        encoded_title
    );

    let resp_bytes = client
        .get(&url)
        .header("Referer", referer)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;
    let mut d = flate2::read::GzDecoder::new(&resp_bytes[..]);
    let mut resp = String::new();
    d.read_to_string(&mut resp).map_err(|e| e.to_string())?;
    println!("{}", resp);

    let re = Regex::new(r"(?s)window.__playinfo__=(.*?)</script>").unwrap();
    let caps = re.captures(&resp).ok_or("未找到视频信息")?;
    let json_data: Value =
        serde_json::from_str(&caps[1]).map_err(|e| format!("JSON解析失败: {}", e))?;

    let video_url = json_data["data"]["dash"]["video"][0]["baseUrl"]
        .as_str()
        .ok_or("未找到视频链接")?
        .to_string();
    let audio_url = json_data["data"]["dash"]["audio"][0]["baseUrl"]
        .as_str()
        .ok_or("未找到音频链接")?
        .to_string();

    // 确保路径存在
    let save_dir = Path::new(&path);
    if !save_dir.exists() {
        tokio::fs::create_dir_all(save_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let video_path = save_dir.join(format!("tolbox_getBilibili_video_{}.mp4", title));
    let audio_path = save_dir.join(format!("tolbox_getBilibili_audio_{}.mp3", title));
    let output_path = save_dir.join(format!("tolbox_getBilibili_output_{}.mp4", title));

    // 下载视频
    let mut video_file = File::create(&video_path).map_err(|e| e.to_string())?;
    let video_bytes = client
        .get(&video_url)
        .header("Referer", &url)
        .header("Origin", "https://www.bilibili.com")
        .header("Connection", "keep-alive")
        .header("Range", "bytes=0-") // 强制全量下载
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| format!("读取视频数据失败: {}", e))?;
    video_file
        .write_all(&video_bytes)
        .map_err(|e| format!("写入视频失败: {}", e))?;

    // 下载音频
    let mut audio_file = File::create(&audio_path).map_err(|e| e.to_string())?;
    let audio_bytes = client
        .get(&audio_url)
        .header("Referer", &url)
        .header("Origin", "https://www.bilibili.com")
        .header("Connection", "keep-alive")
        .header("Range", "bytes=0-") // 强制全量下载
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| format!("读取音频数据失败: {}", e))?;
    audio_file
        .write_all(&audio_bytes)
        .map_err(|e| format!("写入音频失败: {}", e))?;

    // 调用 ffmpeg 合并
    let status = Command::new("ffmpeg")
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .args([
            "-i",
            video_path.to_str().unwrap(),
            "-i",
            audio_path.to_str().unwrap(),
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-strict",
            "experimental",
            output_path.to_str().unwrap(),
        ])
        .status()
        .await
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(format!("下载并合成完成 -> {}", output_path.display()))
    } else {
        Err("FFmpeg 合成失败".to_string())
    }
}
