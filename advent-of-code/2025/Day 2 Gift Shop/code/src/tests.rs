#[cfg(test)]
mod tests{
    use crate::{Range, get_dups_for_range};

    #[test]
    fn test_11_22(){
        let start = 11;
        let end = 22;
        let exp = 2;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_95_115(){
        let start = 95;
        let end = 115;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_998_1012(){
        let start = 998;
        let end = 1012;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_4(){
        let start = 1188511880;
        let end = 1188511890;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_5(){
        let start = 222220;
        let end = 222224;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_6(){
        let start = 1698522;
        let end = 1698528;
        let exp = 0;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_7(){
        let start = 446443;
        let end = 446449;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }

    #[test]
    fn test_8(){
        let start = 38593856;
        let end = 38593862;
        let exp = 1;

        let test_range: &Range = &Range{start: start, end: end};

        let dups = get_dups_for_range(test_range);
        assert_eq!(dups, exp)
    }
}