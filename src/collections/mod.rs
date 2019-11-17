
mod traits;
use traits::TemplateGeneratable;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItunesCollectionInfo {
    wrapper_type: String,
    collection_type: String,
    artist_id: u32,
    collection_id: u32,
    amg_artist_id: u32,
    artist_name: String,
    collection_name: String,
    collection_censored_name: String,
    artist_view_url: String,
    collection_view_url: String,
    artwork_url_60: String,
    artwork_url_100: String,
    collection_price: f32,
    collection_explicitness: String,
    content_advisory_rating: String,
    track_count: u32,
    copyright: String,
    country: String,
    currency: String,
    release_date: String, // TODO: date
    primary_genre_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItunesSearchRes {
    result_count: u32,
    results: Vec<ItunesCollectionInfo>,
}

impl ItunesCollectionInfo {
    pub fn extract_print(&self) {
        println!("{{\n\t{}\n\t{}\n\t{}\n\t{}\n}}", self.artist_name, self.collection_name, self.artwork_url_100, self.release_date);
    }
}

impl ItunesSearchRes {
    pub fn print_contents(&self) {
        for (idx, col) in self.results.iter().enumerate() {
            print!("{}: ", idx);
            println!("template:\n{}", col.generate_template());
            col.extract_print();
        }
    }
}

impl TemplateGeneratable for ItunesCollectionInfo {
    fn generate_template(&self) -> String {
        let res = format!("## {0}
<img src=\"{3}\" alt=\"album jacket for {0} by {1}\">
- Artist: {1}
- Release date: {2}
- Genre: (user input)
- Link: {4}

### Review

Lorem ipsum ...", 
            self.collection_name, 
            self.artist_name, 
            self.release_date, 
            self.artwork_url_100, 
            self.collection_view_url);
        res
    }
}
