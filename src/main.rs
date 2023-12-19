use plotly::{Plot, Scatter, ImageFormat};
use chrono;
use plotly::common::{Mode, Title};
use rand::Rng;
// use structure::Structure;
use std::fs;

mod plt;
mod structure;
mod park;
mod person;

fn main(){
    println!("-------------------------");
    println!("-------------------------");
    let mut p = park::Park{..Default::default()};
    p.init();
    p.add_person(4);
    p.init();

}

#[test]
fn test_plot(){
    //plt::test_plot();
}


#[test]
fn test_structure(){
    let s = structure::Structure{..Default::default()};

    println!("{}",s);

}


#[test]
fn test_park(){
    let p = park::Park{..Default::default()};
}

#[test]
fn test_person(){
    let p = person::Person{..Default::default()};
}