use delfi::Datapoint;

#[derive(Debug, Datapoint)]
struct Point(f64, f64);

#[derive(Debug, Datapoint)]
struct Count {
    ch: char,
    i: usize,
}

fn main() {
    let c = Count { ch: 'a', i: 32 };
    println!("{c:#?}");
    let r = c.record();
    println!("{:?}", r);

    let p = Point(3.0, 2.0);
    println!("{p:#?}");
    let r = p.record();
    println!("{:?}", r);
}

