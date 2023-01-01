use crate::Datapoint;

impl<const N: usize, Data: ToString> Datapoint<N> for [Data; N] {
    fn record(&self) -> [String; N] {
        self.iter()
            .map(Data::to_string)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

impl<D1: ToString> Datapoint<1> for (D1,) {
    fn record(&self) -> [String; 1] {
        [self.0.to_string()]
    }
}

impl<D1: ToString, D2: ToString> Datapoint<2> for (D1, D2) {
    fn record(&self) -> [String; 2] {
        [self.0.to_string(), self.1.to_string()]
    }
}

impl<D1: ToString, D2: ToString, D3: ToString> Datapoint<3> for (D1, D2, D3) {
    fn record(&self) -> [String; 3] {
        [self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let data = 3.14;
        let datapoint: &dyn Datapoint<1> = &[data];
        let record = datapoint.record();
        let compare = [String::from("3.14")];
        assert_eq!(record, compare);
    }

    #[test]
    fn multiple_elements() {
        let data = [0, 1, 2];
        let datapoint: &dyn Datapoint<3> = &data;
        let record = datapoint.record();
        let compare = [String::from("0"), String::from("1"), String::from("2")];
        assert_eq!(record, compare);
    }

    fn recording(list: Vec<&dyn Datapoint<3>>) {
        let mut iterator = list.iter();

        let element_1 = iterator.next().unwrap().record();
        let comparison_1 = [String::from("1"), String::from("2"), String::from("3")];
        assert_eq!(element_1, comparison_1);

        let element_2 = iterator.next().unwrap().record();
        let comparison_2 = [String::from("-1"), String::from("-2"), String::from("-3")];
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
        let datapoint: &dyn Datapoint<2> = &data;
        let record = datapoint.record();
        let compare = ["hello".to_string(), (4).to_string()];
        assert_eq!(record, compare);
    }
}
