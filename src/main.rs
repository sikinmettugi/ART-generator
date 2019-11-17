#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;

// use std::io::Read;
mod collections;
mod webapp;

use collections::{ ItunesSearchRes };
use webapp::{ get_review_template };
// impl ItunesCollectionInfo {
//     pub fn extract_print(&self) {
//         println!("{{\n\t{}\n\t{}\n\t{}\n\t{}\n}}", self.artist_name, self.collection_name, self.artwork_url_100, self.release_date);
//     }
// }

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::get("https://itunes.apple.com/search?term=numb333rs&country=us&media=music&entity=album")?;
    let search_res: ItunesSearchRes = res.json()?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    // println!("search_res:\n{:?}", search_res);

    search_res.print_contents();

    Ok(())
}