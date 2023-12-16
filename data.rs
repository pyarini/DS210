use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug)]
// struct representing movie information with title, composer, and genre
pub struct MovieInfo {
    pub title: String,
    pub composer: String,
    pub genre: String,
}
impl MovieInfo {
    fn new(title: String, composer: String, genre: String) -> Self {
        MovieInfo {title, composer, genre}
    }
}
// read CSV file and extract movie information
pub fn read_csv(file_name: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    let mut titles: Vec<String> = Vec::new();
    let mut composers: Vec<String> = Vec::new();
    let mut genres: Vec<String> = Vec::new();

    // iterate thru every record in csv file and convert all titles, composers, and genres into strings
    for result in reader.records() {
        let record = result?;
        if let (Some(title), Some(composer), Some(genre)) = (record.get(0), record.get(1), record.get(2)) {
            titles.push(title.to_string());
            composers.push(composer.to_string());
            genres.push(genre.to_string());
            let mut movie_info = MovieInfo::new(title.to_string(), composer.to_string(), genre.to_string());
            movie_info.title = title.to_string();
            movie_info.composer = composer.to_string();
            movie_info.genre = genre.to_string()
        }
    }
    println!("Titles: {:?}", titles);
    println!("Composers: {:?}", composers);
    println!("Genres {:?}", genres);
    Ok((titles, composers, genres))
}