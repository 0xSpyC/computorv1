mod tokenize_polynome {
    use polynomial::tokenize;
    use polynomial::Token;

    #[test]
    fn empty_string() {
        assert_eq!(tokenize("").collect::<Vec<_>>(), &[]);
    }

    #[test]
    fn no_right_member() {
        assert_eq!(
            tokenize("2x^1 + 3x^2").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1 ".to_owned())),
                Ok(Token::Term("+ 3x^2".to_owned())),
            ]
        );
    }

    #[test]
    fn empty_left_member() {
        assert_eq!(
            tokenize(" = 2x^1 + 3x^2").collect::<Vec<_>>(),
            &[
                Ok(Token::Term(" ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" 2x^1 ".to_owned())),
                Ok(Token::Term("+ 3x^2".to_owned())),
            ]
        );
    }

    #[test]
    fn empty_right_member() {
        assert_eq!(
            tokenize("2x^1 + 3x^2 = ").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1 ".to_owned())),
                Ok(Token::Term("+ 3x^2 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" ".to_owned())),
            ]
        );
    }

    #[test]
    fn negative_sign() {
        assert_eq!(
            tokenize("2x^1 - 3x^2 = 3x^4 - 1x^2").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1 ".to_owned())),
                Ok(Token::Term("- 3x^2 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" 3x^4 ".to_owned())),
                Ok(Token::Term("- 1x^2".to_owned())),
            ]
        );
    }

    #[test]
    fn multiple_equal() {
        assert_eq!(
            tokenize("2x^1==").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Equal),
            ]
        );
    }

    #[test]
    fn multiple_sign() {
        assert_eq!(
            tokenize("+-2x^1").collect::<Vec<_>>(),
            &[Err(()), Ok(Token::Term("-2x^1".to_owned())),]
        );
    }

    #[test]
    fn sign_between_coefficient() {
        assert_eq!(
            tokenize("2-x^1").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2".to_owned())),
                Ok(Token::Term("-x^1".to_owned())),
            ]
        );
    }

    #[test]
    fn sign_between_exponent() {
        assert_eq!(
            tokenize("2x-^1").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x".to_owned())),
                Ok(Token::Term("-^1".to_owned())),
            ]
        );
    }

    #[test]
    fn multiple_sign_between_coefficient() {
        assert_eq!(
            tokenize("2+-x^1").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2".to_owned())),
                Err(()),
                Ok(Token::Term("-x^1".to_owned())),
            ]
        );
    }

    #[test]
    fn multiple_sign_between_exponent() {
        assert_eq!(
            tokenize("2x+-^1").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x".to_owned())),
                Err(()),
                Ok(Token::Term("-^1".to_owned())),
            ]
        );
    }

    #[test]
    fn sign_started() {
        assert_eq!(
            tokenize("-2x^1").collect::<Vec<_>>(),
            &[Ok(Token::Term("-2x^1".to_owned()))]
        );
    }

    #[test]
    fn sign_terminated() {
        assert_eq!(
            tokenize("2x^1-").collect::<Vec<_>>(),
            &[Ok(Token::Term("2x^1".to_owned())), Err(()),]
        );
    }

    #[test]
    fn null_polynome() {
        assert_eq!(
            tokenize("0x^0 + 0x^1 = 0").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("0x^0 ".to_owned())),
                Ok(Token::Term("+ 0x^1 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" 0".to_owned())),
            ]
        );
    }

    #[test]
    fn single_polynome() {
        assert_eq!(
            tokenize("2x^2").collect::<Vec<_>>(),
            &[Ok(Token::Term("2x^2".to_owned())),]
        );
    }

    #[test]
    fn polynome_negative() {
        assert_eq!(
            tokenize("2x^-2").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^".to_owned())),
                Ok(Token::Term("-2".to_owned()))
            ]
        );
    }

    #[test]
    fn polynome_two() {
        assert_eq!(
            tokenize("2x^1 + 3x^2 + 12 = 0").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1 ".to_owned())),
                Ok(Token::Term("+ 3x^2 ".to_owned())),
                Ok(Token::Term("+ 12 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" 0".to_owned())),
            ]
        );
    }

    #[test]
    fn unreduced_polynome() {
        assert_eq!(
            tokenize("2x^1 + 3x^2 = 3x^4 + 1x^2").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("2x^1 ".to_owned())),
                Ok(Token::Term("+ 3x^2 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" 3x^4 ".to_owned())),
                Ok(Token::Term("+ 1x^2".to_owned())),
            ]
        );
    }

    #[test]
    fn valid_polynome() {
        assert_eq!(
            tokenize("5 + 4 * X + X^2 = X^2 ").collect::<Vec<_>>(),
            &[
                Ok(Token::Term("5 ".to_owned())),
                Ok(Token::Term("+ 4 * X ".to_owned())),
                Ok(Token::Term("+ X^2 ".to_owned())),
                Ok(Token::Equal),
                Ok(Token::Term(" X^2 ".to_owned())),
            ]
        );
    }
}
