use delfi::Dataset;

#[test]
fn test() {
    let vec = vec![1, 2, 3];
    let _ = Dataset::from(vec);
}

#[test]
fn columns() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let data = Dataset::columns([x,y],["x","y"]);
    data.save("./data/test/columns.csv").unwrap();
}

#[test]
fn macros() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let data = dataset!{
        "x" => x,
        "y" => y,
    };
    data.save("./data/test/macros.csv").unwrap();
}