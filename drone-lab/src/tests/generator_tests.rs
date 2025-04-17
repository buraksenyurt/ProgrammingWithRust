#[cfg(test)]
mod test {
    use crate::data::{DRONE_MODELS, get_random_number};

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
}
