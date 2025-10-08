use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub url: String,
    pub shortid: String,
    pub content: String, 
    pub title: String,
    pub c_time: String,
    pub u_time: String,
}

#[derive(Debug, Clone)]
pub struct CodiMD{
    pub url: String,
    pub title: String,
    pub path: String,
}

impl Default for CodiMD{
    fn default() -> Self{
        CodiMD{
            url: String::from(""),
            title: String::from(""),
            path: String::from(""),
        }
    }
}

impl CodiMD{
    pub fn new(tmp_url: &str, output: &str, infos: &Vec<Info>) -> Self{
        let mut codimd = CodiMD::default();
        for i in infos{
            if !tmp_url.contains(&i.url){continue;}
            codimd.url = i.url.to_string();
            codimd.title = i.title.to_string();
            codimd.path = String::from(output);
        }
        codimd
    }
}