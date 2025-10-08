use dioxus::prelude::*;
use dioxus::desktop::*;
use dioxus::desktop::tao::event::Event;

use super::json::*;
use super::download::*;

#[derive(Debug, Clone)]
pub struct App{
    pub json: Json,
    pub is_exec: bool,
}
impl Default for App{
    fn default() -> App{
        App { 
            json: Json::new(), 
            is_exec: false,
        }
    }
}

pub fn use_effect_download_codimd(mut app: Signal<App>){
    if !app().is_exec{return;}
    let url = app().json.convert_url.to_string();
    let output_path = app().json.output_path.to_string();
    let mut cms = Vec::new();
    let json = app().json;
    let handler = std::thread::spawn(move ||{
        let _res = download_codimd(&url, &output_path, &json, &mut cms, &mut 0);
    });
    handler.join().unwrap();
    app.write().is_exec = false; 
}

pub fn event_handler<UserWindowEvent>(event: &Event<UserWindowEvent>, mut app: Signal<App>){
    if let Event::WindowEvent{//ウィンドウサイズ変更時の処理
        event: WindowEvent::Resized(size),
        ..
    } = event {
        app.write().json.wi.width = size.width;
        app.write().json.wi.height = size.height;
    }
    if let Event::WindowEvent{//ウィンドウ位置変更時の処理
        event: WindowEvent::Moved(pos),
        ..
    } = event {
        app.write().json.wi.pos_x = pos.x;
        app.write().json.wi.pos_y = pos.y;
    }
    if let Event::WindowEvent{//exe終了時に情報保存する処理
        event: WindowEvent::CloseRequested, 
        ..
    } = event {
        app().json.save();
    }  
}
