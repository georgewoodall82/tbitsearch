//tbitsearch - A torrent search engine that scrapes bitsearch for results.

use reqwest::Url;
use soup::prelude::*;
use text_io::read;

fn main() {
    //Get program args
    let args: Vec<String> = std::env::args().collect();
    //Merge all args into one string, except the first
    let mut query = args[1..].join(" ");

    //if no args were provided
    if query.is_empty() {
        print!("Bitsearch Search:");
        query = read!("{}\n");
    }

    let mut page = 1;

    //url encode the query
    let mut url = Url::parse("https://bitsearch.to/search").unwrap();
    url.set_query(Some(format!("q={}", query).as_str()));

    let mut numresults: i32 = 0;
    let mut numpages: i32 = 0;

    loop {
        let response = reqwest::blocking::get(url.clone()).expect("Failed to reach bitsearch.to");
        let souplol = Soup::from_reader(response).unwrap();
        let results = souplol
            .tag("li")
            .class("search-result")
            .class("card")
            .find_all();

        // If numresults has not already been set

        numresults = if page == 1 {
            souplol
                .tag("span")
                .class("w3-bar-item")
                .find()
                .unwrap()
                .tag("b")
                .find()
                .unwrap()
                .text()
                .parse()
                .expect("Failed to parse website.")
        } else {
            numresults
        };
        numpages = if page == 1 {
            (numresults as f32 / 20.0).floor() as i32
        } else {
            numpages
        };

        if numpages == 0 {
            numpages = 1;
        }

        struct Torrent {
            name: String,
            magnet: String,
            torrent: String,
            leechers: String,
            seeders: String,
            size: String,
            date: String,
        }

        let mut parsed: Vec<Torrent> = Vec::new();

        for result in results {
            let magnet = result.tag("a").class("dl-magnet").find();

            // If there is no magnet link, that means it has found an advert, so skip.
            if magnet.is_none() {
                continue;
            }

            let name = result
                .tag("h5")
                .class("title")
                .find()
                .unwrap()
                .tag("a")
                .find()
                .unwrap()
                .text();
            let magnet = magnet.unwrap().get("href").unwrap();
            let torrent = result
                .tag("a")
                .class("dl-torrent")
                .find()
                .unwrap()
                .get("href")
                .unwrap();
            let seeders = result
                .tag("img")
                .attr("alt", "Seeder")
                .find()
                .unwrap()
                .parent()
                .unwrap()
                .tag("font")
                .find()
                .unwrap()
                .text();
            let size = result
                .tag("img")
                .attr("alt", "Size")
                .find()
                .unwrap()
                .parent()
                .unwrap()
                .text();
            let date = result
                .tag("img")
                .attr("alt", "Date")
                .find()
                .unwrap()
                .parent()
                .unwrap()
                .text();
            let leechers = result
                .tag("img")
                .attr("alt", "Leecher")
                .find()
                .unwrap()
                .parent()
                .unwrap()
                .tag("font")
                .find()
                .unwrap()
                .text();
            parsed.push(Torrent {
                name: name,
                magnet: magnet,
                torrent: Url::parse(torrent.as_str()).unwrap().try_into().unwrap(),
                seeders: seeders,
                size: size,
                date: date,
                leechers: leechers,
            });
        }

        if parsed.len() == 0 {
            println!("No results found.");
            break;
        }

        for (i, torrent) in parsed.iter().enumerate() {
            println!("{}: {}", i + 1, torrent.name);

            println!(
                "Seeders: {} | Leechers: {} | Size: {} | Date: {}\n",
                torrent.seeders, torrent.leechers, torrent.size, torrent.date
            );
        }

        print!("Page {}/{}\nChoice (Press Enter for next):", page, numpages);
        let choice: String = read!("{}\n");

        let temp = choice.parse::<usize>();

        match temp {
            Ok(num) => {
                println!(
                    "\nName: {}\n\nTorrent Link: {}\n\nMagnet Link: {}",
                    parsed[num - 1].name,
                    parsed[num - 1].torrent,
                    parsed[num - 1].magnet
                );
                break;
            }
            Err(_) => {
                if page == numpages {
                    continue;
                }
                page += 1;
                url.set_query(Some(format!("q={}&page={}", query, page).as_str()));
                continue;
            }
        }
    }
}
