
#[cfg(test)]
mod tests{
    use code::input_parsing::Range;
    use code::part1;
    use code::part2;

    #[test]
    fn test_11_22(){
        let start = 11;
        let end = 22;
        let exp = 33;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp);
    }

    #[test]
    fn test_95_115(){
        let start = 95;
        let end = 115;
        let exp = 99;
        let exp2 = 210;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
    }

    #[test]
    fn test_998_1012(){
        let start = 998;
        let end = 1010;

        let exp = 1010;
        let exp2 = 2009;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
        
    }

    #[test]
    fn test_4(){
        let start = 1188511880;
        let end = 1188511890;
        let exp = 1188511885;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_5(){
        let start = 222220;
        let end = 222224;
        let exp = 222222;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp);
    }

    #[test]
    fn test_6(){
        let start = 1698522;
        let end = 1698528;
        let exp = 0;
        let exp2 = 0;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
    }

    #[test]
    fn test_7(){
        let start = 446443;
        let end = 446449;
        let exp = 446446;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_8(){
        let start = 38593856;
        let end = 38593862;

        let exp = 38593859;
        let exp2 = 38593859;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
    }

    
    #[test]
    fn test_9(){
        let start = 565653;
        let end = 565659;

        let exp = 0;
        let exp2 = 565656;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
    }
    
    
    #[test]
    fn test_10(){
        let start = 824824821;
        let end = 824824827;

        let exp = 0;
        let exp2 = 824824824;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = part1::get_dups_for_range(test_range);
        assert_eq!(dups, exp);

        let dups = part2::get_dups_for_range(test_range);
        assert_eq!(dups, exp2)
    }
}