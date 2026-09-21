use std::env;
use std::io::{self, Read};

use emoji_cluster_split::segment;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input = if args.is_empty() {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        buf.trim_end_matches('\n').to_string()
    } else {
        args.join(" ")
    };

    let clusters = segment::split_clusters(&input);

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
