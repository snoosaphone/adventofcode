use aoc_lib::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct City {
    name: String,
    legs: Vec<Leg>,
}

#[derive(Debug, PartialEq, Eq)]
struct Leg {
    city_a: String,
    city_b: String,
    distance: usize
}

#[derive(Debug, PartialEq, Eq)]
struct Route {
    path: Vec<String>,
    distance: usize
}

fn main() {
    let input = get_file_contents("./input.txt");

    let legs = create_legs(input);

    for line in &legs {
        println!("{:?}", line);
    }

    let unique_starting_points = create_unique_start_list(&legs);

    let routes = create_routes(legs);

    for route in routes {
        println!("{:?}", route);
    }
}

fn create_legs(input: Vec<String>) -> Vec<Leg> {
    let mut legs = Vec::new();

    for line in input {
        let split = line.split(' ').collect::<Vec<_>>();

        let new_leg = Leg {
            city_a: split[0].to_string(),
            city_b: split[2].to_string(),
            distance: split[4].to_string().parse::<usize>().unwrap()
        };

        legs.push(new_leg);
    }

    legs
}

fn create_routes(input: Vec<Leg>) -> Vec<Route> {
    let mut routes = Vec::new();

    // For each unique starting position
    for leg in input.as_slice() {
        let mut route = Route{ path: vec![leg.city_a.clone(), leg.city_b.clone()], distance: leg.distance };

        for other_leg in input.as_slice() {
            if (other_leg.city_a == route.path.last().unwrap().to_string() || other_leg.city_b == route.path.last().unwrap().to_string()) && !route.path.contains(&other_leg.city_a){
                route.path.push(other_leg.city_b.clone());
                route.distance += other_leg.distance;
            }
        }

        routes.push(route);
    }

    routes
}

fn create_unique_start_list(input: &Vec<Leg>) -> Vec<String> {
    
}

#[cfg(test)]
mod test;
