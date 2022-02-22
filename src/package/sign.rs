use super::Package;

impl Package {
    pub fn sign(&self) {
        println!("signing");
    }

    pub fn verify(&self) {
        println!("verifying");
    }
}
