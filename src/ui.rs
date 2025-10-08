
use dioxus::prelude::*;
use dioxus::desktop::use_wry_event_handler;
use super::app::*;

#[component]
pub fn ui() -> Element {
    let mut app = use_signal(|| App::default());

    use_effect(move || { use_effect_download_codimd(app) });//CodiMDのページをダウンロード

    use_wry_event_handler(move |event, _| { event_handler(event, app); });//ウィンドウ変更時の記録用処理
    rsx! {
        div {
            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "convert_url:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "ダウンロードしたいCodiMDのページURLを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.convert_url = e.value(),
                    value: app().json.convert_url
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "codimd_json:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "CodiMDの全ページ情報をJsonで表示するサイトのURLを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.codimd_json = e.value(),
                    value: app().json.codimd_json
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "url_path:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "codimd/package.json の urlPath を設定している場合は、同じ文字を指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.url_path = e.value(),
                    value: app().json.url_path
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "user_name:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "CodiMDの全ページ情報をJsonで表示するサイトをユーザー認証している場合は、ユーザー名を指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.user_name = e.value(),
                    value: app().json.user_name
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "password:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "CodiMDの全ページ情報をJsonで表示するサイトをユーザー認証している場合は、パスワードを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.password = e.value(),
                    value: app().json.password
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "style:"
                }
                input{
                    style: "width: calc(100% - 120px); height: 20px;",
                    title: "ダウンロードしたときのhtmlのsytleを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.style = e.value(),
                    value: app().json.style
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "resource_path:"
                }
                input{
                    style: "width: calc(100% - 150px);",
                    title: "codimd/public/uploads/ のフルパスを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.resource_path = e.value().replace("\\","/"),
                    value: app().json.resource_path
                }
                button {  
                    style: "width: 30px;",
                    disabled: app().is_exec,
                    onclick: move |_| {
                        let path = native_dialog::DialogBuilder::file().open_single_dir().show();
                        if let Ok(res) = path{
                            if let Some(p) = res{
                                app.write().json.resource_path = format!("{}/", &p.display().to_string().replace("\\","/"));
                            }
                        }
                    },
                    "..."
                }
            }

            div {
                style: "display: flex; margin: 5px;",
                label{
                    style: "width: 120px; text-align: end; height: 15px;",
                    "output_path:"
                }
                input{
                    style: "width: calc(100% - 150px);",
                    title: "出力先のフルパスを指定してください",
                    disabled: app().is_exec,
                    oninput: move |e| app.write().json.output_path = e.value().replace("\\","/"),
                    value: app().json.output_path
                }
                button {  
                    style: "width: 30px;",
                    disabled: app().is_exec,
                    onclick: move |_| {
                        let path = native_dialog::DialogBuilder::file().set_location(&app().json.output_path).open_single_dir().show();
                        if let Ok(res) = path{
                            if let Some(p) = res{
                                app.write().json.output_path = p.display().to_string().replace("\\","/");
                            }
                        }
                    },
                    "..."
                }
            }
            
            button { 
                style: "width: 100%;", 
                disabled: app().is_exec,
                onclick: move |_| {
                    app.write().is_exec = true;
                },
                "実行"
            }
        }
    }
}
