use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use super::Range;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(Range);

impl From<Range> for Page {
    fn from(r: Range) -> Self {
        Page(r)
    }
}

impl FromStr for Page {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Range>().map(Into::into)
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p.~{}", self.0)
    }
}

impl Debug for Page {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[test]
fn it_works() {
    let cases = [
        ("217", Ok(Page(217.into()))),
        ("282", Ok(Page(282.into()))),
        ("170", Ok(Page(170.into()))),
        ("212", Ok(Page(212.into()))),
        ("268", Ok(Page(268.into()))),
        ("258", Ok(Page(258.into()))),
        ("275", Ok(Page(275.into()))),
        ("207", Ok(Page(207.into()))),
        ("171", Ok(Page(171.into()))),
        ("237", Ok(Page(237.into()))),
        ("p.152", Ok(Page(152.into()))),
        ("229", Ok(Page(229.into()))),
        ("313", Ok(Page(313.into()))),
        ("307", Ok(Page(307.into()))),
        ("279", Ok(Page(279.into()))),
        ("233", Ok(Page(233.into()))),
        ("191", Ok(Page(191.into()))),
        ("194", Ok(Page(194.into()))),
        ("302", Ok(Page(302.into()))),
        ("298", Ok(Page(298.into()))),
        ("260", Ok(Page(260.into()))),
        ("241", Ok(Page(241.into()))),
        ("221", Ok(Page(221.into()))),
        ("285", Ok(Page(285.into()))),
        ("206", Ok(Page(206.into()))),
        ("228", Ok(Page(228.into()))),
        ("274", Ok(Page(274.into()))),
        ("290", Ok(Page(290.into()))),
        ("281", Ok(Page(281.into()))),
        ("195", Ok(Page(195.into()))),
        ("292", Ok(Page(292.into()))),
        ("238", Ok(Page(238.into()))),
        ("263", Ok(Page(263.into()))),
        ("270", Ok(Page(270.into()))),
        ("255", Ok(Page(255.into()))),
        ("175", Ok(Page(175.into()))),
        ("214", Ok(Page(214.into()))),
        ("210", Ok(Page(210.into()))),
        ("201", Ok(Page(201.into()))),
        ("296", Ok(Page(296.into()))),
        ("236", Ok(Page(236.into()))),
        ("291", Ok(Page(291.into()))),
        ("286", Ok(Page(286.into()))),
        ("278", Ok(Page(278.into()))),
        ("295", Ok(Page(295.into()))),
        ("p.151", Ok(Page(151.into()))),
        ("227", Ok(Page(227.into()))),
        ("230", Ok(Page(230.into()))),
        ("269", Ok(Page(269.into()))),
        ("256", Ok(Page(256.into()))),
        ("310", Ok(Page(310.into()))),
        ("223", Ok(Page(223.into()))),
        ("267", Ok(Page(267.into()))),
        ("311", Ok(Page(311.into()))),
        ("336", Ok(Page(336.into()))),
        ("273", Ok(Page(273.into()))),
        ("224", Ok(Page(224.into()))),
        ("252", Ok(Page(252.into()))),
        ("297", Ok(Page(297.into()))),
        ("172", Ok(Page(172.into()))),
        ("p.159", Ok(Page(159.into()))),
        ("272", Ok(Page(272.into()))),
        ("284", Ok(Page(284.into()))),
        ("219", Ok(Page(219.into()))),
        ("248", Ok(Page(248.into()))),
        ("198", Ok(Page(198.into()))),
        ("249", Ok(Page(249.into()))),
        ("211", Ok(Page(211.into()))),
        ("352", Ok(Page(352.into()))),
        ("301", Ok(Page(301.into()))),
        ("216", Ok(Page(216.into()))),
        ("280", Ok(Page(280.into()))),
        ("220", Ok(Page(220.into()))),
        ("339", Ok(Page(339.into()))),
        ("p.170", Ok(Page(170.into()))),
        ("203", Ok(Page(203.into()))),
        ("196", Ok(Page(196.into()))),
        ("215", Ok(Page(215.into()))),
        ("312", Ok(Page(312.into()))),
        ("335", Ok(Page(335.into()))),
        ("234", Ok(Page(234.into()))),
        ("174", Ok(Page(174.into()))),
        ("254", Ok(Page(254.into()))),
        ("251", Ok(Page(251.into()))),
        ("370", Ok(Page(370.into()))),
        ("213", Ok(Page(213.into()))),
        ("208", Ok(Page(208.into()))),
        ("247", Ok(Page(247.into()))),
        ("306", Ok(Page(306.into()))),
        ("250", Ok(Page(250.into()))),
        ("p.145", Ok(Page(145.into()))),
        ("287", Ok(Page(287.into()))),
        ("262", Ok(Page(262.into()))),
        ("362", Ok(Page(362.into()))),
        ("193", Ok(Page(193.into()))),
        ("p.411", Ok(Page(411.into()))),
        ("169", Ok(Page(169.into()))),
        ("331", Ok(Page(331.into()))),
        ("355", Ok(Page(355.into()))),
        ("376", Ok(Page(376.into()))),
        ("345", Ok(Page(345.into()))),
        ("p.133", Ok(Page(133.into()))),
        ("364", Ok(Page(364.into()))),
        ("209", Ok(Page(209.into()))),
        ("232", Ok(Page(232.into()))),
        ("244", Ok(Page(244.into()))),
        ("379", Ok(Page(379.into()))),
        ("328", Ok(Page(328.into()))),
        ("243", Ok(Page(243.into()))),
        ("344", Ok(Page(344.into()))),
        ("265", Ok(Page(265.into()))),
        ("341", Ok(Page(341.into()))),
        ("381", Ok(Page(381.into()))),
        ("246", Ok(Page(246.into()))),
        ("222", Ok(Page(222.into()))),
        ("p.166", Ok(Page(166.into()))),
        ("245", Ok(Page(245.into()))),
        ("338", Ok(Page(338.into()))),
        ("p.154", Ok(Page(154.into()))),
        ("p.131", Ok(Page(131.into()))),
        ("343", Ok(Page(343.into()))),
        ("p.406", Ok(Page(406.into()))),
        ("326", Ok(Page(326.into()))),
        ("p.217", Ok(Page(217.into()))),
        ("p.158", Ok(Page(158.into()))),
        ("317", Ok(Page(317.into()))),
        ("p.407", Ok(Page(407.into()))),
        ("p.397", Ok(Page(397.into()))),
        ("323", Ok(Page(323.into()))),
        ("321", Ok(Page(321.into()))),
        ("304", Ok(Page(304.into()))),
        ("378", Ok(Page(378.into()))),
        ("305", Ok(Page(305.into()))),
        ("374", Ok(Page(374.into()))),
        ("182", Ok(Page(182.into()))),
        ("p.156", Ok(Page(156.into()))),
        ("p.400", Ok(Page(400.into()))),
        ("180", Ok(Page(180.into()))),
        ("p.208", Ok(Page(208.into()))),
        ("p.408", Ok(Page(408.into()))),
        ("p.214", Ok(Page(214.into()))),
        ("p.169", Ok(Page(169.into()))),
        ("259", Ok(Page(259.into()))),
        ("p.204", Ok(Page(204.into()))),
        ("337", Ok(Page(337.into()))),
        ("p.218", Ok(Page(218.into()))),
        ("p.395", Ok(Page(395.into()))),
        ("347", Ok(Page(347.into()))),
        ("p.155", Ok(Page(155.into()))),
        ("p.168", Ok(Page(168.into()))),
        ("276", Ok(Page(276.into()))),
        ("368", Ok(Page(368.into()))),
        ("325", Ok(Page(325.into()))),
        ("365", Ok(Page(365.into()))),
        ("342", Ok(Page(342.into()))),
        ("p.460", Ok(Page(460.into()))),
        ("p.403", Ok(Page(403.into()))),
        ("p.165", Ok(Page(165.into()))),
        ("170-171", Ok(Page((170..=171).into()))),
        ("274/275", Ok(Page((274..=275).into()))),
        ("277/278", Ok(Page((277..=278).into()))),
        ("281/282", Ok(Page((281..=282).into()))),
        ("325/326", Ok(Page((325..=326).into()))),
        ("p.406-407", Ok(Page((406..=407).into()))),
        ("p.445", Ok(Page(455.into()))),
        ("310/311", Ok(Page((310..=311).into()))),
        ("296/297", Ok(Page((296..=297).into()))),
        ("204 - 205", Ok(Page((204..=205).into()))),
        ("227 – 228", Ok(Page((227..=228).into()))),
        ("191 e 192", Ok(Page((191..=192).into()))),
        ("p.167-8", Ok(Page((167..=168).into()))),
    ];

    for (resp, expec) in cases.iter().map(|(s, e)| (s.parse(), e)) {
        assert_eq!(resp, *expec)
    }
}
