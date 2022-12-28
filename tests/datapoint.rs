use delfi::Datapoint;

#[derive(Debug, Datapoint)]
struct Count {
    ch: char,
    i: usize,
}

#[derive(Debug, Datapoint)]
struct Point(f64, f64);

#[test]
fn derive_named_struct() {
    let a = Count { ch: 'a', i: 49 };
    let record = a.record();
    let compare = [('a').to_string(), (49).to_string()];
    assert_eq!(record, compare);
}

#[test]
fn derive_unnamed_struct() {
    let p = Point(3.0, 4.0);
    let record = p.record();
    let compare = [(3.0).to_string(), (4.0).to_string()];
    assert_eq!(record, compare);
}

#[test]
fn trait_objects() {
    let a = Count { ch: 'a', i: 49 };
    let p = Point(3.0, 4.0);
    let list: [&dyn Datapoint<2>; 2] = [&a, &p];
    for dp in list {
        println!("{:?}", dp.record());
    }
}