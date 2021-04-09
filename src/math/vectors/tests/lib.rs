

#[cfg(test)]
mod tests {
    use vectors::Vector3;

    pub const STATIC_TEST: [Vector3; 3] = [
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    ];

    #[test]
    pub fn static_test() {
        assert_eq!(STATIC_TEST[0], Vector3::new(1.0, 0.0, 0.0),);
        assert_eq!(STATIC_TEST[1], Vector3::new(0.0, 1.0, 0.0),);
        assert_eq!(STATIC_TEST[2], Vector3::new(0.0, 0.0, 1.0),);
    }

    #[test]
    pub fn default() {
        let v = Vector3::default();
        assert_eq!(v, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    pub fn normalize() {
        let u = Vector3::new(1.0, 0.0, 0.0);
        let v = Vector3::new(5.0, 0.0, 0.0);

        assert_ne!(u, v);
        assert_eq!(u, v.normalize());
    }

    #[test]
    pub fn norm() {
        let v = Vector3::new(5.0, 0.0, 0.0);
        assert_eq!(v.norm(), 5.0);
    }

    #[test]
    pub fn add_number() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v + 1.0, Vector3::new(2.0, 3.0, 4.0));
    }

    #[test]
    pub fn sub_number() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v - 1.0, Vector3::new(0.0, 1.0, 2.0));
    }

    #[test]
    pub fn mul_number() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    pub fn add_vector() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(u + v, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    pub fn sub_vector() {
        let u = Vector3::new(1.0, 2.0, 3.0);
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(u - v, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    pub fn dot() {
        let u = Vector3::new(1.0, 0.0, 0.0);
        let v = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(u.dot(&v), 0.0);
    }

    #[test]
    pub fn cross() {
        let u = Vector3::new(1.0, 0.0, 0.0);
        let v = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(u.cross(&v), Vector3::new(0.0, 0.0, 1.0));
    }

}