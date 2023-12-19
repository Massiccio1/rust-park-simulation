use plotly::{Plot, Scatter, ImageFormat};
use chrono;
use plotly::common::{Mode, Title};
use rand::Rng;
// use structure::Structure;
use std::fs;
mod plt;
mod structure;
mod park;

fn main(){
    println!("-------------------------");
    println!("-------------------------");

}


#[test]
fn test1(){
    let s = structure::Structure{..Default::default()};

    println!("{}",s);

}
#[test]
fn test_plot(){
    plt::test_plot();
}