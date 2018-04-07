#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate regex;
use regex::Regex;
extern crate chrono;
use chrono::*;


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    address: Address,
    phone: String
}
#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    // basic use of serde-json
    let data = r#"{
        "name":"Otter",
        "age": 4,
        "address": {
            "street": "Here Avenue",
            "city":"NowhereLand"
        },
        "phone":"555-5555"
    }"#;
    let person: Person = serde_json::from_str(data).expect("serialization error");
    println!("{} lives at '{}, {}'", person.name, person.address.street, person.address.city) ;
    println!("person: {:?}", person);

    // basic matching fusing regex
    let regex = Regex::new(r"(\d{2}):(\d+)").unwrap();
    println!("'   10:230' matches {:?}", regex.captures("   10:230"));
    println!("'[20:22]' matches {:?}", regex.captures("[20:22]"));
    println!("10:x23 matches {:?}", regex.captures("10:x23"));

    // matching dates with regex
    let date_regex = Regex::new(r"(?x)
    (?P<year>\d{4})
    -
    (?P<month>\d{2})
    -
    (?P<day>\d{2})
    ").expect("bad regex");

    let mut caps = date_regex.captures("2010-03-04").expect("match failed");
    let mut int_year: i32 = caps["year"].parse().unwrap();
    let mut uint_month: u32 = caps["month"].parse().unwrap();
    let mut uint_day: u32 = caps["day"].parse().unwrap();
    println!("year {:?}", int_year);
    println!("month {:?}", uint_month);
    println!("day {:?}", uint_day);

    // chrono crate use - this will only parse correct dates
    let mut date = Local.ymd(int_year,uint_month,uint_day);
    println!("date: {:?}", date);
    //match date {
    //    Some(d) => println!("The Date is a good Date {:?}", d),
    //    None => println!("The Date is a bad Date, so:  {:?}", date)
    //}
    caps = date_regex.captures("0010-33-54").expect("match failed");
    int_year = caps["year"].parse().unwrap();
    uint_month = caps["month"].parse().unwrap();
    uint_day = caps["day"].parse().unwrap();
    println!("year (not a real year) {:?}", int_year);
    println!("month (not a real month) {:?}", uint_month);
    println!("day (not a real day) {:?}", uint_day);

    // chrono crate use - this will not parse this dates
    // date = Local.ymd(int_year,uint_month,uint_day); // this will throw an error
    // println!("date: {:?}", date);
    //match date {
    //    Some(d) => println!("The Date is a good Date {:?}", date),
    //    None => println!("The Date is a bad Date, so:  {:?}", date)
    //}
}
