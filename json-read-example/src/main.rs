#[macro_use]
extern crate json;

fn main() {
    // making a json with json::parse
    let mut doc = json::parse(r#"{
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed");
    println!("json debug:   {:?}", doc);
    println!("json display: {}", doc.dump());
    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);
    assert_eq!(200, code);
    assert_eq!(true, success);

    // editing the json
    { // have to nest here for immutable borrow
        let features = &mut doc["payload"]["features"];
        println!("features: {:?}", features);
        for v in features.members() {
            println!("member: {}", v.as_str().unwrap()); // May explode
        }
        features.push("cargo!").expect("cannot push");
    }
    println!("json display: {}", doc);
    
    // using macros to create json
    let data = object!(
        "name"    => "Otter",
        "age"     => 4,
        "numbers" => array![1,2,3]
    );
    assert_eq!(
        data.dump(),
        r#"{"name":"Otter","age":4,"numbers":[1,2,3]}"#
    );
    println!("data:         {}", data.dump());
}
