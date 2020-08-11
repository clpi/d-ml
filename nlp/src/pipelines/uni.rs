use std::collections::HashMap;
//use timely::{.communication::}
use rayon::prelude::*;
use petgraph::{Graph, prelude::*};
use serde::{Serialize, Deserialize};
use std::io::BufReader;
use std::fs::{ File,
    read_to_string, read, write, read_dir, create_dir_all
};
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use csv::{Reader, Writer, ReaderBuilder, WriterBuilder};

#[derive(Serialize, Deserialize)]
pub struct UniversityData {
    name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    domains: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    web_pages: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    alpha_two_code: Option<String>,
    country: String,
    #[serde(rename="state-province", skip_serializing_if="Option::is_none")]
    state_province: Option<String>,
}

pub async fn get_uni_pages(country: &str) -> Vec<String> {
    let file = BufReader::new(File::open("uni/uni.json").unwrap());
    let mut unis: Vec<UniversityData> = serde_json::from_reader(file).unwrap();
    let mut links: Vec<String> = Vec::with_capacity(unis.len());
    let mut out = csv::Writer::from_path("uni/out.csv").unwrap();
    unis.into_iter().filter(|uni| uni.web_pages.is_some()).enumerate().for_each(|(i, uni)|  {
        if uni.country.eq(country) { 
            let link = uni.web_pages.clone().unwrap().get(0).unwrap().to_string();
            links.push(link.clone());
            out.write_record(&[uni.name, uni.country, link]).unwrap();
        }
    });
    links
}

pub async fn visit_unis(links: Vec<String>) -> reqwest::Result<()> {
    let mut classes: Vec<String> = Vec::new();
    let mut webs: Vec<DiGraph<String, i32>> = Vec::new();
    let mut courses: Vec<DiGraph<String, i32>> = Vec::new();
    let mut graph: DiGraph<String, i32> = DiGraph::new();
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;
    let mut out = csv::WriterBuilder::new()
        .flexible(true)
        .delimiter(b',')
        .from_writer(std::io::BufWriter::new(File::open("uni/courses.csv").unwrap()));
    for page in links.into_iter() {
        let page_ind: NodeIndex = graph.add_node(url::Url::parse(page.clone().as_str())
            .unwrap().domain()
            .unwrap().to_string());
        let data = match reqwest::get(&page).await { Ok(data) => data.text().await?, Err(_) => continue };
        let links: Vec<String> = scrape_links(&page, &vec!["course", "class"]).await?.unwrap();
        println!("{}", links.len());
        //links.into_iter().for_each(|link| {
            //println!("{:?}", link);
        //});
        let mut lnk = links;
        lnk.insert(0, page.to_string());
        out.write_record(lnk).unwrap();
    }
    Ok(())
}

pub async fn scrape_links(page: &str, matches: &Vec<&str>) -> reqwest::Result<Option<Vec<String>>> {
    let data = match reqwest::get(&page.to_string()).await { 
        Ok(data) => data.text().await?, 
        Err(_) => return Ok(None),
    };
    let doc = Document::from(data.as_str());
    let links = doc.find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|n| n.contains("/") || n.contains("www"))
        .filter(|n| matches.into_iter().any(|word| n.contains(word)))
        .map(|link| {
            if !link.contains("http") {
                let link = format!("{}{}", &page, &link.strip_prefix("/").unwrap_or(link));
                link 
            } else { link.to_string() }
        })
        .collect::<Vec<String>>();
    Ok(Some(links))
}

//pub async fn visit_courses(links: Vec<String>) -> reqwest::Result<()> {
    //for link in links.into_par_iter() {
        //let data = match reqwest::get(link).await {
            //Ok(data) => data.text().await?,
            //Err(_) => continue,
        //};
        //let doc = Document::from(data.as_str());
        //let links = 
    //}
//}

pub async fn create_network() -> () {
    //let data = reqwest::ClientBuilder::new().connect_timeout(10)
        //.build();
}
