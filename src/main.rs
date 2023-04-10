use webbrowser;
use std::env;

fn web_transform(url: &Vec<String>) -> String {
    let mut transformed = String::new();
    for element in url.iter().skip(2) {
        transformed.push_str(&element.to_string());
        transformed.push('+');
    }
    return transformed
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut url = String::new();
    if &args[1] == "-g" {
        url.push_str("https://www.google.com/search?q=");
        url.push_str(&web_transform(&args));
        webbrowser::open(&url).expect("failed to open URL");
    }
    else if &args[1] == "-y" {
        url.push_str("https://www.youtube.com/results?search_query=");
        url.push_str(&web_transform(&args));
        webbrowser::open(&url).expect("failed to open URL");
    }
    else {
        webbrowser::open(&args[1]).expect("failed to open URL");
    }




}
