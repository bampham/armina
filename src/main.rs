use std::env;
use std::io;
use webbrowser;

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
    let mut url = String::new();

    match args[1].as_str() {
        "-g" => {
            url.push_str("https://www.google.com/search?q=");
            url.push_str(&web_transform(&args));
            webbrowser::open(&url)?;
        }
        "-y" => {
            url.push_str("https://www.google.com/search?q=");
            url.push_str(&web_transform(&args));
            webbrowser::open(&url).expect("failed to open URL");
        }
        _ => webbrowser::open(&args[1])?,
    };

    Ok(())
}
