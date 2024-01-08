use super::*;

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
fn create_routes_vec() {
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

    let result = create_routes(input);

    let expected = vec![
        Route{
            path: vec!["Dublin".to_string(), "London".to_string(), "Belfast".to_string()],
            distance: 982
        },
        Route{
            path: vec!["Dublin".to_string(), "Belfast".to_string(), "London".to_string()],
            distance: 659
        },
        Route{
            path: vec!["London".to_string(), "Dublin".to_string(), "Belfast".to_string()],
            distance: 605
        },
        Route{
            path: vec!["London".to_string(), "Belfast".to_string(), "Dublin".to_string()],
            distance: 659
        },
        Route{
            path: vec!["Belfast".to_string(), "London".to_string(), "Dublin".to_string()],
            distance: 982
        },
        Route{
            path: vec!["Belfast".to_string(), "Dublin".to_string(), "London".to_string()],
            distance: 605
        }
    ];

    assert_eq!(result, expected);
}
