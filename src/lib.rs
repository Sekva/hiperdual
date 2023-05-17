pub struct HDNumber {
    real: f64,
    eps_1: f64,
    eps_2: f64,
    eps_12: f64,
}

impl HDNumber {
    pub fn new(real: f64, eps_1: f64, eps_2: f64, eps_12: f64) -> HDNumber {
        HDNumber {
            real,
            eps_1,
            eps_2,
            eps_12,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = HDNumber::new(1.0, 0.0, 0.0, 1.0);
        assert_eq!(x.real, 1.0);
        assert_eq!(x.eps_1, 0.0);
        assert_eq!(x.eps_2, 0.0);
        assert_eq!(x.eps_12, 1.0);
    }
}
