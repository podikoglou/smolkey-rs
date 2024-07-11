#[derive(Default, Debug)]
pub struct ServiceState {
    pub store: papaya::HashMap<String, String>,
}
