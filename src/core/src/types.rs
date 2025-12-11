pub struct Relation {
    pub name: String,
    pub weight: f64,
}

pub type WoTGraph<N> = petgraph::graph::Graph<N, Relation, petgraph::Directed>;

impl Relation {
    pub fn new(name: impl Into<String>, weight: f64) -> Result<Self, String> {
        if !(-1.0..=1.0).contains(&weight) {
            return Err(format!(
                "Weight must be between -1.0 and 1.0, got {}",
                weight
            ));
        }
        Ok(Relation {
            name: name.into(),
            weight,
        })
    }
}
