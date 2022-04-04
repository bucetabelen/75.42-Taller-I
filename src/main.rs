use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let seq1 = read_file_lines("./a.txt")?;

    let seq2 = read_file_lines("./b.txt")?;

    print_diff(lcs(&seq1, &seq2), &seq1, &seq2, seq1.len(), seq2.len());

    Ok(())
}
/// Returns a Vec<String> containing the lines of the file.
fn read_file_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let mut data = Vec::new();
    let f = read_lines(&filename)?;
    for line in f.into_iter().flatten() {
        data.push(line.to_string());
    }

    Ok(data)
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
/// let seq1 and se2 be sequences
/// let m be the length of seq1, and let n be the length of seq2
/// Returns a grid with the lengths of subsequences.
fn lcs(seq1: &[String], seq2: &[String]) -> Vec<Vec<i32>> {
    let m: usize = seq1.len();

    let n: usize = seq2.len();

    let mut grid = vec![vec![0; n + 1]; m + 1];

    for (i, s1) in seq1.iter().enumerate().take(m) {
        for (j, s2) in seq2.iter().enumerate().take(n) {
            println!("i={} y j={}", i, j);

            if s1 == s2 {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else {
                grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
            }
        }
    }
    grid
}

/// 'grid' is the computed grid by lcs(seq1: &[String], seq2: &[String])
/// seq1 and seq2 are the sequences
/// i y j specify the location within 'grid' that we want to search for when diff is read.
/// Initially we call the fn with i = len(m) j = len(n).
fn print_diff(grid: Vec<Vec<i32>>, seq1: &[String], seq2: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && seq1[i - 1] == seq2[j - 1] {
        print_diff(grid, seq1, seq2, i - 1, j - 1);

        print!("{} ", seq1[i - 1]);
    } else if j > 0 && (i == 0 || (grid[i][j - 1] >= grid[i - 1][j])) {
        print_diff(grid, seq1, seq2, i, j - 1);

        print!("> {} ", seq2[j - 1]);
    } else if i > 0 && (j == 0 || grid[i][j - 1] < grid[i - 1][j]) {
        print_diff(grid, seq1, seq2, i - 1, j);

        print!("< {} ", seq1[i - 1]);
    } else {
        print!("");
    }
}
