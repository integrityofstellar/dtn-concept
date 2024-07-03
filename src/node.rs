use crate::bundle::Bundle;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: String,
    pub storage: Vec<Bundle>,
}
