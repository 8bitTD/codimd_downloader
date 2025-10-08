use serde::{Serialize, Deserialize};
//use super::define::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Header{
    pub number: usize,
    pub name: String
}
/*
pub fn is_uniq_equal(line: &str) -> bool{//文字列がヘッダー文字(===)かどうか
    let mut  v_tmp:Vec<&str> = line.split("").collect();
    v_tmp.retain(|&x| x != "");
    v_tmp.retain(|&x| x != " ");
    let uniq: std::collections::HashSet<&str> = v_tmp.into_iter().collect();
    let rv: Vec<&str> = uniq.into_iter().collect::<Vec<&str>>();
    rv == ["="]
}

pub fn replace_header(md:&str) -> String{
    let mut lines:Vec<&str> = md.lines().collect();
    lines.reverse();
    let mut header:Vec<String> = Vec::new();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_equal = false;
    for l in lines{
        if !is_equal{

            if is_uniq_equal(l){ is_equal = true; }
            else{ html_lines.push(String::from(l)); }
            html_lines.push(String::from(l));
        }else{
            if  l.trim().starts_with("```"){
                html_lines.push(String::from(l));
                header.reverse();
                let h_name = header.clone().into_iter().map(|x| x).collect::<Vec<String>>().join(common::HEADERRPL);
                let h = format!("{}{}","# ", h_name);
                html_lines.push(h);
                header.clear();
                is_equal = false;
            }else if l.trim() == "" ||  l.trim().starts_with("#") || is_uniq_equal(l)  {
                header.reverse();
                let h_name = header.clone().into_iter().map(|x| x).collect::<Vec<String>>().join(common::HEADERRPL);
                let h = format!("{}{}","# ", h_name);
                html_lines.push(h);
                header.clear();
                if !is_uniq_equal(l){ is_equal = false; }
            }else{
                header.push(String::from(l));
            }
        }
    }
    if header.len() != 0{
        header.reverse();
        let h_name = header.clone().into_iter().map(|x| x).collect::<Vec<String>>().join(common::HEADERRPL);
        let h = format!("{}{}","# ", h_name);
        html_lines.push(h);
    }
    html_lines.reverse();
    let res = html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n");
    res
}
*/
pub fn get_header(md:&str) -> Vec<Header>{
    let lines:Vec<&str> = md.lines().collect();
    let mut headers = Vec::new();
    for l in lines{
        if !regex::Regex::new(r"^[#]{1,3} ").unwrap().is_match(l.trim()){continue;}
        let re = regex::Regex::new(r"^[#]{1,3} (?P<h>.*)").unwrap();
        for caps in re.captures_iter(l.trim()) {
            let h = caps.get(1).map_or("", |m| m.as_str());
            let cnt = if l.trim().starts_with("###"){3}
            else if l.trim().starts_with("##"){2}
            else{1};
            let header = Header{
                number: cnt,
                name: h.to_string()
            };
            headers.push(header);
        }
    }
    let mut result_headers = Vec::new();
    headers.reverse();
    for h in headers{
        result_headers.push(h);
    }
    result_headers.reverse();
    result_headers
}

pub fn replace_header_name(header: &str) -> String{//ヘッダーの特殊文字を置き換える処理
    let mut n_header = header.to_string();
    let b = regex::Regex::new(r".*<font color=.*</font>.*").unwrap().is_match(&n_header);
    if b{
        let mut front = String::new();
        let mut title = String::new();
        let mut back = String::new();
        let re = regex::Regex::new(r#"(?P<front>.*)<font color=.*>(?P<title>.*)</font>(?P<back>.*)"#).unwrap();
        for caps in re.captures_iter(&n_header) {
            front = String::from(&caps["front"]);
            title = String::from(&caps["title"]);
            back = String::from(&caps["back"]);
        }
        n_header = format!("{}{}{}",front,title,back);
    }
    let re = regex::Regex::new(r"[=|+|\-|*|.|:|/|]+?").unwrap();
    let r_name = re.replace_all(&n_header, "_").to_string();
    let re = regex::Regex::new(r"[(|)|\s|#]+?").unwrap();
    let r_name = re.replace_all(&r_name, "").to_string(); 
    let r_name = r_name.replace("!","&iexcl;");
    r_name
}

pub fn get_toc_menu(headers: Vec<Header>) -> String{
    let mut res = Vec::new();
    res.push(String::from(r#""#));
    res.push(String::from(r#"</div>"#));
    res.push(String::from(r#"<div id="ui-toc-affix" class="ui-affix-toc ui-toc-dropdown unselectable hidden-print" data-spy="affix" style="top:17px;display:none;">"#));
    res.push(String::from(r#"<div class="toc">"#));
    res.push(String::from(r#"<ul class="nav">"#));
    let mut back_uili = Vec::new();
    for (i, h) in headers.iter().enumerate(){
        let r_name = replace_header_name(&h.name);
        if i != 0 && h.number < headers[i-1].number{
            let num  = headers[i-1].number - h.number;
            for _ in 0..num{
                res.push(String::from(r#"</ul></li>"#));
                back_uili.pop();
            }
        }
        if i != headers.len()-1{
            if h.number < headers[i+1].number{
                let r = format!(r#"<li class=""><a href="{0}{1}" title="{2}">{2}</a><ul class="nav">"#,"#",r_name, h.name);
                res.push(r);
                back_uili.insert(0, String::from(r#"</ul></li>"#));
            }else{
                let r = format!(r#"<li class=""><a href="{0}{1}" title="{2}">{2}</a></li>"#,"#",r_name, h.name);
                res.push(r);
            }
        }else{
            let r = format!(r#"<li class=""><a href="{0}{1}" title="{2}">{2}</a></li>"#,"#",r_name, h.name);
            res.push(r);
        }
    }
    for b in back_uili{ res.push(b); }
    res.push(String::from("</ul>"));
    res.push(String::from("</div>"));
    res.push(String::from("</div>"));
    let tmp = res.into_iter().map(|x| x).collect::<Vec<String>>().join("\n");
    return tmp;
}