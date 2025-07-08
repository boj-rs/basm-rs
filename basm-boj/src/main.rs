// src/main.rs

use clap::Parser;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use scraper::{Html, Selector};
use std::{env, fs};

/// BOJ 문제 샘플 입출력 데이터를 basm/<문제번호>/ 폴더에 저장하는 CLI
#[derive(Parser)]
#[command(author, version, about = "BOJ Testcase Fetcher CLI")] 
struct Args {
    /// BOJ 문제 번호
    problem: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 인자 파싱 및 URL 생성
    let args = Args::parse();
    let pid = args.problem;
    let problem_str = pid.to_string();
    let url = format!("https://www.acmicpc.net/problem/{}", problem_str);
    // println!("Debug: Fetching URL: {}", url);

    // 2. HTTP 클라이언트 설정 (User-Agent)
    let mut headers = HeaderMap::new();
    let ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36";
    // println!("Debug: Using User-Agent: {}", ua);
    headers.insert(
        HeaderName::from_static("user-agent"),
        HeaderValue::from_static(ua),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // 3. 페이지 요청 및 HTML 파싱
    let response = client.get(&url).send().await?;
    // println!("Debug: HTTP status: {}", response.status());
    let body = response.text().await?;
    // println!("Debug: Fetched {} bytes", body.len());
    let document = Html::parse_document(&body);

    // 4. problem-body 아래 모든 <pre> 요소 중 샘플 입출력만 필터링
    let pre_sel = Selector::parse("div#problem-body pre").unwrap();
    let pres: Vec<_> = document
        .select(&pre_sel)
        .filter(|el| {
            if let Some(cls) = el.value().attr("class") {
                if cls.split_whitespace().any(|c| c == "sampledata") {
                    return true;
                }
            }
            if let Some(id) = el.value().attr("id") {
                if id.starts_with("sample-input") || id.starts_with("sample-output") {
                    return true;
                }
            }
            false
        })
        .collect();
    // println!("Debug: Found {} matching <pre> elements", pres.len());
    for el in &pres {
        let _id = el.value().attr("id").unwrap_or("");
        let _class = el.value().attr("class").unwrap_or("");
        // println!("  Element id='{}', class='{}'", _id, _class);
    }
    if pres.len() < 2 {
        eprintln!("Error: 샘플 데이터(pre)가 충분하지 않습니다. found={} pre tags", pres.len());
        return Ok(());
    }

    // 5. 예제 입출력 쌍으로 분리 (순서대로 input, output)
    let mut examples = Vec::new();
    for chunk in pres.chunks(2) {
        if chunk.len() < 2 {
            break;
        }
        let input_text = chunk[0]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        let output_text = chunk[1]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        examples.push((input_text, output_text));
    }
    // println!("Debug: Extracted {} example pairs", examples.len());
    for (i, (input, output)) in examples.iter().enumerate() {
        println!("Example {} input:\n{}\n---\noutput:\n{}\n===", i+1, input, output);
    }

    if examples.is_empty() {
        eprintln!("Error: examples not found after filtering pre tags");
        return Ok(());
    }

    // 6. basm/<problem>/ 폴더 생성 및 파일 저장
    let root = env::current_dir()?;
    let tc_input_dir = root.join("basm").join(&problem_str).join("input");
    let tc_output_dir = root.join("basm").join(&problem_str).join("output");
    fs::create_dir_all(&tc_input_dir)?;
    fs::create_dir_all(&tc_output_dir)?;
    for (i, (input, output)) in examples.iter().enumerate() {
        let in_path = tc_input_dir.join(format!("{}.txt", i + 1));
        let out_path = tc_output_dir.join(format!("{}.txt", i + 1));
        fs::write(&in_path, input)?;
        fs::write(&out_path, output)?;
        println!("Wrote {} and {}", in_path.display(), out_path.display());
    }

    Ok(())
}
