// imports
use reqwest;

/**
 * Loads de HTML page from the url passed as parameter.
 * Returns HTTP code and body text, if applicable.
 */
fn load_page(url: &str) -> Result<(u16, String), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let status = response.status();
    let body = response.text()?;

    Ok((status.as_u16(), body))
}

/**
 * Main function.
 * Make request for https://checkip.dns.he.net and extract the IP address.
 * Print the IP address.
 */
fn main() {
    let url = "https://checkip.dns.he.net";
    let (status, body) = load_page(url).unwrap();

    if status == 200 {
        // if everything is ok, this will be received
        // <body>Your IP address is : 192.168.1.1</body>

        // extract only IP address digitis

        let ip_address = body
            .split("</body>")
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap();

        println!("{}", ip_address);
    } else {
        println!("Error: {}", status);
    }
}
