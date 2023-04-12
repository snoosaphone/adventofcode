use aoc_lib::*;

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

    for line in legs {
        println!("{:?}", line);
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

fn create_paths(input: Vec<Leg>) -> Vec<Route> {
    let mut routes = Vec::new();

    for leg in input.as_slice() {
        let mut route = Route{ path: vec![leg.city_a.clone(), leg.city_b.clone()], distance: leg.distance };

        for other_leg in input.as_slice() {
            if other_leg.city_a == route.path.last().unwrap().to_string() || other_leg.city_b == route.path.last().unwrap().to_string() && (!route.path.contains(other_leg.city_a) ){
                route.path.push(other_leg.city_b.clone());
                route.distance += other_leg.distance;
            }
        }

        routes.push(route);
    }

    routes
}

#[test]
fn create_legs_vec() {
    let input = vec![
        "London to Dublin = 464".to_string(),
        "London to Belfast = 518".to_string(),
        "Dublin to Belfast = 141".to_string()
    ];

    let result = create_legs(input);

    let expected = vec![
        Leg{
            city_a: "London".to_string(),
            city_b: "Dublin".to_string(),
            distance: 464
        }, 
        Leg{
            city_a: "London".to_string(),
            city_b: "Belfast".to_string(),
            distance: 518
        }, 
        Leg{
            city_a: "Dublin".to_string(),
            city_b: "Belfast".to_string(),
            distance: 141
        }
    ];

    assert_eq!(result, expected)
}

#[test]
fn create_paths_vec() {
    let input = vec![
        Leg{
            city_a: "London".to_string(),
            city_b: "Dublin".to_string(),
            distance: 464
        }, 
        Leg{
            city_a: "London".to_string(),
            city_b: "Belfast".to_string(),
            distance: 518
        }, 
        Leg{
            city_a: "Dublin".to_string(),
            city_b: "Belfast".to_string(),
            distance: 141
        }
    ];

    let result = create_paths(input);

    let expected = vec![
        Route{
            path: vec!["Dublin".to_string(), "London".to_string(), "Belfast".to_string()],
            distance: 982
        }
    ];

    assert_eq!(result, expected);
}
