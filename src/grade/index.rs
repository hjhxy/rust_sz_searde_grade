use std::collections::HashMap;
use std::error::Error;
use std::io;
use tokio::time::{sleep, Duration};
use reqwest::{header, Client, Response};
use serde_json::json;
use serde::Deserialize;
use crate::grade::util::*;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

pub async fn search_grade(cookie: String){
    match validate_cookie(&cookie) {
        Ok(_) => {}
        Err(e) => {
            println!("❌ {}", e);
            before_exit();
            return;
        }
    }

    #[derive(Debug, Deserialize)]
    struct ResponseData {
        datas: Datas,
        code: String,
    }

    #[derive(Debug, Deserialize)]
    struct Datas {
        xscjcx: Xscjcx,
    }

    #[derive(Debug, Deserialize)]
    struct Xscjcx {
        totalSize: u32,
        pageNumber: u32,
        pageSize: u32,
        rows: Vec<Row>,  // `rows` 是一个数组
        extParams: ExtParams,
    }

    #[derive(Debug, Deserialize)]
    struct Row {
        KSXZDM: Option<String>,
        CJXSZ: Option<String>,
        KCMC: Option<String>,
        XNXQDM: Option<String>,
        KKDWDM: Option<String>,
        SFJG: Option<u8>,
        JDZ: Option<f32>,
        XF: Option<f32>,
        CZRXM: Option<String>,
        KFCXRQ: Option<String>,
        SFJG_DISPLAY: Option<String>,
        KSXZDM_DISPLAY: Option<String>,
        CJFZDM: Option<String>,
        ZTDM: Option<u32>,
        WID: Option<String>,
        XH: Option<String>,
        CZR: Option<String>,
        LRRXM: Option<String>,
        CZSJ: Option<String>,
        DYBFZCJ: Option<f32>,
        XNXQDM_DISPLAY: Option<String>,
        CJ: Option<String>,
        KCLBDM: Option<String>,
        KCLBMC: Option<String>,
        LRSJ: Option<String>,
        DQSFKFCX: Option<u8>,
        KCDM: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct ExtParams {
        logId: String,
        code: i32,
        totalPage: u32,
    }

    let mut form_data = HashMap::new();
    form_data.insert("querySetting", json!([
        {
            "name": "_gotoFirstPage",
            "value": true,
            "linkOpt": "AND",
            "builder": "equal"
        }
    ]).to_string());
    form_data.insert("pageSize", "10".to_string());
    form_data.insert("pageNumber", "1".to_string());

    println!("🚗开始查询成绩...稍等☕️");
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{bar:40.green/white}] {pos:>3}/{len} {msg}")
        .unwrap()
        .progress_chars("█>-"));
    for i in 1..=100 {
        pb.set_position(i);
        // pb.set_message(format!("查询进度: {}%", i));
        sleep(Duration::from_millis(5)).await;
    }

    let client = Client::new();
    let response = client.
        post("https://ehall.szu.edu.cn/gsapp/sys/szdxwdcjapp/modules/wdcj/xscjcx.do")
        .header(header::COOKIE, cookie)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&form_data)
        .send().await;

    let response = match response {
        Ok(r) => r,
        Err(e) => {
            pb.finish_with_message("查询失败！");
            before_exit();
            return;
        }
    };

    let text = match response.text().await {
        Ok(t) => t,
        Err(e) => {
            pb.finish_with_message("❌ 查询失败！获取数据失败！");
            before_exit();
            return; // 或者其他处理逻辑
        }
    };

    let data = match serde_json::from_str::<ResponseData>(&text) {
        Ok(d) => d,
        Err(e) => {
            pb.finish_with_message("❌ 查询失败！JSON解析失败！");
            before_exit();
            return;
        }
    };
    let total_size = data.datas.xscjcx.totalSize;
    let courses = data.datas.xscjcx.rows.iter().map(|row| {
        let class_name = row.KCMC.clone().unwrap_or_else(|| "暂无".to_string());
        let class_grade = row.CJXSZ.clone().unwrap_or_else(|| "暂无".to_string());
        let class_gpa = row.JDZ.clone().unwrap_or_else(|| 0f32).to_string();
        let class_teacher = row.CZRXM.clone().unwrap_or_else(|| "暂无".to_string());
        let class_credit = row.XF.clone().unwrap_or_else(|| 0f32).to_string();
        let res = vec![
            pad_str(&class_name, 20),
            pad_str(&class_grade, 20),
            pad_str(&class_gpa, 8),
            pad_str(&class_teacher, 16),
            pad_str(&class_credit, 10),
        ];
        res
    }).collect::<Vec<_>>();
    let res_title = vec![
        pad_str("课程名称", 20),
        pad_str("成绩（百分制）", 20),
        pad_str("绩点", 8),
        pad_str("任课教师", 16),
        pad_str("学分", 10),
    ];

    println!("\n");
    println!("📑总课程数🖊：{}门", total_size);
    println!("📑已出成绩🧑‍🏫：{}门", courses.len());
    println!("{}", res_title.join(" | ").bold().cyan());
    for course in courses {
        println!("{}", course.join(" | "));
    }
    before_exit();
}