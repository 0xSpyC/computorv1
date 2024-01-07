mod parsing_polynome_valid {

    use polynomial::Polynomial;

    #[test]
    fn empty_string() {
        assert_eq!(Polynomial::from_str("").unwrap().terms, []);
    }

    #[test]
    fn single_integer() {
        assert_eq!(Polynomial::from_str("1").unwrap().terms, [(1.0, 0)]);
    }

    #[test]
    fn single_positive_integer() {
        assert_eq!(Polynomial::from_str("+1").unwrap().terms, [(1.0, 0)]);
    }

    #[test]
    fn single_negative_integer() {
        assert_eq!(Polynomial::from_str("-1").unwrap().terms, [(-1.0, 0)]);
    }

    #[test]
    fn single_float() {
        assert_eq!(Polynomial::from_str("1.2").unwrap().terms, [(1.2, 0)]);
    }

    #[test]
    fn single_positive_float() {
        assert_eq!(Polynomial::from_str("+1.2").unwrap().terms, [(1.2, 0)]);
    }

    #[test]
    fn single_negative_float() {
        assert_eq!(Polynomial::from_str("-1.2").unwrap().terms, [(-1.2, 0)]);
    }

    #[test]
    fn single_variable() {
        assert_eq!(Polynomial::from_str("x").unwrap().terms, [(1.0, 1)]);
    }

    #[test]
    fn single_positive_variable() {
        assert_eq!(Polynomial::from_str("+x").unwrap().terms, [(1.0, 1)]);
    }

    #[test]
    fn single_negative_variable() {
        assert_eq!(Polynomial::from_str("-x").unwrap().terms, [(-1.0, 1)]);
    }

    #[test]
    fn simple_polynome() {
        assert_eq!(Polynomial::from_str("1.2x").unwrap().terms, [(1.2, 1)]);
    }

    #[test]
    fn simple_positive_polynome() {
        assert_eq!(Polynomial::from_str("+1.2x").unwrap().terms, [(1.2, 1)]);
    }

    #[test]
    fn simple_negative_polynome() {
        assert_eq!(Polynomial::from_str("-1.2x").unwrap().terms, [(-1.2, 1)]);
    }

    #[test]
    fn simple_polynome_complex_float() {
        assert_eq!(Polynomial::from_str(".2x").unwrap().terms, [(0.2, 1)]);
    }

    #[test]
    fn simple_positive_polynome_complex_float() {
        assert_eq!(Polynomial::from_str("+.2x").unwrap().terms, [(0.2, 1)]);
    }

    #[test]
    fn simple_negative_polynome_complex_float() {
        assert_eq!(Polynomial::from_str("-.2x").unwrap().terms, [(-0.2, 1)]);
    }

    #[test]
    fn polynome_two_no_coefficient() {
        assert_eq!(Polynomial::from_str("x^2").unwrap().terms, [(1.0, 2)]);
    }

    #[test]
    fn positive_polynome_two_no_coefficient() {
        assert_eq!(Polynomial::from_str("+x^2").unwrap().terms, [(1.0, 2)]);
    }

    #[test]
    fn negative_polynome_two_no_coefficient() {
        assert_eq!(Polynomial::from_str("-x^2").unwrap().terms, [(-1.0, 2)]);
    }

    #[test]
    fn polynome_two() {
        assert_eq!(
            Polynomial::from_str("1x^2 + x").unwrap().terms,
            [(1.0, 2), (1.0, 1)]
        );
    }

    #[test]
    fn positive_polynome_two() {
        assert_eq!(
            Polynomial::from_str("+1x^2 + x").unwrap().terms,
            [(1.0, 2), (1.0, 1)]
        );
    }

    #[test]
    fn negative_polynome_two() {
        assert_eq!(
            Polynomial::from_str("-1x^2 - x").unwrap().terms,
            [(-1.0, 2), (-1.0, 1)]
        );
    }

    #[test]
    fn valid_polynome() {
        assert_eq!(
            Polynomial::from_str("5 + 4 * X + X^2 = X^2 ")
                .unwrap()
                .terms,
            [(5.0, 0), (4.0, 1), (1.0, 2), (-1.0, 2)]
        );
    }
}

mod parsing_polynome_invalid {

    use polynomial::ParsingError;
    use polynomial::Polynomial;

    #[test]
    fn multiple_virgule() {
        assert_eq!(
            Polynomial::from_str("1..2").unwrap_err(),
            ParsingError::Coefficient
        );
    }

    #[test]
    fn float_exponent() {
        assert_eq!(
            Polynomial::from_str("2x^3.2").unwrap_err(),
            ParsingError::Exponent
        );
    }

    #[test]
    fn missing_exponent() {
        assert_eq!(
            Polynomial::from_str("-2x2").unwrap_err(),
            ParsingError::MissingExponent
        );
    }

    #[test]
    fn empty_exponent() {
        assert_eq!(
            Polynomial::from_str("-2x^").unwrap_err(),
            ParsingError::EmptyExponent
        );
    }

    #[test]
    fn missing_number() {
        assert_eq!(
            Polynomial::from_str("+").unwrap_err(),
            ParsingError::Polynome
        );
    }

    #[test]
    fn exponent_without_variable() {
        assert_eq!(
            Polynomial::from_str("^2").unwrap_err(),
            ParsingError::ExponentWithoutVariable
        );
    }

    #[test]
    fn too_many_variable() {
        assert_eq!(
            Polynomial::from_str("x^2x").unwrap_err(),
            ParsingError::TooManyVariables
        );
    }

    #[test]
    fn too_many_exponent() {
        assert_eq!(
            Polynomial::from_str("^^2x").unwrap_err(),
            ParsingError::TooManyExponents
        );
    }

    #[test]
    fn exponent_before_variable() {
        assert_eq!(
            Polynomial::from_str("^2x").unwrap_err(),
            ParsingError::ExponentPosition
        );
    }

    #[test]
    fn number_between_variable_exponent() {
        assert_eq!(
            Polynomial::from_str("2x2^2").unwrap_err(),
            ParsingError::ExponentPosition
        );
    }

    #[test]
    fn invalid_character() {
        assert_eq!(
            Polynomial::from_str("j2x2^2").unwrap_err(),
            ParsingError::Character
        );
    }

    #[test]
    fn multiple_equal() {
        assert_eq!(
            Polynomial::from_str("2x^2 = = 3x").unwrap_err(),
            ParsingError::TooManyEquals
        );
    }

    #[test]
    fn multiple_multiplication_sign() {
        assert_eq!(
            Polynomial::from_str("2 * * x^2 = = 3x").unwrap_err(),
            ParsingError::TooManyMultiplicationSigns
        );
    }
}
