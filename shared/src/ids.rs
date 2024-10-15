use rand::{Rng, SeedableRng};

pub fn generate_id(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::rngs::StdRng;
    let rng = StdRng::from_entropy();
    rng.sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
