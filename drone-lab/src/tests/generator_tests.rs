#[cfg(test)]
mod test {
    use crate::data::{DRONE_MODELS, get_random_between, get_random_number};

    #[test]
    fn random_drone_model_test() {
        let max_value = DRONE_MODELS.len();
        let random_number = get_random_number(max_value);
        let model = DRONE_MODELS[random_number];
        assert!(
            DRONE_MODELS
                .iter()
                .any(|m| m.to_string() == model.to_string())
        );
    }

    #[test]
    fn random_number_between_range_test() {
        let min_value = 10;
        let max_value = 50;
        let actual = get_random_between(min_value, max_value);
        assert!(actual >= min_value && actual <= max_value);
    }
}
