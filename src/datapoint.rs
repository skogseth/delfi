/*!
Implementations on the [Datapoint] trait
*/

use crate::Datapoint;

impl<const N: usize, Data: ToString> Datapoint for [Data; N] {
    fn record(&self, delimiter: char) -> String {
        let mut string = String::new();
        for i in 0..(N-1) {
            string += &self[i].to_string();
            string.push(delimiter);
        }
        string += &self[N-1].to_string();
        string
    }
}

impl<D1: ToString> Datapoint for (D1,) {
    fn record(&self, _delimiter: char) -> String {
        self.0.to_string()
    }
}

impl<D1: ToString, D2: ToString> Datapoint for (D1, D2) {
    fn record(&self, delimiter: char) -> String {
        format!("{first}{d}{second}", first = self.0.to_string(), second = self.1.to_string(), d = delimiter)
    }
}

impl<D1: ToString, D2: ToString, D3: ToString> Datapoint for (D1, D2, D3) {
    fn record(&self, delimiter: char) -> String {
        format!("{first}{d}{second}{d}{third}", first = self.0.to_string(), second = self.1.to_string(), third = self.2.to_string(), d = delimiter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let data = 3.14;
        let datapoint: &dyn Datapoint = &[data];
        let record = datapoint.record(',');
        let compare = String::from("3.14");
        assert_eq!(record, compare);
    }

    #[test]
    fn multiple_elements() {
        let data = [0, 1, 2];
        let datapoint: &dyn Datapoint = &data;
        let record = datapoint.record(',');
        let compare = String::from("0,1,2");
        assert_eq!(record, compare);
    }

    fn recording(list: Vec<&dyn Datapoint>) {
        let mut iterator = list.iter();

        let element_1 = iterator.next().unwrap().record(',');
        let comparison_1 = String::from("1,2,3");
        assert_eq!(element_1, comparison_1);

        let element_2 = iterator.next().unwrap().record(';');
        let comparison_2 = String::from("-1;-2;-3");
        assert_eq!(element_2, comparison_2);

        assert!(iterator.next().is_none())
    }

    #[test]
    fn list() {
        let datapoint_1 = [1, 2, 3];
        let datapoint_2 = [-1, -2, -3];
        recording(vec![&datapoint_1, &datapoint_2]);
    }

    #[test]
    fn tuples() {
        let data = ("hello", 4);
        let datapoint: &dyn Datapoint = &data;
        let record = datapoint.record(',');
        let compare = "hello,4".to_string();
        assert_eq!(record, compare);
    }
}
