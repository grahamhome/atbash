/// Tests provided by Exercism.org
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_encode_yes() {
        assert_eq!(encode("yes"), "bvh");
    }
    #[test]
    fn test_encode_no() {
        assert_eq!(encode("no"), "ml");
    }
    #[test]
    fn test_encode_omg() {
        assert_eq!(encode("OMG"), "lnt");
    }
    #[test]
    fn test_encode_spaces() {
        assert_eq!(encode("O M G"), "lnt");
    }
    #[test]
    fn test_encode_mindblowingly() {
        assert_eq!(encode("mindblowingly"), "nrmwy oldrm tob");
    }
    #[test]
    fn test_encode_numbers() {
        assert_eq!(
            encode("Testing,1 2 3, testing."),
            "gvhgr mt123 gvhgr mt"
        );
    }
    #[test]
    fn test_encode_deep_thought() {
        assert_eq!(encode("Truth is fiction."), "gifgs rhurx grlm");
    }
    #[test]
    fn test_encode_all_the_letters() {
        assert_eq!(
            encode("The quick brown fox jumps over the lazy dog."),
            "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"
        );
    }
    #[test]
    fn test_decode_exercism() {
        assert_eq!(decode("vcvix rhn"), "exercism");
    }
    #[test]
    fn test_decode_a_sentence() {
        assert_eq!(
            decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v"),
            "anobstacleisoftenasteppingstone"
        );
    }
    #[test]
    fn test_decode_numbers() {
        assert_eq!(decode("gvhgr mt123 gvhgr mt"), "testing123testing");
    }
    #[test]
    fn test_decode_all_the_letters() {
        assert_eq!(
            decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"),
            "thequickbrownfoxjumpsoverthelazydog"
        );
    }
    #[test]
    fn test_decode_with_too_many_spaces() {
        assert_eq!(decode("vc vix    r hn"), "exercism");
    }
    #[test]
    fn test_decode_with_no_spaces() {
        assert_eq!(
            decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv"),
            "anobstacleisoftenasteppingstone",
        );
    }
}