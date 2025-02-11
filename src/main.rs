mod grade;

use clap::Parser;
use dialoguer::{Input, MultiSelect, Select};
use dialoguer::theme::ColorfulTheme;
use serde::{Serialize, Deserialize};

use grade::search_grade;

#[tokio::main]
async fn main() {

    // #[derive(Parser, Debug)]
    // #[command(author, version = "0.1.0", about="一个小工具🔧")]
    // struct Args {
    //     #[arg[short, long, required = true]]
    //     url: String,
    //
    //     #[arg(short, long, default_value = "master")]
    //     branch: String,
    // }

    // let app = Args::parse();
    interactive_selection().await;
}

async fn interactive_selection(){
    // struct ChooseStruct {
    //     name: String,
    //     child: Vec<>
    // }
    let options = vec!["成绩查询📖", "thinking..."];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a template")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let user_input = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("输入cookie：")
                .interact()
                .unwrap();
            search_grade(user_input).await;
        },
        _=>{
            println!("{:?}模版当前仍在开发中，请重新选择", options[selection]);
        }
    }
}
