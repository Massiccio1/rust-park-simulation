use plotly::{Plot, Scatter, ImageFormat};
use chrono;
use plotly::common::{Mode, Title};
use rand::Rng;
use std::fs;

pub fn main(){
    println!("-------------------------");
    test_plot();
}
pub fn my_plot(points:(Vec<f32>, Vec<f32>),show:bool,save:bool) {
    // Create data for the line chart

    // Create a scatter plot with the data
    let scatter = Scatter::new(points.0, points.1).name("Scatter").mode(Mode::Markers);

    // Create a plot and add the scatter plot
    let mut plot = Plot::new();
    plot.add_trace(scatter);

    // Customize the plot layout
    plot.set_layout(plotly::Layout::new().title(Title::from("My Line Chart")));

    // Show the plot in a browser window
    //plot.show();
    //let date = chrono::offset::Local::now().to_string();
    let date = chrono::offset::Local::now().to_string().replace(":", ".");
    
    if show{
        plot.show();
    }
    
    if save{
        
        fs::create_dir_all("../images").unwrap();

        plot.write_image("../images/latest.png", ImageFormat::PNG, 800, 600, 1.0);
        plot.write_image(format!("../images/{date}.png"), ImageFormat::PNG, 800, 600, 1.0);
    }
}


pub fn random_points(number:u32,range_x: (f32,f32),range_y: (f32,f32)) -> (Vec<f32>, Vec<f32>){

    let mut rng = rand::thread_rng();
    let mut x:Vec<f32>=vec![];
    let mut y:Vec<f32>=vec![];

    for _ in 0..number{
        x.push(rng.gen_range(range_x.0..range_x.1));
        y.push(rng.gen_range(range_y.0..range_y.1));
    }


    // println!("x: {:?}",x);
    // println!("y: {:?}",y);


    return (x,y);
}

pub fn test_plot(){
    let points = random_points(100, (-3.0, 1.0),(-1.0,2.0));
    my_plot(points, true, false);
}