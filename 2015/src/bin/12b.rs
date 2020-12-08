use serde_json::Value;

const INPUT: &str = include_str!("../input/12.txt");

fn sum(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(sum).sum(),
        Value::Object(o) => {
            if o.values()
                .find(|v| matches!(v.as_str(), Some("red")))
                .is_some()
            {
                0
            } else {
                o.values().map(sum).sum()
            }
        }
        _ => 0,
    }
}

fn main() {
    let v: Value = serde_json::from_str(INPUT).unwrap();
    let solution = sum(&v);
    println!("{}", solution);
}
