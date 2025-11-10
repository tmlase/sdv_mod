use sdv_mods::Version;

fn main() {
    // println!("{:?}", "1.2.3".split('.').collect::<Vec<_>>());
    // let mut parts = "1.2.3".split('.');
    // while let Some(p) = parts.next() {
    //     println!("{p}");
    // }

    let c = serde_json::from_str::<Version>(r#""3""#).unwrap();
    let v = serde_json::from_str::<Version>(r#""1.0""#).unwrap();
    println!("{:?}", v);
    println!("{}", serde_json::to_string(&v).unwrap());
    println!("compare is {}", c > v);
}
