use emoji_cluster_split::segment::split_clusters;

struct Case {
    name: &'static str,
    input: &'static str,
    expected_clusters: usize,
    codepoints_per_cluster: &'static [usize],
}

// Each row is a case that has, at some point, tripped up naive "split on
// chars()" or "count graphemes" logic for emoji specifically.
const CASES: &[Case] = &[
    Case {
        name: "plain ascii stays one cluster per letter",
        input: "abc",
        expected_clusters: 3,
        codepoints_per_cluster: &[1, 1, 1],
    },
    Case {
        name: "simple emoji, no modifiers",
        input: "\u{1F600}",
        expected_clusters: 1,
        codepoints_per_cluster: &[1],
    },
    Case {
        name: "skin tone modifier attaches to its base",
        input: "\u{1F44B}\u{1F3FD}",
        expected_clusters: 1,
        codepoints_per_cluster: &[2],
    },
    Case {
        name: "four-person zwj family sequence",
        input: "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}",
        expected_clusters: 1,
        codepoints_per_cluster: &[7],
    },
    Case {
        name: "flag is a pair of regional indicators",
        input: "\u{1F1FA}\u{1F1F8}",
        expected_clusters: 1,
        codepoints_per_cluster: &[2],
    },
    Case {
        name: "two flags back to back don't merge",
        input: "\u{1F1FA}\u{1F1F8}\u{1F1EC}\u{1F1E7}",
        expected_clusters: 2,
        codepoints_per_cluster: &[2, 2],
    },
    Case {
        name: "trailing lone regional indicator stands alone",
        input: "\u{1F1FA}\u{1F1F8}\u{1F1EC}",
        expected_clusters: 2,
        codepoints_per_cluster: &[2, 1],
    },
    Case {
        name: "keycap sequence: digit + variation selector + keycap mark",
        input: "1\u{FE0F}\u{20E3}",
        expected_clusters: 1,
        codepoints_per_cluster: &[3],
    },
    Case {
        name: "text presentation selector isn't a new cluster",
        input: "\u{263A}\u{FE0E}",
        expected_clusters: 1,
        codepoints_per_cluster: &[2],
    },
    Case {
        name: "skin tone + zwj + gender sign + variation selector",
        input: "\u{1F926}\u{1F3FD}\u{200D}\u{2640}\u{FE0F}",
        expected_clusters: 1,
        codepoints_per_cluster: &[5],
    },
    Case {
        name: "england subdivision flag: tag base, tag letters, cancel tag",
        input: "\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}",
        expected_clusters: 1,
        codepoints_per_cluster: &[7],
    },
    Case {
        name: "emoji followed by a plain letter doesn't absorb it",
        input: "\u{1F600}x",
        expected_clusters: 2,
        codepoints_per_cluster: &[1, 1],
    },
];

#[test]
fn table_driven_cluster_cases() {
    for case in CASES {
        let clusters = split_clusters(case.input);
        assert_eq!(
            clusters.len(),
            case.expected_clusters,
            "case '{}': wrong cluster count",
            case.name
        );

        let counts: Vec<usize> = clusters.iter().map(|c| c.chars().count()).collect();
        assert_eq!(
            counts, case.codepoints_per_cluster,
            "case '{}': wrong codepoints per cluster",
            case.name
        );
    }
}
