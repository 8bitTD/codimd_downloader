use super::define::*;
use super::line::*;
use super::header::*;

pub fn convert_html(md: &str) -> String{
    let html = md.to_string();
    let html = delete_space(&html);//スペースのみの行で、スペース部分を削除
    let html = convert_html_mermaid(&html);//マーメイド変換
    let html = convert_html_flow(&html);//フロー変換
    let html = convert_html_table(&html);//テーブル変換
    let html = convert_html_alert(&html);//アラート変換 ::info ::warning ::danger ::success ::spoiler
    let html = convert_html_code_number(&html);//コード(行数表示)変換
    
    let html = convert_html_code(&html);//コード変換
    let html = convert_html_toc(&html);//[toc]変換
    let html = convert_html_header(&html);//ヘッダー変換 #
    let html = convert_html_checkbox(&html);//チェックボックス変換
    let html = convert_html_image(&html);//画像変換
    let html = convert_html_quote(&html);//引用変換
    let html = convert_html_line(&html);//ライン変換
    let html = convert_html_list(&html);//リスト変換
    let html = convert_html_url(&html);//URL変換
    let html = convert_html_emozi(&html);//絵文字変換
    
    let html = convert_html_bold(&html);//太字変換**
    let html = convert_html_marker(&html);//マーカー変換==
    let html = convert_html_insert(&html);//挿入ライン変換++
    let html = convert_html_cancel(&html);//打消し変換~~
    let html = convert_html_italic(&html);//斜体変換
    let html = convert_html_youtube(&html);//youtube変換
    let html = convert_html_newline(&html);//改行変換
    let html = replace_code_post(&html);//一時的に置き換えていた文字列を元の文字に置き換え("Ò","=")("Ó","*")("Ô","+")("Õ","~")
    html
}

pub fn convert_html_toc(md :&str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let headers = get_header(&md);
    for l in lines{
        if l.trim() == "[toc]" || l.trim() == "[TOC]"{
            for t in get_toc(&headers){
                html_lines.push(t);
            }
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

fn get_toc(headers:&Vec<Header>) -> Vec<String>{
    let mut res:Vec<String> = Vec::new();
    res.push(String::from(r#"<div class="toc"><ul>"#));
    let all_num = headers.len();
    let mut back_uili = Vec::new();
    for (i,h) in headers.into_iter().enumerate(){
        let r_name = replace_header_name(&h.name);
        if i != 0 && h.number < headers[i-1].number{
            let num  = headers[i-1].number - h.number;
            for _ in 0..num{
                res.push(String::from(r#"</ul></li>"#));
                back_uili.pop();
            }
        }
        if i != all_num-1{
            if h.number < headers[i+1].number{
                let r = format!(r#"<li><a href="{0}{1}" title="{2}">{2}</a><ul>"#,"#",r_name, h.name);
                res.push(r);
                back_uili.insert(0, String::from(r#"</ul></li>"#));
            }else{
                let r = format!(r#"<li><a href="{0}{1}" title="{2}">{2}</a></li>"#,"#",r_name, h.name);
                res.push(r);
            }
        }else{
            let r = format!(r#"<li><a href="{0}{1}" title="{2}">{2}</a></li>"#,"#",r_name, h.name);
            res.push(r);
        }
    }
    for b in back_uili{ res.push(b); }
    res.push(String::from(r#"</div>"#));
    res
}

pub fn convert_html_newline(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_script = false;
    let mut is_code = false;
    let all_num = lines.len();
    let c_lines = lines.clone();
    for (i,l) in lines.into_iter().enumerate(){
        if l.starts_with(r#"<script type="text/javascript">"#){is_script = true;}
        if l.contains(r#"<pre><code>"#){is_code = true;}
        if is_script || is_code{
            let rs = format!("{}",l);
            html_lines.push(rs);
        }else{
            if i == all_num -1{ html_lines.push(format!("{}",l));}
            else if wildmatch::WildMatch::new("<h* id=*>").matches(c_lines[i+1]) || c_lines[i+1] == "<hr></hr>"{
                html_lines.push(format!("{}",l));
            }else{
                let b = is_line_newline(l);
                let rs = if b{
                    format!("{}{}",l,"<br>")
                }else{
                    format!("{}",l)
                };
                html_lines.push(rs);
            }
        }
        if l.starts_with(r#"</script>"#){is_script = false;}
        if l.contains(r#"</code></pre>"#){is_code = false;}
     }
    let result = html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n");
    result
}

fn delete_space(md :&str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_code = false;
    for l in lines{
        if l.trim().starts_with("```"){is_code = !is_code;}
        let rl = match is_code{
            true => { String::from(l)},
            _ => {String::from(l)}
        };
        html_lines.push(rl);
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_youtube(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    for l in lines{
        if wildmatch::WildMatch::new("{%youtube *%}").matches(l){
            let url = l.replace("{%youtube ","").replace("%}","");
            let r = format!(r#"<div class="youtube" data-videoid="{0}"><iframe webkitallowfullscreen="" mozallowfullscreen="" allowfullscreen="" src="https://www.youtube.com/embed/{0}" frameborder="0"></iframe></div>"#,&url);
            html_lines.push(r);
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_emozi(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_code = false;
    for l in lines{
        if l.contains("<pre><code>")  {is_code = true;}
        if l.contains("</code></pre>"){is_code = false;}
        if regex::Regex::new(r":[^/][!-~]{2,10}:").unwrap().is_match(l) && !is_code{
            let re = regex::Regex::new(r":(?P<h>.+?):").unwrap();
            let result = re.replace_all(l, r#"<img class="emoji" alt=":$h:" src="https://cdn.jsdelivr.net/npm/@hackmd/emojify.js@2.1.0/dist/images/basic/$h.png">"#);
            html_lines.push(result.to_string());
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_checkbox(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_check = false;
    for l in lines{
        if l.find("- [ ]").is_some()||l.find("- [x]").is_some(){
            if !is_check{
                html_lines.push(String::from("<ul>"));
                is_check = true;
            }
            let mozi = l.replace("- [ ]","").replace("- [x]","");
            if l.find("x").is_some(){
                let r = format!(r#"<li class="task-list-item"><input type="checkbox" class="task-list-item-checkbox" checked="" disabled="disabled"><label></label>{}</li>"#,&mozi);
                html_lines.push(r);
            }else{
                let r = format!(r#"<li class="task-list-item"><input type="checkbox" class="task-list-item-checkbox" disabled="disabled"><label></label>{}</li>"#,&mozi);
                html_lines.push(r);
            }
        }else{
            if is_check{ 
                html_lines.push(String::from("</ul>")); 
                is_check = false;
            }
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_italic(md: &str) -> String{
    let re = regex::Regex::new(r"\*(?P<h>.+?)\*").unwrap();
    let result = re.replace_all(md, "<em>$h</em>");
    result.to_string()
}

pub fn convert_html_cancel(md: &str) -> String{
    let re = regex::Regex::new(r#"\~\~(?P<h>.[\s\S]+?)\~\~"#).unwrap();
    let result = re.replace_all(md, "<s>$h</s>");
    result.to_string()
}

pub fn convert_html_bold(md: &str) -> String{
    let re = regex::Regex::new(r#"[\*]{2}(?P<h>.[\s\S]*?)[\*]{2}"#).unwrap();
    let result = re.replace_all(md, "<strong>$h</strong>");
    result.to_string()
}

pub fn convert_html_marker(md: &str) -> String{
    let re = regex::Regex::new(r#"(?s)==(?P<h>.[^ ][\s\S]*?[^ ])=="#).unwrap();
    let result = re.replace_all(md, "$g<mark>$h</mark>");

    result.to_string()
}

pub fn convert_html_insert(md: &str) -> String{
    let re = regex::Regex::new(r#"\+\+(?P<h>.[\s\S]+?)\+\+"#).unwrap();
    let result = re.replace_all(md, "$g<ins>$h</ins>");
    result.to_string()
}

pub fn convert_html_url(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    for l in lines{
        html_lines.push(line_exec(l));
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

fn convert_image(l: &str) -> String{
    let mut front = String::new();
    let mut url = String::new();
    let mut back = String::new();
    if wildmatch::WildMatch::new("*![*](*=*x*)*").matches(l){
        let mut rx = String::new();
        let mut ry = String::new();
        let re = regex::Regex::new(r#"(?P<front>.*)!\[.*\]\((?P<url>[^ ]+) =(?P<rx>[^x]+)x(?P<ry>[^\)]+)\)(?P<back>.*)"#).unwrap();
        for caps in re.captures_iter(l) {
            front = String::from(&caps["front"]);
            url = String::from(&caps["url"]);
            rx = String::from(&caps["rx"]);
            ry = String::from(&caps["ry"]);
            back = String::from(&caps["back"]);
        }
        let img_html = format!(r#"{}<img src="{}" alt="" class="md-image md-image" width="{}" height="{}">{}"#, front, url, rx, ry, back);
        return img_html;
    }else{
        let re = regex::Regex::new(r#"(?P<front>.*)!\[.*\]\((?P<url>[^\)]+)\)(?P<back>.*)"#).unwrap();
        for caps in re.captures_iter(l) {
            front = String::from(&caps["front"]);
            url = String::from(&caps["url"]);
            back = String::from(&caps["back"]);
        }
        let img_html = format!(r#"{}<img src="{}" alt="" class="md-image md-image">{}<br>"#, front, url, back);
        return img_html;
    };
}

pub fn convert_html_image(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    for l in lines{
        if is_image(l){
            let mut res = String::from(l);
            while is_image(&res){
                res = convert_image(&res);
            }
            html_lines.push(res);
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_table(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_table = false;// |のテーブル判定　最初かどうか
    let mut is_code = false;
    let mut table_lines:Vec<Vec<String>> = Vec::new();//テーブルの配列
    for l in lines{
        if l.trim().starts_with("```"){is_code = !is_code;}
        if !l.trim().starts_with("|"){//テーブル終了時の閉じる処理
            if is_table{
                create_table(&table_lines, &mut html_lines);
            }
            table_lines.clear();
            is_table = false;
        }
        if l.trim().starts_with("|")&&l.match_indices("|").count()>1 && !is_code{//テーブル
            if !is_table {//テーブルの一番上の項目
                let mut lines = Vec::new();
                let v_tmp:Vec<&str> = l.trim().split("|").collect();
                 let num = v_tmp.len();
                for (i,t) in v_tmp.into_iter().enumerate(){
                    if i==0||i==num-1{continue;}
                    lines.push(String::from(t));
                }
                table_lines.push(lines);
                is_table = true;
            }else{
                let mut lines = Vec::new();
                let v_tmp:Vec<&str> = l.trim().split("|").collect();
                let num = v_tmp.len();
                for (i,t) in v_tmp.into_iter().enumerate(){
                    if i==0||i==num-1{continue;}
                    lines.push(String::from(t));
                }
                table_lines.push(lines);
            }         
        }else{
            html_lines.push(String::from(l));
        }
    }
    if table_lines.len() != 0 && is_table{
        create_table(&table_lines, &mut html_lines);
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

fn create_table(table_lines:&Vec<Vec<String>> , html_lines:&mut Vec<String>){//テーブル化処理
    let mut als = Vec::new();
    let tmp_als = table_lines[1].clone();
    for t in &tmp_als{
        let al = if wildmatch::WildMatch::new("*:*:*").matches(t){
            "center"
        }else if wildmatch::WildMatch::new("*-:*").matches(t){
            "right"
        }else if wildmatch::WildMatch::new("*:-*").matches(t){
            "left"
        }else{
            "left"
        };
        als.push(al);
    }
    let mut cnt = 0;
    for (i,tt) in table_lines.iter().enumerate(){
        if i == 1{continue;}
        else if i == 0{
            html_lines.push(String::from("<table>"));
            html_lines.push(String::from("<thead>"));
            html_lines.push(String::from("<tr>"));
            for t in tt{
                let table = format!(r#"<th style="text-align:{}">{}</th>"#, als[cnt],t);
                html_lines.push(table);
                cnt = cnt + 1;
            }
            html_lines.push(String::from("</tr>"));
            html_lines.push(String::from("</thead>"));
            cnt = 0;
        }else{
             html_lines.push(String::from("<tr>"));
            for t in tt{
                let table = format!(r#"<td style="text-align:{}">{}</td>"#, als[cnt],t);
                html_lines.push(table);
                cnt = cnt + 1;
            }
            html_lines.push(String::from("</tr>"));
            cnt = 0;
        }
        if i == table_lines.len()-1{
            html_lines.push(String::from("</table>"));
            html_lines.push(String::from("</thead>"));
        }
    }
}

pub fn convert_html_alert(md: &str) -> String{
    let re = regex::Regex::new(r":::[ |　]?+info[^A-Z]?[\s](?P<h>.[\s\S]+?):::").unwrap();
    let result = re.replace_all(md, r#"<div class="alert alert-info">
$h</div>"#);
    let re = regex::Regex::new(r":::[ |　]?+danger[^A-Z]?[\s](?P<h>.[\s\S]+?):::").unwrap();
    let result = re.replace_all(&result, r#"<div class="alert alert-danger">
$h</div>"#);
    let re = regex::Regex::new(r":::[ |　]?+success[^A-Z]?[\s](?P<h>.[\s\S]+?):::").unwrap();
    let result = re.replace_all(&result, r#"<div class="alert alert-success">
$h</div>"#);
    let re = regex::Regex::new(r":::[ |　]?+warning[^A-Z]?[\s](?P<h>.[\s\S]+?):::").unwrap();
    let result = re.replace_all(&result, r#"<div class="alert alert-warning">
$h</div>"#);
    let re = regex::Regex::new(r":::[ |　]?+spoiler[^A-Z]?[\s](?P<h>.[\s\S]+?):::").unwrap();
    let result = re.replace_all(&result, r#"<details>
$h</details>"#);
    let res = result.to_string();
    res
}

pub fn convert_html_quote(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut quote_cnt = 0;//引用カウント
    let mut old_quote_cnt = 0;//1行前の引用カウント
    for l in lines{
        let mut q_html = String::new();
        if l.trim().starts_with(">") {//引用
            let cnt = l.match_indices(">").count();
            let mz = l.replacen(">","",cnt);
            let mz = line_exec(&mz);
            if old_quote_cnt == cnt{
                q_html = format!("{}{}",&mz,"<br/>");
            }else if old_quote_cnt > cnt{
                let sa = old_quote_cnt - cnt;
                html_lines.push(format!("{}","</blockquote>".repeat(sa)));
                quote_cnt = quote_cnt - sa;
                q_html = format!("{}",&mz);
            }else{
                if quote_cnt == 0{
                    q_html = match mz == ""{
                        true => {format!("{}{}","<blockquote>\n".repeat(cnt), &mz)},
                        _ => {format!("{}{}{}","<blockquote>\n".repeat(cnt), &mz,"<br/>")}
                    };
                }else{
                    q_html = format!("{}{}{}","<blockquote>\n",&mz,"<br/>");
                }
                quote_cnt = quote_cnt + cnt;
            }
            html_lines.push(q_html);
            old_quote_cnt = cnt;
        }else if l.trim() == ""{
            q_html.push_str(&"</blockquote>".repeat(quote_cnt));
            html_lines.push(q_html);
            quote_cnt = 0;
            old_quote_cnt = 0;
        }else{
            if quote_cnt != 0{
                q_html = "</blockquote>".repeat(quote_cnt);
                html_lines.push(q_html);
            }
            html_lines.push(String::from(l));
            quote_cnt = 0;
            old_quote_cnt = 0;
        }
    }
    if quote_cnt != 0{
         let q_html = "</blockquote>".repeat(quote_cnt);
        html_lines.push(q_html);
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

fn get_number_of_line(lines: &Vec<&str>, num: usize) -> usize{
    let mut cnt: usize = 0;
    let mut is_end = false;
    for (i,l) in lines.into_iter().enumerate(){
        if i <= num || is_end{continue;}
        let rl = l.replace(" ","");
        if rl == "```"{ 
            is_end = true;
            cnt = i-num; 
        }
    }
    cnt
}

pub fn convert_html_flow(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_flow = false;//flow中かどうかの判定
    let all_num = lines.len();
    let c_lines = lines.clone();
    let mut cnt_diagram = 1;
    for (i,l) in lines.into_iter().enumerate(){
        
        let rl = l.replace(" ","");
        if rl == "```flow"&&!is_flow{
            html_lines.push(format!(r#"<div class="panel panel-default">{0}"#, common::NOTBREAK));
            html_lines.push(format!(r#"<div class="panel-body">{0}"#, common::NOTBREAK));
            html_lines.push(format!(r#"    <div id="diagram{0}"></div>{1}"#, cnt_diagram, common::NOTBREAK)); 
            html_lines.push(format!(r#"    </div>{0}"#, common::NOTBREAK)); 
            html_lines.push(format!(r#"</div>{0}"#, common::NOTBREAK)); 
            html_lines.push(format!(r#"<script type="text/javascript">{0}"#, common::NOTBREAK));
            html_lines.push(format!(r#"    $(document).ready(function () {0}{1}"#,"{", common::NOTBREAK));
            html_lines.push(format!(r#"        var diagram = flowchart.parse({0}"#, common::NOTBREAK));
            is_flow = !is_flow;
        }else if rl == "```" && is_flow{
            html_lines.push(format!(r#"        );{0}"#, common::NOTBREAK));
            html_lines.push(format!(r#"        diagram.drawSVG('diagram{0}');{1}"#, cnt_diagram, common::NOTBREAK));
            html_lines.push(format!(r#"     {0});{1}"#,"}", common::NOTBREAK));
            html_lines.push(format!(r#"</script>{0}"#, common::NOTBREAK));
            is_flow = !is_flow;
            cnt_diagram += 1;
        }else{
            if is_flow{
                if i != all_num-1&& c_lines[i+1].replace(" ","") != "```"{
                    html_lines.push(format!(r#"'			{0}\n' +{1}"#,l, common::NOTBREAK));
                }else{
                    html_lines.push(format!(r#"'			{0}\n'{1}"#,l, common::NOTBREAK));
                }
            }else{
                html_lines.push(String::from(l));
            }
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_mermaid(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_mermaid = false;//mermaid中かどうかの判定
    for l in lines{
        let rl = l.replace(" ","");
        if rl == "```mermaid"&&!is_mermaid{
            let html_c = format!(r#"<pre class="mermaid"><code>"#);
            html_lines.push(html_c);
            is_mermaid = !is_mermaid;
        }else if rl == "```" && is_mermaid{
            let html_line = "</code></pre>";
            html_lines.push(String::from(html_line));
            is_mermaid = !is_mermaid;
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_code_number(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let c_lines = lines.clone();
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_code = false;//コード中かどうかの判定
    for (i,l) in lines.into_iter().enumerate(){
        let rl = l.replace(" ","");
        if wildmatch::WildMatch::new(r#"```*=*"#).matches(&rl)&&!is_code{
            let tmp_num:Vec<&str> = rl.split("=").collect();
            let num: usize = match tmp_num.last().unwrap().parse::<usize>().is_ok(){
                true => {tmp_num.last().unwrap().parse().unwrap()},
                _ => {1},
            };
            let html_line = format!(r#"<pre><code class=""><div class="wrapper"><div class="gutter linenumber"><span data-linenumber="{0}"></span>"#,num);
            html_lines.push(String::from(html_line));
            let c_num = get_number_of_line(&c_lines, i);
            for n in num+1..num+c_num-2{
                let html_c = format!(r#"<span data-linenumber="{0}"></span>"#,n);
                html_lines.push(html_c);
            }
            let html_c = format!(r#"<span data-linenumber="{0}"></span></div><div class="code">"#,num+c_num-2);
            html_lines.push(html_c);
            is_code = !is_code;
        }else if rl == "```" && is_code{
            let html_line = "</div></div></code></pre>";
            html_lines.push(String::from(html_line));
            is_code = !is_code;
        }else{
            if is_code{
                html_lines.push(replace_code_pre(&l));
            }else{
                html_lines.push(String::from(l));
            }
            
        }
    }
    let mut return_lines = String::new();
    for l in &html_lines{
        if wildmatch::WildMatch::new(r#"<span data-linenumber="*"></span></div><div class="code">"#).matches(&l){
            return_lines.push_str(&format!("{}",l));
        }else{
            return_lines.push_str(&format!("{}{}",l,"\n"));
        }
    }
    return_lines
}

fn replace_code_pre(md: &str) -> String{//コード中の特殊文字を一時的に別のものに置き換える処理
    md.replace("<","&lt;").replace(">","&gt;").replace("=","Ò").replace("*","Ó").replace("+","Ô").replace("~","Õ")
    .replace("_","Ñ")
    .replace("#","Ð")
    .replace("-","ℜ")
    .replace("!", " 𝒻")
}

fn replace_code_post(md: &str) -> String{//コード中の特殊文字を一時的に別のものに置き換える処理
    let result = md.replace("Ò","=").replace("Ó","*").replace("Ô","+").replace("Õ","~").replace("Ñ","_")
    .replace("Ð","#")
    .replace("ℜ","-")
    .replace(" 𝒻","!");
    //コード中の改行を削除
    let re = regex::Regex::new(r"<pre><code>[\s]{1,}").unwrap();
    let result = re.replace_all(&result, "<pre><code>");
    let re = regex::Regex::new(r"[\s]{1,}</code></pre>").unwrap();
    let result = re.replace_all(&result, "</code></pre>");
    return result.to_string();
}

pub fn convert_html_code(md: &str) -> String{
    //  ```
    //  aaa
    //  bbb
    //  ```
    //↑の``` ```部分を<pre><code> </code></pre>に変換する処理
    let md = if regex::Regex::new(r"  `{3}\n").unwrap().is_match(md){
        let re = regex::Regex::new(r"  `{3}\n(?P<h>.[\s\S]+?)  `{3}").unwrap();
        let result = re.replace_all(md,
            |caps: &regex::Captures| {
                let res = caps.get(1).unwrap().as_str();
                format!("<pre><code>{0}</code></pre>", replace_code_pre(&res.replace("  ","")))
            }
        );
        result.to_string()
    }else{
        md.to_string()
    };
    
    //通常の複数行コードの変換処理
    let re = regex::Regex::new(r"[`]{3}(?P<h>.*)[`]{3}\n").unwrap();
    let result = re.replace_all(&md,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!("<pre><code>{0}</code></pre>\n", replace_code_pre(res))
        }
    );
    let mut html_lines:Vec<String> = Vec::new();
    let mut is_code = false;//コード中かどうかの判定
    let result = result.to_string();
    let lines:Vec<&str> = result.lines().collect();
    for l in lines{
        let rl = l.replace(" ","");
        if wildmatch::WildMatch::new(r#"```*"#).matches(&rl)&&!is_code{
            let html_line = format!(r#"<pre><code>"#);
            html_lines.push(String::from(html_line));            
            is_code = !is_code;
        }else if rl == "```" && is_code{
            let html_line = "</code></pre>";
            html_lines.push(String::from(html_line));
            is_code = !is_code;
        }else{
            if is_code{
                html_lines.push(replace_code_pre(&l));
            }else{
                html_lines.push(String::from(l));
            }
        }
    }
    let result = html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n");
    let re = regex::Regex::new(r"[`](?P<h>.+?)[`]").unwrap();
    let result = re.replace_all(&result,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!("<code>{0}</code>", replace_code_pre(res))
        }
    );
    result.to_string()
}

pub fn convert_html_line(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines:Vec<String> = Vec::new();
    for l in lines{
        let mut r:Vec<&str> = l.split("").collect();
        r.sort();
        r.dedup();
        let res = r.into_iter().map(|x| x).collect::<Vec<&str>>().join("");
        let res = res.replace(" ","").replace("	","");
        if res == "-" || res == "*" || res == "_"{
            let hr_html = format!("<hr></hr>");
            html_lines.push(hr_html);
        }else{
            html_lines.push(String::from(l));
        }
    }
    html_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_list(md: &str) -> String{
    let lines:Vec<&str> = md.lines().collect();
    let mut html_lines1:Vec<String> = Vec::new();
    let mut indent_count = 0;
    let mut old_indent_count = 0;
    let mut is_code = false;
    for l in lines{
        let res = is_line_list(l);
        if res.is_some(){
            let count = res.unwrap();
            if indent_count < count && indent_count < 6{//リスト化 * aaa → <li> aaa
                indent_count += 1;
                html_lines1.push(String::from("<ul>"));
                let rs = line_list(l);
                html_lines1.push(rs);
            }else if indent_count > count {
                indent_count = count;
                let sa = old_indent_count - indent_count;
                for _ in 0..sa{ 
                    html_lines1.push(String::from("</ul></li>")); 
                }
                let rs = line_list(l);
                html_lines1.push(rs);
            }else if indent_count == count{
                html_lines1.push(String::from("</li>"));
                let rs = line_list(l);
                html_lines1.push(rs);
            }
        }else if l.trim().starts_with("<pre><code>") || l.trim().starts_with("</code></pre>") 
              || l.trim().starts_with("<code>") || l.trim().starts_with("</code>"){
            is_code = !is_code;
            html_lines1.push(String::from(l));
        }else if l.trim().starts_with("<div class=") {
            indent_count = 0;
            let sa = old_indent_count - indent_count;
            for _ in 0..sa{ html_lines1.push(String::from("</ul></li>")); }
            html_lines1.push(String::from(l));
        }else if l.trim() == "</div>"{            
            indent_count = 0;
            let sa = old_indent_count - indent_count;
            for _ in 0..sa{ html_lines1.push(String::from("</ul></li>")); }
            html_lines1.push(String::from(l));
        }else if l.trim().starts_with("<img") || l.trim().starts_with("<blockquote>") || l.trim().starts_with("</blockquote>") {
            html_lines1.push(String::from(l));
        }else if l.trim().starts_with("<") {
            indent_count = 0;
            let sa = old_indent_count - indent_count;
            for _ in 0..sa{ html_lines1.push(String::from("</ul></li>")); }
            html_lines1.push(String::from(l));
        }else if l == ""{
            indent_count = 0;
            let sa = old_indent_count - indent_count;
            for _ in 0..sa{ html_lines1.push(String::from("</ul></li>")); }
            html_lines1.push(String::from(l));
        }else{
            html_lines1.push(String::from(l));
        }
        old_indent_count = indent_count;
    }
    for _ in 0..indent_count{ html_lines1.push(String::from("</ul></li>")); }
    let mut html_lines2:Vec<String> = Vec::new();
    for l in html_lines1{
        if l.contains(">* "){
            let (f,s) = l.rsplit_once(">").unwrap();
            let rs = line_list(s);
            let res = format!("{}{}{}",f,">", rs);
            html_lines2.push(res);
        }else{
            html_lines2.push(l);
        }
    }
    html_lines2.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}

pub fn convert_html_header(md: &str) -> String{
    
    let re = regex::Regex::new(r"(?m)^[|> |>　]?+###### (?P<h>.*)").unwrap();
    let h6 = re.replace_all(md,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h6 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h6>
"#,"#",&replace_header_name(&res),res)
        }
    );
    let re = regex::Regex::new(r"(?m)^[|> |>　]?+##### (?P<h>.*)").unwrap();
    let h5= re.replace_all(&h6,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h5 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h5>
"#,"#",&replace_header_name(&res),res)
        }
    );
    
    let re = regex::Regex::new(r"(?m)^[|> |>　]?+#### (?P<h>.*)").unwrap();
    let h4= re.replace_all(&h5,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h4 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h4>
"#,"#",&replace_header_name(&res),res)
        }
    );
    
    let re = regex::Regex::new(r"(?m)^[|> |>　]?+### (?P<h>.*)").unwrap();
    let h3= re.replace_all(&h4,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h3 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h3>
"#,"#",&replace_header_name(&res),res)
        }
    );
    
    let re = regex::Regex::new(r"(?m)^[|> |>　]?+##[\s|\S](?P<h>.*)").unwrap();
    let h2= re.replace_all(&h3,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h2 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h2>
"#,"#",&replace_header_name(&res),res)
        }
    );
    
    let re = regex::Regex::new(r"^[|> |>　]?+# (?P<h>.*)").unwrap();
    let h1= re.replace_all(&h2,
        |caps: &regex::Captures| {
            let res = caps.get(1).unwrap().as_str();
            format!(r#"
<h1 id="{1}"><a class="anchor hidden-xs" href="{0}{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h1>
"#,"#",&replace_header_name(&res),res)
        }
    );
    h1.to_string()
}

pub fn copy_resource(res:&Vec<String>, path:&str){//リソースをimgフォルダにコピーする処理
    for r in res{
        let rsc = std::path::Path::new(&r);
        let name = rsc.file_name().unwrap().to_str().unwrap();
        let pst = format!("{}{}{}",path,"/",name);
        let _rst = std::fs::copy(r, &pst);
    }
}

pub fn get_resource_path(html:&str, url_path: &str, resource_path: &str) -> Vec<String>{//htmlからリソースパスを取得する処理
    let mut res = Vec::new();
    let uploads = match url_path.is_empty(){
        true => {"/uploads/"},
        _ => {&format!("/{}/uploads/", url_path)}
    };
    //画像を取得![]()
    let mozi:Vec<&str> = html.lines().collect();
    for m in mozi{
        if !wildmatch::WildMatch::new(r#"*![*](*.*)*"#).matches(m) {continue;}
        let tmp:Vec<&str> = m.split("(").collect();
        for t in tmp{
            let rs = t.split_once(")");
            if rs == None{continue;}
            let (r,_) = rs.unwrap();
            //if r.starts_with(common::UPLOADS){
            if r.starts_with(uploads){
                let tmp = match r.contains("="){
                    true =>{
                        let (f,_s) = r.split_once("=").unwrap();
                        f.replace(" ","").replace(uploads, resource_path)
                    },
                    _ => {
                        r.replace(uploads, resource_path)
                    }
                };
                res.push(tmp);
            }
        }
    }
    //画像を取得<img src=...>
    let mozi:Vec<&str> = html.lines().collect();
    for m in mozi{
        if !wildmatch::WildMatch::new(r#"*<img*src=*/uploads/*>*"#).matches(m) {continue;}
        let tmp:Vec<&str> = m.split(" ").collect();
        for t in tmp{
            let rs = t.split_once(r#"src=""#);
            if rs == None{continue;}
            let (_,r) = rs.unwrap();
            
            let r = match r.contains(r#"""#){
                true => {
                    let rs = r.split_once(r#"""#);
                    let (r,_) = rs.unwrap();
                    r
                },
                _ => r
            };
            
            if r.starts_with(uploads){
                let tmp =r.replace(uploads,resource_path);
                res.push(tmp);
            }
        }
    }
    //.mp4を取得
    let mozi:Vec<&str> = html.split(r#"""#).collect();
    for m in mozi{
        if !m.contains(uploads)||!m.contains(".mp4"){continue;}
        if m.starts_with(uploads){
            let tmp = m.replace(uploads,resource_path);
            res.push(tmp);
        }
    }
    let mozi:Vec<&str> = html.split(r#"'"#).collect();
    for m in mozi{
        if !m.contains(uploads)||!m.contains(".mp4"){continue;}
        if m.starts_with(uploads){
            let tmp = m.replace(uploads, resource_path);
            res.push(tmp);
        }
    }
    //pdfを取得
    let mozi:Vec<&str> = html.lines().collect();
    for m in mozi{
        if !wildmatch::WildMatch::new(r#"*<iframe*.pdf*</iframe>*"#).matches(m){continue;}
        let tmp:Vec<&str> = m.split(" ").collect();
        for t in tmp{
            if !wildmatch::WildMatch::new(r#"*src="*.pdf"*"#).matches(t) || !t.contains(uploads){continue;}
            let f = t.find("/codimd/uploads/").unwrap();
            let s = t.find(".pdf").unwrap();
            let r = &t[f..s+4];
            let tmp = r.replace(uploads, resource_path);
            res.push(tmp);
        }
    }
    res
}

pub fn replace_path(md: &str, url_path: &str) -> String{//パス変換
    let mut img_lines = Vec::new();
    let uploads = match url_path.is_empty(){
        true => {"/uploads/"},
        _ => {&format!("/{}/uploads/", url_path)}
    };
    //画像を取得![]()
    let mozi:Vec<&str> = md.lines().collect();
    for m in mozi {
        if wildmatch::WildMatch::new("*![*](*)*").matches(m){
            if m.contains(uploads){
                let tmp = m.replace(uploads,common::RESOURCE);
                img_lines.push(tmp);
            }else{
                img_lines.push(String::from(m));
            }
        }else{
            img_lines.push(String::from(m));
        }
    }
    //画像を取得<img src=...>
    let mut img_lines2 = Vec::new();
    for m in img_lines{
        if wildmatch::WildMatch::new(r#"*<img*src=*/uploads/upload_*>*"#).matches(&m){
            let tmp = m.replace(uploads,common::RESOURCE);
            img_lines2.push(tmp);
        }else{
            img_lines2.push(String::from(m));
        }
    }
    //mp4のパスを変換
    let mut mve_lines = Vec::new();
    for m in img_lines2{
        if m.contains(uploads)&&m.contains(".mp4")&&m.contains("src="){
            let tmp = m.replace(uploads,common::RESOURCE);
            mve_lines.push(tmp);
            
        }else{
            mve_lines.push(m);
        }
    }
    //pdfのパスを変換
    let mut pdf_lines = Vec::new();
    for m in mve_lines{
        if wildmatch::WildMatch::new(r#"*<iframe*.pdf*</iframe>*"#).matches(&m){
            let tmp = m.replace(uploads,common::RESOURCE);
            pdf_lines.push(tmp);
        }else{
            pdf_lines.push(m);
        }
    }
    pdf_lines.into_iter().map(|x| x).collect::<Vec<String>>().join("\n")
}
