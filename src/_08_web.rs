extern crate reqwest;

pub fn main() {
    get_demo();
}

fn get_demo() {
    let url = "https://www.google.com/dshfldh";
    match reqwest::get(url) {
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::OK {
                println!("request is answered, here is the response length:\n{}",
                         response.text().expect("THERE IS NO RESPONSE").len());
            } else {
                println!("response status was \"{}\" instead of being ok.", response.status());
            }
        },
        Err(_) => { eprintln!("error in getting the home page"); }
    }

    // shorter version
    let response_text = reqwest::get(url).expect("could not fetch the url")
        .text().expect("could not read the response text");
    println!("response text:\n{}", response_text);
}
