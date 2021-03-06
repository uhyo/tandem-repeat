extern crate bencher;
extern crate getopts;
extern crate rand;

pub mod options;

pub mod algorithm;

#[cfg(test)]
mod test {
    mod ultra_naive {
        use algorithm;
        use algorithm::result::AlgoResult;
        #[test]
        fn test() {
            assert_eq!(
                algorithm::ultra_naive::algorithm("foofoobarfoobarbarfooaba\0".as_bytes()),
                AlgoResult {
                    from: 3,
                    length: 6,
                    count: 2,
                }
            );
        }
    }
    mod divide {
        use algorithm;
        use algorithm::result::AlgoResult;
        #[test]
        fn test() {
            assert_eq!(
                algorithm::divide::algorithm("foofoobarfoobarbarfooaba\0".as_bytes()),
                AlgoResult {
                    from: 3,
                    length: 6,
                    count: 2,
                }
            );
        }
        #[test]
        fn test_left() {
            assert_eq!(
                //                            012345678901234567890123456789012345678
                algorithm::divide::algorithm("foobarfoobaraoaoaojfuwjhgnebfyu78yr32hf\0".as_bytes()),
                AlgoResult {
                    from: 0,
                    length: 6,
                    count: 2,
                }
            );
        }
    }
    mod lcp {
        use algorithm;
        use algorithm::result::AlgoResult;
        #[test]
        fn test() {
            assert_eq!(
                algorithm::lcp::algorithm("foofoobarfoobarbarfooabafoobarbarfoo".as_bytes()),
                // algorithm::lcp::algorithm("aababcabddabcab\0".as_bytes()),
                AlgoResult {
                    from: 3,
                    length: 6,
                    count: 2,
                }
            );
        }
    }
    mod lcp_divide {
        use algorithm;
        use algorithm::result::AlgoResult;
        #[test]
        fn test() {
            assert_eq!(
                //                                012345678901234567890123456789012345678
                algorithm::lcp_divide::algorithm("foofoobarfoobarbarfooaba".as_bytes()),
                AlgoResult {
                    from: 3,
                    length: 6,
                    count: 2,
                }
            );
        }
        #[test]
        fn test_left() {
            assert_eq!(
                //                                012345678901234567890123456789012345678
                algorithm::lcp_divide::algorithm("foobarfoobaraoaoaojfuwjhgnebfyu78yr32hf".as_bytes()),
                //                                876543210987654321098765432109876543210
                AlgoResult {
                    from: 0,
                    length: 6,
                    count: 2,
                }
            );
        }
    }
}
