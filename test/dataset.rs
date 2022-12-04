use delfi::Dataset;

#[test]
fn columns_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let data = Dataset::columns([x,y],["x","y"]);
    data.save("./data/tests/columns-arrays.csv").unwrap();
}

#[test]
fn macro_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let data = dataset!{
        "x" => x,
        "y" => y,
    };
    data.save("./data/tests/macro-arrays.csv").unwrap();
}

#[test]
fn columns_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let dataset = Dataset::columns([t,x], ["time", "length"]);
    data.save("./data/tests/columns-vectors.csv").unwrap();
}

#[test]
fn macro_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let dataset = Dataset::columns([t,x], ["time", "length"]);
    data.save("./data/tests/macro-vectors.csv").unwrap();
}
