use std::{collections::VecDeque, error::Error};
use serde::Deserialize;
use csv::Reader;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct BorderCountryRow 
{
    country_code: String,
    country_name: String,
    country_border_code: String,
    country_border_name: String,    
}

fn read_data_from_csv(filepath:&str) -> Result<Vec<BorderCountryRow>, Box<dyn Error>> 
{

    let mut rdr = Reader::from_path(filepath)?;
    let mut data: Vec<BorderCountryRow> = Vec::new();

    for result in rdr.deserialize(){
        let record: BorderCountryRow = result?;
        data.push(record);
    }

    Ok(data)

}

// create a graph (hashmap)  
fn create_graph(data: &Vec<BorderCountryRow>) -> HashMap<String, Vec<String>> 
{
    let mut graph = HashMap::new();
    
    for row in data {
        graph.entry(row.country_code.clone()).or_insert(Vec::new()).push(row.country_border_code.clone());
        graph.entry(row.country_border_code.clone()).or_insert(Vec::new()).push(row.country_code.clone());
    }

    graph
}

// algorithm to search for best path between 2 countries
// breath first search
fn breadth_first_search(graph: &HashMap<String, Vec<String>>, start_country:&str, end_country: &str) -> Option<Vec<String>> 
{
    let mut visited = HashMap::new();
    
    //double ended queue
    let mut queue = VecDeque::new();

    visited.insert(start_country.to_string(), None);
    queue.push_back(start_country.to_string());

    while let Some(country) = queue.pop_front() 
    {
        if &country == end_country
        {
            let mut path = Vec::new();
            let mut current = Some(end_country);        

            while let Some(country) = current
            {
                path.push(country.to_string());
                
                //get the value from the HashMap, convert it to a reference, and flatten the result
                current = visited.get(country).and_then(|x| x.as_deref());
            }

            path.reverse();
            return Some(path);
        }

        if let Some(neighbours) = graph.get(&country)
        {
            for neighbour in neighbours
            {
                if !visited.contains_key(neighbour)
                {
                    visited.insert(neighbour.to_string(), Some(country.to_string()));
                    queue.push_back(neighbour.to_string());
                }
            }
        }
    }

    None

}

fn main() -> Result<(), Box<dyn Error>> 
{

    let country1:&str = "CA";
    let country2:&str = "AR";
    let filepath:&str = "src/borders.csv";

    let data = read_data_from_csv(filepath)?;
    let graph = create_graph(&data);
    
    let path = breadth_first_search(&graph, country1, country2);
    
    match path {
        Some(path) => println!("Path: {:?}", path),
        None => println!("No path found"),
    }
    
    Ok(())
}