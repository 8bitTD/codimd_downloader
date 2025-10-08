use std::io::prelude::*;
use super::codimd::*;
use super::header;
use super::html;
use super::html_add;
use super::line::*;
use super::json::*;

pub fn get_infos(codimd_json: &str, user_name: &str, password: &str) -> Vec<Info>{//ページ情報を取得
    let client = reqwest::blocking::Client::new();
    let Ok(response) = client
        .get(codimd_json)
        .basic_auth(String::from(user_name), Some(String::from(password)))
        .timeout(std::time::Duration::new(30,0))
        .send() else {return Vec::new()};
    let Ok(bytes) = response.bytes() else {return Vec::new()};
    let Ok(cvt) = String::from_utf8(bytes.to_vec()) else {return Vec::new()};
    let Ok(infos) = serde_json::from_str(&cvt[..]) else {return Vec::new()};
    infos
}

pub fn get_title(url: &str, codimd_json: &str, user_name: &str, password: &str) -> String{//urlからタイトルを取得
    let mut title = String::new();
    let tmp = url.split_once("#");
    let url = if tmp.is_some(){ tmp.unwrap().0}
    else{url};
    for i in get_infos(codimd_json, user_name, password){
        if !url.contains(&i.url){continue;}
        title = String::from(&i.title);
    }
    title
}

pub fn replace_img_path(md:&str) -> String{
    let re = regex::Regex::new(r"<img(?P<h>.[\s\S]+?)>").unwrap();//<img *>の中で改行(\n)を空欄に置き換える処理
    let md = re.replace_all(&md,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str().to_string();
            format!("<img{}>", &res.replace("\n",""))
        }
    );
    let md = md.to_string();
    let re = regex::Regex::new(r#"src="(?P<h>.[\s\S]+?)""#).unwrap();//src="*"の中のパスのスペース部分を削除する処理
    let result = re.replace_all(&md,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            let result = format!(r#"src="{}""#, res.replace(" ",""));
            result
        }
    );
    let md = result.to_string();
    md
}


pub fn replace_mp4_path(md:&str) -> String{
    let rs = md.replace(r#".mp4'/>"#,r#".mp4'></iframe>"#);
    let rs = rs.replace(r#".mp4"/>"#,r#".mp4"></iframe>"#);
    let rs = rs.replace(r#".mp4"
/>"#,r#".mp4"></iframe>"#);
    rs
}

pub fn get_url_from_shortid(shortid: &str, codimd_json: &str, user_name: &str, password: &str) -> String{//shortidからURLを取得する処理
    let mut url = String::new();
    let infos = get_infos(codimd_json, user_name, password);
    for i in  infos{
        if &i.shortid  == shortid{
            url = i.url.to_string();
        }
    }
    url
}

pub fn get_codimd_url(url: &str) -> String{
    let codimd_url_index = url.rfind("/").unwrap();
    let mut codimd_url = url.to_string();
    let _ = codimd_url.split_off(codimd_url_index+1);
    return codimd_url;
}

pub fn download_codimd_link(md: &str, output_path: &str, json: &Json, cms: &mut Vec<CodiMD>,cnt: &mut usize) -> String{
    let codimd_url = get_codimd_url(&cms[0].url);
    let mut html_lines:Vec<String> = Vec::new();
    let mozi:Vec<&str> = md.lines().collect();
    let infos = get_infos(&json.codimd_json, &json.user_name, &json.password);
    for m in mozi{
        if m.contains(&codimd_url)&& wildmatch::WildMatch::new("*[*](*)*").matches(m){
            let (url, np) = get_codimd_info(m, &codimd_url, &json.codimd_json, &json.user_name, &json.password);
            html_lines.push(np);
            let urls:Vec<String> = cms.iter().map(|c| c.url.to_string()).collect();
            if !urls.contains(&url){
                download_codimd(&url, output_path, &json, cms,cnt);
                cms.push(CodiMD::new(&url,&json.output_path,&infos));
            }
        }else if m.starts_with(&codimd_url){
            let mut tmp_m = String::from(m);
            if tmp_m.contains("?"){
                let (f, _) = m.split_once("?").unwrap();
                tmp_m = String::from(f);
            }
            if tmp_m.contains("#"){
                let (f, _) = m.split_once("#").unwrap();
                tmp_m = String::from(f);
            }
            let title = get_title(&tmp_m.replace("-","+"), &json.codimd_json, &json.user_name, &json.password);
            let nl = format!("[{}]({})", title, tmp_m);
            let (url, np) = get_codimd_info(&nl, &codimd_url, &json.codimd_json, &json.user_name, &json.password);
            html_lines.push(np);
            let urls:Vec<String> = cms.iter().map(|c| c.url.to_string()).collect();
            if !urls.contains(&url){
                download_codimd(&url, output_path, &json, cms,cnt);
                cms.push(CodiMD::new(&url, &json.output_path,&infos));
            }
        }else{
            html_lines.push(String::from(m));
        }
    }
    let tmp = html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n");
    return tmp;
}

pub fn download_codimd(url: &str, output_path: &str, json: &Json, cms: &mut Vec<CodiMD>, cnt:&mut usize) -> bool{
    println!("{:?}", "codimd_downloader!");
    *cnt += 1;
    let url = url.replace("-","+");
    let mut md = String::from("");
    let mut output = format!("{}", output_path);
    let mut path = String::from("");
    let mut title = String::from("");
    let infos = get_infos(&json.codimd_json, &json.user_name, &json.password);
    cms.push(CodiMD::new(&url,&output,&infos));
    for i in infos{
        if !url.contains(&i.url){continue;}
        md = String::from(&i.content);
        title = String::from(&i.title);
        let re = regex::Regex::new(r#"[/|\\|:|\*|"|<|>|\|]+?"#).unwrap();
        let result = re.replace_all(&title, "");
        title = result.to_string();
        output = match cnt{
            1 => {format!("{}{}{}",output,"/",&title)},
            _ => {format!("{}",output)},
        };
        path = format!("{}{}{}{}",output,"/",&title,".html");
    }
    if &md == ""{return false;}
    println!("{} - {}",title, url);
    //let md = header::replace_header(&md);
    let headers = header::get_header(&md);
    if *cnt == 1{
        if std::path::Path::new(&output).is_dir(){rm_rf::remove(&output).unwrap();}
        if !std::path::Path::new(&output).is_dir(){Some(std::fs::create_dir_all(&output));}
    }
    let img_path = format!("{}{}",output,"/resource");
    Some(std::fs::create_dir_all(&img_path));
    
    //mp4のiframe表記修正
    let md = replace_mp4_path(&md);
    //リソース取得
    let md = replace_img_path(&md);
    let res = html::get_resource_path(&md, &json.url_path, &json.resource_path);
    html::copy_resource(&res, &img_path);
    
    let md = html::replace_path(&md, &json.url_path);
    //CodiMDのページ情報を取得
    let md = download_codimd_link(&md, &output, &json, cms, cnt);
    let html = html::convert_html(&md); 
    let html_add = html_add::add::HTMLADDFRONT.replace("style--style", &json.style);
    let html = format!("{}{}", html_add, &html);
    let html = format!("{}{}", &html, header::get_toc_menu(headers));
    let html = format!("{}{}", &html, html_add::add::HTMLADDBACK);
    let html = html.replace("title--title",&format!("{} - CodiMD", &title));
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(html.as_bytes()).unwrap();
    true
}