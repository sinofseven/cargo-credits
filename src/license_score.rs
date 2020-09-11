use regex::Regex;
#[cfg(test)]
use rstest::rstest;

pub struct LicenseScorer {
    re_score_100_0: Regex,
    re_score_095_0: Regex,
    re_score_090_0: Regex,
    re_score_085_0: Regex,
    re_score_080_0: Regex,
    re_score_080_1: Regex,
    re_score_075_0: Regex,
    re_score_075_1: Regex,
    re_score_070_0: Regex,
    re_score_065_0: Regex,
    re_score_060_0: Regex,
    re_score_055_0: Regex,
    re_score_050_0: Regex,
    re_score_045_0: Regex,
    re_score_045_1: Regex,
    re_score_040_0: Regex,
    re_score_035_0: Regex,
    re_score_030_0: Regex,
    re_score_030_1: Regex,
}

impl LicenseScorer {
    pub fn new() -> LicenseScorer {
        LicenseScorer {
            re_score_100_0: Regex::new(r"(?-mix:\A(?i-mx:(un)?licen[sc]e)\z)").unwrap(),
            re_score_095_0: Regex::new(
                r"(?-mix:\A(?i-mx:(un)?licen[sc]e)(?-mix:\.(?-mix:md|markdown|txt|html)\z)\z)",
            )
            .unwrap(),
            re_score_090_0: Regex::new(r"(?-mix:\A(?i-mx:copy(ing|right))\z)").unwrap(),
            re_score_085_0: Regex::new(
                r"(?-mix:\A(?i-mx:copy(ing|right))(?-mix:\.(?-mix:md|markdown|txt|html)\z)\z)",
            )
            .unwrap(),
            // r"(?-mix:\A(?i-mx:(un)?licen[sc]e)(?i-mx:\.(?!spdx|header|gemspec)[^./]+\z)\z)"
            re_score_080_0: Regex::new(r"(?-mix:\A(?i-mx:(un)?licen[sc]e)(?i-mx:\.[^./]+\z)\z)")
                .unwrap(),
            re_score_080_1: Regex::new(
                r"(?-mix:\A(?i-mx:(un)?licen[sc]e)(?i-mx:\.(spdx|header|gemspec)[^./]+\z)\z)",
            )
            .unwrap(),
            // r"(?-mix:\A(?i-mx:copy(ing|right))(?i-mx:\.(?!spdx|header|gemspec)[^./]+\z)\z)"
            re_score_075_0: Regex::new(r"(?-mix:\A(?i-mx:copy(ing|right))(?i-mx:\.[^./]+\z)\z)")
                .unwrap(),
            re_score_075_1: Regex::new(
                r"(?-mix:\A(?i-mx:copy(ing|right))(?i-mx:\.(spdx|header|gemspec)[^./]+\z)\z)",
            )
            .unwrap(),
            re_score_070_0: Regex::new(r"(?-mix:\A(?i-mx:(un)?licen[sc]e)[-_])").unwrap(),
            re_score_065_0: Regex::new(r"(?-mix:\A(?i-mx:copy(ing|right))[-_])").unwrap(),
            re_score_060_0: Regex::new(r"(?-mix:[-_](?i-mx:(un)?licen[sc]e))").unwrap(),
            re_score_055_0: Regex::new(r"(?-mix:[-_](?i-mx:copy(ing|right)))").unwrap(),
            re_score_050_0: Regex::new(
                r"(?-mix:\A(?i-mx:ofl)(?-mix:\.(?-mix:md|markdown|txt|html)\z))",
            )
            .unwrap(),
            // r"(?-mix:\A(?i-mx:ofl)(?i-mx:\.(?!spdx|header|gemspec)[^./]+\z))"
            re_score_045_0: Regex::new(r"(?-mix:\A(?i-mx:ofl)(?i-mx:\.[^./]+\z))").unwrap(),
            re_score_045_1: Regex::new(
                r"(?-mix:\A(?i-mx:ofl)(?i-mx:\.(spdx|header|gemspec)[^./]+\z))",
            )
            .unwrap(),
            re_score_040_0: Regex::new(r"(?-mix:\A(?i-mx:ofl)\z)").unwrap(),
            re_score_035_0: Regex::new(r"(?-mix:\A(?i-mx:patents)\z)").unwrap(),
            // r"(?-mix:\A(?i-mx:patents)(?i-mx:\.(?!spdx|header|gemspec)[^./]+\z)\z)"
            re_score_030_0: Regex::new(r"(?-mix:\A(?i-mx:patents)(?i-mx:\.[^./]+\z)\z)").unwrap(),
            re_score_030_1: Regex::new(
                r"(?-mix:\A(?i-mx:patents)(?i-mx:\.(spdx|header|gemspec)[^./]+\z)\z)",
            )
            .unwrap(),
        }
    }

    fn check_score_100(&self, tex: &str) -> bool {
        self.re_score_100_0.is_match(tex)
    }

    fn check_score_095(&self, tex: &str) -> bool {
        self.re_score_095_0.is_match(tex)
    }

    fn check_score_090(&self, tex: &str) -> bool {
        self.re_score_090_0.is_match(tex)
    }

    fn check_score_085(&self, tex: &str) -> bool {
        self.re_score_085_0.is_match(tex)
    }

    fn check_score_080(&self, tex: &str) -> bool {
        self.re_score_080_0.is_match(tex) && !self.re_score_080_1.is_match(tex)
    }

    fn check_score_075(&self, tex: &str) -> bool {
        self.re_score_075_0.is_match(tex) && !self.re_score_075_1.is_match(tex)
    }

    fn check_score_070(&self, tex: &str) -> bool {
        self.re_score_070_0.is_match(tex)
    }

    fn check_score_065(&self, tex: &str) -> bool {
        self.re_score_065_0.is_match(tex)
    }

    fn check_score_060(&self, tex: &str) -> bool {
        self.re_score_060_0.is_match(tex)
    }

    fn check_score_055(&self, tex: &str) -> bool {
        self.re_score_055_0.is_match(tex)
    }

    fn check_score_050(&self, tex: &str) -> bool {
        self.re_score_050_0.is_match(tex)
    }

    fn check_score_045(&self, tex: &str) -> bool {
        self.re_score_045_0.is_match(tex) && !self.re_score_045_1.is_match(tex)
    }

    fn check_score_040(&self, tex: &str) -> bool {
        self.re_score_040_0.is_match(tex)
    }

    fn check_score_035(&self, tex: &str) -> bool {
        self.re_score_035_0.is_match(tex)
    }

    fn check_score_030(&self, tex: &str) -> bool {
        self.re_score_030_0.is_match(tex) && !self.re_score_030_1.is_match(tex)
    }

    pub fn get_score(&self, tex: &str) -> u8 {
        return if self.check_score_100(tex) {
            100
        } else if self.check_score_095(tex) {
            95
        } else if self.check_score_090(tex) {
            90
        } else if self.check_score_085(tex) {
            85
        } else if self.check_score_080(tex) {
            80
        } else if self.check_score_075(tex) {
            75
        } else if self.check_score_070(tex) {
            70
        } else if self.check_score_065(tex) {
            65
        } else if self.check_score_060(tex) {
            60
        } else if self.check_score_055(tex) {
            55
        } else if self.check_score_050(tex) {
            50
        } else if self.check_score_045(tex) {
            45
        } else if self.check_score_040(tex) {
            40
        } else if self.check_score_035(tex) {
            35
        } else if self.check_score_030(tex) {
            30
        } else {
            0
        };
    }
}

#[test]
#[rstest(
    tex,
    expected,
    case("license", true),
    case("licence", true),
    case("LICENSE", true),
    case("LICENCE", true),
    case("alicense", false),
    case("licensea", false),
    case("unlicense", true),
    case("UNLICENSE", true),
    case("aunlicense", false),
    case("AUNLICENSE", false)
)]
fn test_check_score_100(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_100(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("license.md", true),
    case("LICENSE.md", true),
    case("license.txt", true)
)]
fn test_check_score_095(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_095(tex));
}

#[test]
#[rstest(tex, expected, case("COPYING", true), case("copyRIGHT", true))]
fn test_check_score_090(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_090(tex));
}

#[test]
#[rstest(tex, expected, case("COPYRIGHT.txt", true), case("copying.txt", true))]
fn test_check_score_085(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_085(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("LICENSE.php", true),
    case("LICENCE.docs", true),
    case("LICENSE.spdxa", false)
)]
fn test_check_score_080(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_080(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("copying.image", true),
    case("COPYRIGHT.go", true),
    case("COPYRIGHT.spdxa", false)
)]
fn test_check_score_075(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_075(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("LICENSE-MIT", true),
    case("LICENSE_1_0.txt", true)
)]
fn test_check_score_070(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_070(tex));
}

#[test]
#[rstest(tex, expected, case("COPYING-GPL", true), case("COPYRIGHT-BSD", true))]
fn test_check_score_065(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_065(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("MIT-LICENSE.txt", true),
    case("mit-license-foo.md", true)
)]
fn test_check_score_060(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_060(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("MIT-copying.txt", true),
    case("mit-COPYING-FOO.md", true)
)]
fn test_check_score_055(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_055(tex));
}

#[test]
#[rstest(tex, expected, case("OFL.md", true))]
fn test_check_score_050(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_050(tex));
}

#[test]
#[rstest(tex, expected, case("ofl.textile", true), case("ofl.spdxa", false))]
fn test_check_score_045(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_045(tex));
}

#[test]
#[rstest(tex, expected, case("ofl", true), case("ofl.md", false))]
fn test_check_score_040(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_040(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("patents", true),
    case("PATENTS", true),
    case("patents.md", false)
)]
fn test_check_score_035(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_035(tex));
}

#[test]
#[rstest(tex, expected, case("patents.md", true), case("PATENTS.spdxa", false))]
fn test_check_score_030(tex: &str, expected: bool) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.check_score_030(tex));
}

#[test]
#[rstest(
    tex,
    expected,
    case("license", 100),
    case("LICENCE", 100),
    case("unLICENSE", 100),
    case("unlicence", 100),
    case("license.md", 95),
    case("LICENSE.md", 95),
    case("license.txt", 95),
    case("COPYING", 90),
    case("copyRIGHT", 90),
    case("COPYRIGHT.txt", 85),
    case("copying.txt", 85),
    case("LICENSE.php", 80),
    case("LICENCE.docs", 80),
    case("copying.image", 75),
    case("COPYRIGHT.go", 75),
    case("LICENSE-MIT", 70),
    case("LICENSE_1_0.txt", 70),
    case("COPYING-GPL", 65),
    case("COPYRIGHT-BSD", 65),
    case("MIT-LICENSE.txt", 60),
    case("mit-license-foo.md", 60),
    case("MIT-copying.txt", 55),
    case("mit-COPYING-FOO.md", 55),
    case("OFL.md", 50),
    case("ofl.textile", 45),
    case("ofl", 40),
    case("patents", 35),
    case("PATENTS", 35),
    case("patents.md", 30),
    case("not-the-ofl", 0),
    case("README.txt", 0)
)]
fn test_get_score(tex: &str, expected: u8) {
    let regexes = LicenseScorer::new();
    assert_eq!(expected, regexes.get_score(tex));
}
