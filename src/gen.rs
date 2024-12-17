#[cfg(test)]
pub mod gen {
    use rand::{distributions::Alphanumeric, Rng};
    
    pub fn string(rng: &mut rand::rngs::ThreadRng) -> String {
        rng.sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect()
    }
}
