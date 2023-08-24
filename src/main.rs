//tbitsearch - A torrent search engine that scrapes bitsearch for results.

use std::rc::Rc;
use clap::Parser;
use colored::Colorize;
use markup5ever::rcdom::Node;
use reqwest::Url;
use soup::prelude::*;
use text_io::read;
use serde::Serialize;
use serde_json::to_string_pretty;

#[derive(Parser)]
#[clap(version)]
struct Cli {
    /// The query to search for
    //#[arg(short, long)]
    query: String,

    /// Sort by (relevance, seeders, leechers, data, size)
    #[arg(short, long, default_value = "relevance")]
    sort: String,

    /// The sort order (asc, desc)
    #[arg(short, long, default_value = "desc")]
    order: String,

    /// The category to search in (all, videos, software, music, games)
    #[arg(short, long, default_value = "all")]
    category: String,

    /// The number of pages to show at one time
    #[arg(short, long, default_value = "1")]
    pages: i32,

    /// Output the results in JSON format
    #[arg(short, long)]
    json: bool,
}

#[derive(Serialize)]
struct Torrent {
    name: String,
    magnet: String,
    torrent: String,
    leechers: String,
    seeders: String,
    size: String,
    date: String,
}

fn result_to_torrent(result: Rc<Node>) -> Option<Torrent> {
    let magnet = result.tag("a").class("dl-magnet").find();

    // If there is no magnet link, that means it has found a non-torrent, like an advert.
    if magnet.is_none() {
        return None;
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

    Some(Torrent {
        name,
        magnet,
        torrent,
        leechers,
        seeders,
        size,
        date,
    })
}

fn main() {
    let args = Cli::parse();

    if args.query.is_empty() {
        return;
    }

    let mut mpagecountdown = args.pages;
    let mut mpagebuffer: Vec<Torrent> = Vec::new();

    let mut page = 1;

    let category = match args.category.as_str() {
        "all" => ("", ""),
        "videos" => ("1", "2"),
        "software" => ("5", "1"),
        "music" => ("7", ""),
        "games" => ("6", "1"),
        _ => ("", ""),
    };

    //url encode the query
    let mut url = Url::parse("https://bitsearch.to/search").unwrap();

    let mut numresults: i32 = 0;
    let mut numpages: i32 = 0;

    loop {
        url.set_query(Some(
            format!(
                "q={}&page={}&sort={}&order={}&category={}&subcat={}",
                args.query, page, args.sort, args.order, category.0, category.1
            )
            .as_str(),
        ));

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

        let mut parsed: Vec<Torrent> = Vec::new();

        for result in results {
            match result_to_torrent(result) {
                Some(torrent) => parsed.push(torrent),
                None => continue,
            }
        }

        if mpagecountdown > 1 && args.pages > 1 {
            mpagebuffer.append(parsed.as_mut());
            mpagecountdown -= 1;
            page += 1;
            continue;
        } else if mpagecountdown == 1 && args.pages > 1 {
            mpagebuffer.append(&mut parsed);
            parsed.append(&mut mpagebuffer);
        }

        if parsed.len() == 0 && args.json {
            println!("[]");
            break;
        } else if parsed.len() == 0 {
            println!("No results found.");
            break;
        }

        if args.json {
            println!("{}", to_string_pretty(&parsed).unwrap());
            break;
        }

        for (i, torrent) in parsed.iter().enumerate() {
            println!(
                "{}: {}",
                (i + 1).to_string().bold(),
                torrent.name.underline()
            );

            println!(
                "Seeders: {} | Leechers: {} | Size: {} | Date: {}\n",
                torrent.seeders.bold(),
                torrent.leechers.bold(),
                torrent.size.bold(),
                torrent.date.bold()
            );
        }

        print!("Page {}/{}\nChoice (Press Enter for next):", page, numpages);
        let choice: String = read!("{}\n");

        match choice.parse::<usize>() {
            Ok(num) => {
                if (num.overflowing_sub(1).0) >= parsed.len() {
                    continue;
                }
                println!(
                    "\nName: {}\n\nTorrent Link: {}\n\nMagnet Link: {}",
                    parsed[num - 1].name.bold(),
                    parsed[num - 1].torrent.underline(),
                    parsed[num - 1].magnet.underline()
                );
                break;
            }
            Err(_) => {
                mpagecountdown = args.pages;
                mpagebuffer = Vec::new();
                if page == numpages {
                    continue;
                }
                page += 1;
                continue;
            }
        }
    }
}
