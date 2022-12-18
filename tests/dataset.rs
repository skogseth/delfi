use delfi::{dataset, Dataset};

#[test]
fn columns_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let _dataset = Dataset::columns([x,y],["x","y"]);
    //data.save("./resources/data/tests/columns-arrays.csv").unwrap();
}

#[test]
fn macro_using_arrays() {
    let x = [1, 2, 3];
    let y = [3, 4, 5];
    let _dataset = dataset!{
        "x" => x,
        "y" => y,
    };
    //data.save("./resources/data/tests/macro-arrays.csv").unwrap();
}

#[test]
fn columns_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let _dataset = Dataset::columns([t,x], ["time", "length"]);
    //data.save("./resources/data/tests/columns-vectors.csv").unwrap();
}

#[test]
fn macro_using_vectors() {
    let t = vec![0.0, 1.0, 2.0];
    let x = vec![0.0, 2.0, 6.0];
    let _dataset = dataset!{
        "time" => t,
        "length" => x,
    };
    //data.save("./resources/data/tests/macro-vectors.csv").unwrap();
}
