use crate::pipelines;
use regex::Regex;
use std::io::BufReader;
use std::collections::HashMap;
use rust_bert::pipelines::{
    summarization::SummarizationModel,
    generation::{GPT2Generator, LanguageGenerator, MarianGenerator, GenerateConfig},
    sentiment::{Sentiment, SentimentModel, SentimentPolarity},
    sequence_classification::*, question_answering::*, conversation::*, ner::*,
};
use tch::Tensor;
use tch::vision::*;
use reqwest;
use select::document::Document;
use std::fs::{ File,
    read_to_string, read, write, read_dir, create_dir_all
};
use std::time::{Instant, Duration};
use markov::{Chain, InfiniteChainIterator, InfiniteChainStringIterator};

#[tokio::main]
pub async fn main() -> std::io::Result<()> {

}

pub async fn read_txt(user: &str) -> Vec<(String, Vec<String>)> {
    let re= Regex::new(r"(\(.*?\))|(\[.*?\])").unwrap();
    
    let mut res: Vec<(String, Vec<String>)> = Vec::new();

    read_dir(format!("text/{}/", user)).expect("User not found")
        .enumerate()
        .for_each(|(i, entry)| {
            let txt = &read_to_string(entry.unwrap().path()).unwrap();
            let txt = re.replace_all(txt, "").to_string();
            let sentences = txt.split_terminator(". ")
                .map(|words| words.to_string())
                .into_iter()
                .collect::<Vec<String>>();
            res.push((txt, sentences));
    });
    res
}

pub async fn get_txt() -> () {
    let chain = markov::Chain::<String>::new();

}

pub async fn gen_text(inp: String) -> Vec<Entity> {
    let gen = GPT2Generator::new(Default::default()).unwrap();
    let ner_model = NERModel::new(Default::default()).unwrap();
    let out = ner_model.predict(&[&inp]);
    let output = gen.generate(Some(vec![inp.as_str()]), None);
    out
}

pub async fn read_user() -> reqwest::Result<()> {
    let kd = reqwest::get("https://twitter.com/kiradorst").await?.text().await?;
    println!("{}", kd);
    Ok(())
}

pub async fn tokenize() -> () {


}
