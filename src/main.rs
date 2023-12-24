use std::env;
use std::io;
use webbrowser;

const HELP: &str = "[-g google <search promt>] [-y youtube <search promt>] 
[-w browser <website>] [-t twitch <channel>]";

fn web_transform(url: &Vec<String>) -> String {
    let mut transformed = String::new();
    for element in url.iter().skip(2) {
        transformed.push_str(&element.to_string());
        transformed.push('+');
    }
    return transformed;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{HELP}");
        std::process::exit(1);
    }

    let mut url = String::new();
    match args[1].as_str() {
        "-g" => {
            url.push_str("https://www.google.com/search?q=");
            url.push_str(&web_transform(&args));
            webbrowser::open(&url)?;
        }
        "-y" => {
            url.push_str("https://www.youtube.com/results?search_query=");
            url.push_str(&web_transform(&args));
            webbrowser::open(&url)?;
        }
        "-w" => {
            url.push_str("https://www.");
            url.push_str(&args[2]);
            webbrowser::open(&url)?;
        }
        "-t" => {
            url.push_str("https://www.twitch.tv/");
            url.push_str(&args[2]);
            webbrowser::open(&url)?;
        }
        _ => webbrowser::open(&args[1])?,
    };

    Ok(())
}
