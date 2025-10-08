use super::download::*;
use super::header::*;


pub fn get_codimd_info(m: &str, codimd_url:&str, codimd_json: &str, user_name:&str, password: &str) -> (String, String){//CodiMDのページのURLと置き換えた一行文を取得
    let re = regex::Regex::new(r#"(?P<front>.*)\[(?P<title>[^\]]+)\]\((?P<url>[^\)]+)\)(?P<back>.*)"#).unwrap();
    let mut front = String::new();
    let mut title = String::new();
    let mut url = String::new();
    let mut header = String::new();
    let mut back = String::new();
    for caps in re.captures_iter(m) {
        front = String::from(&caps["front"]);
        title = String::from(&caps["title"]);
        url = String::from(&caps["url"]);
        back = String::from(&caps["back"]);
    }
    
    url = if url.contains(&format!("{}s/", codimd_url)){
        let shortid = url.replace(&format!("{}s/", codimd_url),"");
        get_url_from_shortid(&shortid, codimd_json, user_name, password)
    }else{
        url
    };
    url = if url.contains("#"){
        let (t_url,t_header) = url.split_once("#").unwrap();
        header =  String::from(t_header);
        t_url.to_string()
    }else{url};
    url = if url.contains("?"){
        let (t_url,_) = url.split_once("?").unwrap();
        t_url.to_string()
    }else{url};
    url = url.replace("?view","").replace("?both","");
    url = url.replace("-","+");
    let tt = get_title(&url, codimd_json, user_name, password);
    let html = match &header != ""{
        true => {
            format!("{}{}{}{}{}","./",tt,".html","#",replace_header_name(&header))
        },
        _ => {format!("{}{}{}","./",tt,".html")}
    };
    let line= format!("{}[{}]({}){}",front,title,html,back);
    (url, line)
}

pub fn line_exec(l: &str) -> String{
    let mut res = String::from(l);
    while regex::Regex::new(r".*\[.*\]\(.*\).*").unwrap().is_match(&res) || 
          regex::Regex::new(r"(?m)^https?://").unwrap().is_match(&res){
        res = line_url(&res);        
    };
    let res = line_code(&res);
    res
}

fn line_url(l: &str) -> String{
    let res = if regex::Regex::new(r"(?m)^https?://").unwrap().is_match(l.trim()){
        format!(r#"<a href="{0}" target="_blank" rel="noopener">{0}</a>"#, l.trim())
    }else if wildmatch::WildMatch::new("<h*](*)*fa fa-link*").matches(l){
        let header_num: usize = l[2..3].parse().unwrap();
        let (_,tmp) = l.split_once("](").unwrap();
        let (url,_) = tmp.split_once(")").unwrap();
        let (tmp,_) = l.split_once("](").unwrap();
        let (_,name) = tmp.split_once("[").unwrap();
        let url_html = format!(r#"<a href="{0}" target="_blank" rel="noopener">{1}</a>"#,url,name);
        let res = format!(r#"<h{0} id="{1}"><a class="anchor hidden-xs" href="/#{1}" title="{1}"><i class="fa fa-link"></i></a>{2}</h{0}>"#,header_num, name, url_html);
        res
    }else if regex::Regex::new(r".*\[.*\]\(.*\).*").unwrap().is_match(l){
        if regex::Regex::new(r".*\[.*\]\(.*\.html\).*").unwrap().is_match(l){
            let re = regex::Regex::new(r#"(?m)(?P<h>.*)\[(?P<i>.*?)\]\((?P<j>.+?)\.html\)(?P<k>.*)"#).unwrap();
            let result = re.replace_all(l, r#"$h<a href="$j.html" target="_blank" rel="noopener">$i</a>$k"#);
            result.to_string()
        }else{
            let re = regex::Regex::new(r#"(?P<h>.*)\[(?P<i>.*?)\]\((?P<j>.*?)\)(?P<k>.*)"#).unwrap();
            let result = re.replace_all(l, r#"$h<a href="$j" target="_blank" rel="noopener">$i</a>$k"#);
            result.to_string()
        }   
    }else{
        String::from(l)
    };
    res
}

fn line_code(l: &str) -> String{
    let re = regex::Regex::new(r"[`]{3}(?P<h>.*)[`]{3}").unwrap();
    let result = re.replace_all(l, r#"<code>$h</code>"#);
    result.to_string()
}

pub fn is_line_list(l :&str) -> Option<usize>{
    if       regex::Regex::new(r"       [*|+|-] ").unwrap().is_match(l){
        return Some(6);
    }else if regex::Regex::new(r"      [*|+|-] ").unwrap().is_match(l){
        return Some(5);
    }else if regex::Regex::new(r"     [*|+|-] ").unwrap().is_match(l){
        return Some(4);
    }else if regex::Regex::new(r"    [*|+|-] ").unwrap().is_match(l){
        return Some(3);
    }else if regex::Regex::new(r"  [*|+|-] ").unwrap().is_match(l){
        return Some(2);
    }else if regex::Regex::new(r"^[*|+|-] ").unwrap().is_match(l){
        return Some(1);
    }
    None
}

pub fn line_list(l: &str) -> String{
    let re = regex::Regex::new(r"[\s]?[+|*|-]{1}(?P<h>.*)").unwrap();
    let result = re.replace_all(l, "<li>$h");
    result.to_string()
}

pub fn is_line_newline(l: &str) -> bool{
    let mut is_nl = false;
    if l.ends_with("</strong>"){is_nl = true;}
    else if l.ends_with("</s>"){is_nl = true;}
    else if l.ends_with("</font>"){is_nl = true;}
    else if l.ends_with("</code>"){is_nl = true;}
    else if l.ends_with("</a>"){is_nl = true;}
    else if l.ends_with("</ins>"){is_nl = true;}
    else if l.ends_with(r#".png">"#){is_nl = true;}
    else if l.ends_with(r#"class="md-image md-image">"#){is_nl = true;}
    else if l.ends_with(r#"</mark>"#){is_nl = true;}
    //else if regex::Regex::new(r"[^\x01-\x7E]").unwrap().is_match(l){is_nl = true;}
    else if !l.ends_with(">") && !l.contains("iframe") && l.trim() != "" {is_nl = true;}
    else if wildmatch::WildMatch::new("*<img src=*>").matches(l){is_nl = true;}
    is_nl
}

pub fn is_image(l: &str) -> bool{
    let mut res = false;
    if regex::Regex::new(r".*!\[.*\]\(.* =.*x.*\).*").unwrap().is_match(l){
        res = true;
    }
    if regex::Regex::new(r".*!\[.*\]\(.*\).*").unwrap().is_match(l){
        res = true;
    }
    res
}