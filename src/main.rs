use std::env;
use std::io::{self, Read};

use emoji_cluster_split::segment;

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let json_mode = raw_args.iter().any(|a| a == "--json");
    let runs_mode = raw_args.iter().any(|a| a == "--runs");
    let args: Vec<String> = raw_args
        .into_iter()
        .filter(|a| a != "--json" && a != "--runs")
        .collect();

    let input = if args.is_empty() {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        buf.trim_end_matches('\n').to_string()
    } else {
        args.join(" ")
    };

    let mut clusters = segment::split_clusters(&input);
    if runs_mode {
        clusters = segment::merge_text_runs(&clusters);
    }

    if json_mode {
        println!("{}", segment::to_json(&clusters));
        return;
    }

    for (i, cluster) in clusters.iter().enumerate() {
        let codes = segment::codepoints(cluster).join(" ");
        println!("{:>3}  {}   {}", i + 1, cluster, codes);
    }

    println!();
    println!(
        "{} cluster{}",
        clusters.len(),
        if clusters.len() == 1 { "" } else { "s" }
    );
}
