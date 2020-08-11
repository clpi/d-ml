use std::collections::HashMap;
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
    let out = csv::Writer::from_path("uni/out.csv").unwrap();
    //while let Some(uni) = unis.pop() {}
    unis.into_iter().enumerate().for_each(|(i, uni)|  {
        if uni.country.eq(country) { 
            links.push(uni.web_pages.unwrap().get(0).unwrap().to_owned());
        }
    });
    links
}

pub async fn visit_unis(links: Vec<String>) -> reqwest::Result<()> {
    let mut link_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut webs: Vec<DiGraph<String, i32>> = Vec::new();
    let mut graph: DiGraph<String, i32> = DiGraph::new();
    let client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;
    for page in links.into_iter() {
        let page_ind: NodeIndex = graph.add_node(page.clone());
        let data = match reqwest::get(&page).await {
            Ok(data) => data.text().await?, 
            Err(_) => continue,
        };
        let doc = Document::from(data.as_str());
        let links = doc.find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .filter(|n| n.contains("/") || n.contains("www"))
            .map(|n| {
                if !n.contains("http") {
                    let link = format!("{}{}", &page, &n.strip_prefix("/").unwrap_or(n));
                    let link_ind = graph.add_node(link.clone());
                    let edge_ind = graph.add_edge(page_ind, link_ind, 1);
                    link
                } else { n.to_string() }
            })
            .collect::<Vec<String>>();
        println!("{}", links.len());
        //links.into_iter().for_each(|link| {
            //println!("{:?}", link);
        //});
    }
    Ok(())
}
