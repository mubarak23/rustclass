use std::collections:HashMap;


fn main() {
    // ANOTATE WITH HashMap<K, V>
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("surface", 89);
    scores.insert("Area", 45);
    scores.insert("Ashley", 150);

    let score = scores.get("surface");
    assert_eq(score, Some(89));
}



    // assert_eq!(scores.len(), 3);


// FILL in the blanks and FIX the errors
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score: Option<&i32> = scores.get("Sunface");
     assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score: i32 = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

     assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}


use std::collections::HashMap;
fn main() {
    let teams: [(&str, i32); 3] = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1:HashMap<&str, i32> = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // IMPLEMENT team_map2 in two ways
    // Tips: one of the approaches is to use `collect` method
    // let teams_map2: HashMap<&str, i32> = HashMap::from(teams);

      // let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
    let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}



// FILL in the blanks
use std::collections::HashMap;
fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}


// HashMap Capacity
// Like vectors, HashMaps are growable, but HashMaps can also shrink themselves when they have excess space.
//  You can create a HashMap with a certain starting capacity using HashMap::with_capacity(uint), 
//  or use HashMap::new() to get a HashMap with a default initial capacity (recommended).