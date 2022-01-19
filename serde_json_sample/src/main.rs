use std::f64::consts::PI;
use serde::{Serialize, Deserialize};
use serde_json::json;


fn main() {
    println!("serde sample!");

    run_serde();

    run_calc_shape();

    work_without_types();
}


#[derive(Debug, Deserialize,Serialize)]
struct Person{
    name: String,
    age: usize,
    verified: bool,
}


fn run_serde(){
    let person_str = r#"
        {
          "name": "George",
          "age": 27,
          "verified": false
        }
    "#;

    let person: Person =serde_json::from_str(person_str).unwrap();
    println!("person:{:?}",person);
    let person_ser = serde_json::to_string(&person);
    println!("pserson ser:{}",person_ser.unwrap());



}


#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all="lowercase")]
enum Calculation {
    Perimeter,
    Area,
}

/// tag ="shape"使得原来的结构从:
// {
//   "Circle": {
//     "radius": 4.5
//   }
// }
/// 变为：
// {
//   "shape": "circle",
//   "radius": 4.5
// }
/// 给对象打个标记
#[derive(Debug, Deserialize,Serialize)]
#[serde(rename_all="lowercase", tag = "shape")]
enum Shape {
    Circle {
        radius: f64,
    },
    Rectangle {
        length: f64,
        width: f64,
    }
}

#[derive(Debug, Deserialize,Serialize)]
struct Request {
    calculation: Calculation,
    #[serde(flatten)] // flatten 消除嵌套
    shape: Shape,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    result: f64,
}

fn run_calc_shape(){

    let json = r#"
        {
            "calculation": "perimeter",
            "shape":"circle",
            "radius":2.3
        }
    "#;

    let request:Request = serde_json::from_str(json).unwrap();
    println!("request:{:?}",request);
    // 输出： { calculation: Perimeter, shape: Circle { radius: 2.3 } } ，会发现对象嵌套不一样
    let r =calculation_handler(request);
    println!("respone:{}",r.result);
}


fn calculation_handler(request: Request) -> Response {
    let result = match (request.calculation, request.shape) {
        (Calculation::Perimeter, Shape::Circle{radius}) => PI*2.0* radius,
        (Calculation::Perimeter, Shape::Rectangle { width,length }) => 2.0* width+2.0*length,
        (Calculation::Area, Shape::Circle {radius}) => PI *radius *radius,
        (Calculation::Area,Shape::Rectangle {width,length}) => width*length,
    };

    Response{result}
}


fn work_without_types(){

    let json =r###"
        {
            "name": "Bob",
            "age": 51,
            "address": {
                "country": "Germany",
                "city": "Example City",
                "street": "Example Street"
            },
            "siblings": ["Alice", "Joe"]
        }
    "###;

    // value 的类型是 `serde_json::Value`
    let value =json!(
        {
            "name": "Bob",
            "age": 51,
            "address": {
                "country": "Germany",
                "city": "Example City",
                "street": "Example Street"
            },
            "siblings": ["Alice", "Joe"]
        });

    //let value =json!(json);

    let v = value.get("address").and_then(|name|name.get("country"));
    println!("country:{:?}",v);
    println!("string value:{}", value.to_string()); //变为String


    let full_name ="John Doe";
    let age_last_year = 42;

    let john = json!({
        "name": full_name,
        "age": age_last_year+1,
        "phones":[
            format!("+86 {}", "153998876")
        ]
    });

    println!("the first phone number:{}", john["phones"][0]);




}




















