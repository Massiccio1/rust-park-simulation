use plotly::{Plot, Scatter, ImageFormat, Bar, Layout, layout::{Axis, LayoutGrid, GridPattern, RowOrder}};
use chrono::{self};
use plotly::common::{Mode, Title};
use rand::Rng;
use std::fs;
use serde::{self, Serialize, Deserialize};

use crate::{structure::Structure, park};

pub fn main(){
    println!("-------------------------");
    test_plot();
}
pub fn my_plot<T: std::clone::Clone + serde::ser::Serialize + 'static >(points:(Vec<T>, Vec<T>),show:bool,save:bool) {
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
pub fn test_bar() {
    // Define the data for the bar chart
    let x_data = vec!["Apples", "Oranges", "Bananas"];
    let y_data = vec![10, 20, 15];

    // Create a Bar trace
    let trace = Bar::new(x_data, y_data)
        .name("Fruit sales");

    // Create the plot layout
    let layout = Layout::new()
        .title(Title::new("Fruit Sales"))
        .x_axis(Axis::new().title(Title::new("Fruit")))
        .y_axis(Axis::new().title(Title::new("Sales")));

    // Create the plot and add the trace and layout
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    // Show the plot in the default browser
    plot.show()
}
pub fn plot_bar(x_data:Vec<i32>,y_data:Vec<i32>) {
    // Define the data for the bar chart

    // Create a Bar trace
    let trace = Bar::new(x_data, y_data);

    // Create the plot layout
    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::new("Timestamp")))
        .y_axis(Axis::new().title(Title::new("Line")));

    // Create the plot and add the trace and layout
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    // Show the plot in the default browser
    plot.show()
}

pub fn plot_n_bar(all_data:&Vec<Structure>) {
    // Define the data for the bar chart

    // Create a Bar trace

    let mut trace_vect:Vec<Box<Bar<u32,u32>>> = vec![];
    for st in all_data{
        let trace = Bar::new((0..st.line_log.len() as u32).collect(), st.line_log.clone()).name(st.name.clone());
        trace_vect.push(trace);
    }



    // Create the plot and add the trace and layout
    let mut plot = Plot::new();
    for b in trace_vect{
        plot.add_trace(b);
    }
    // plot.add_traces(trace_vect);


    let layout = Layout::new();
    // plot.set_layout(layout);
    plot.set_layout(layout);

    // Show the plot in the default browser
    plot.show()
}

pub fn simple_subplot(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![1, 2, 4], vec![4, 5, 6])
        .name("trace2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
        
    plot.set_layout(layout);
        if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("simple_subplot"))
    );
}

