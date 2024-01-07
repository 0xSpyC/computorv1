mod polynomial_invalid {

    use polynomial::PolynomeError;
    use polynomial::Polynomial;

    #[test]
    fn polynome_null() {
        let mut polynome = Polynomial::from_str("2x^0").unwrap();
        polynome.reduce();
        assert_eq!(polynome.solve().unwrap_err(), PolynomeError::PolynomeNull);
    }

    #[test]
    fn polynome_three() {
        let mut polynome = Polynomial::from_str("2 + 2 * X^1 + 3 * X^2 + 4 * X^3").unwrap();
        polynome.reduce();
        assert_eq!(polynome.solve().unwrap_err(), PolynomeError::PolynomeDegree);
    }

    #[test]
    fn linear_zero_solution() {
        let mut polynome = Polynomial::from_str(" 0X + 4").unwrap();
        polynome.reduce();
        assert_eq!(polynome.solve().unwrap_err(), PolynomeError::PolynomeNull);
    }
}
