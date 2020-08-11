pub mod pipelines;
pub mod parse;
pub mod gen;

pub use pipelines::essay::*;
pub use pipelines::uni::*;
pub use gen::*;

#[tokio::main]
pub async fn main() -> tokio::io::Result<()> {
    Ok(())
}

pub async fn uni() {
    let links = pipelines::uni::get_uni_pages("United States").await; 
    let link_links = pipelines::uni::visit_unis(links).await.unwrap();
}

pub async fn keewa() {
    let keewa = read_txt("keewa").await;
    keewa.into_iter().enumerate().for_each(|(i, (mut txt, mut sentences))| {
        println!("Text {}", i);
        sentences.into_iter().enumerate().for_each(|(j, mut sentence)| {
            println!("{}: Sentence {}: {}", i, j, sentence);

        });
    });
}
