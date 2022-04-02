use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let seq1 = read_file_lines("./a.txt")?;

    let seq2 = read_file_lines("./b.txt")?;

    print_diff(lqs(&seq1, &seq2), &seq1, &seq2, seq1.len(), seq2.len());

    Ok(())
}

fn read_file_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    // File hosts must exist in current path before this produces output
    let mut data = Vec::new(); // The compiler knows the type
    let f = read_lines(&filename)?;
    for line in f.into_iter().flatten() {
        data.push(line.to_string());
    }
    /* if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(ip) = line {

                data.push(ip.to_string());

            }
        }
    }  */

    Ok(data)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//

fn lqs(seq1: &[String], seq2: &[String]) -> Vec<Vec<i32>> {
    //let X and Y be sequences
    //let m be the length of X, and let n be the length of Y
    let m: usize = seq1.len();

    let n: usize = seq2.len();

    //print!("{}, y {}", seq1[0], seq2[1]);
    // const  m: usize = 5;
    // const  n: usize = 5;
    //let mut c: Vec<Vec<u8>> = Vec::new(m+1, n+1);
    // let mut g : Vec<i32> = Vec::new();
    //let mut grid = Vec::new();
    //grid.push(g);

    //vamo a suponer que esto cumple con los primeros for
    let mut grid = vec![vec![0; n + 1]; m + 1];

    //print!("{:?} \n", grid);

    /*   for i in 0..m{
        for j in 0..n{

            println!("i={} y j={}",i,j);

            if seq1[i] == seq2[j] {
                grid[i+1][j+1] = grid[i][j] + 1;
            }else{
                grid[i+1][j+1] = cmp::max(grid[i+1][j], grid[i][j+1]);
            }

        }

    }   */

    //print!("{:?} ", grid);

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

// C es la grilla computada por lcs()
// X e Y son las secuencias
// i y j especifican la ubicacion dentro de C que se quiere buscar cuando
//    se lee el diff. Al llamar a estar funcion inicialmente, pasarle
//    i=len(X) y j=len(Y)
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
