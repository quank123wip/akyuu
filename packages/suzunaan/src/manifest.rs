pub struct Manifest {
    pub greet: String,
    pub version: String,
}

pub fn manifest() -> Manifest {
    Manifest {
        greet: String::from("This is the Akyuu web api endpoint."),
        version: String::from("0.1.0")
    }
}