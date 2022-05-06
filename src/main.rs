fn main() {
    let cin = std::io::stdin();
    let mut s = String::new();
    cin.read_line(&mut s).unwrap();

    let n = s.trim_end().parse::<i32>().unwrap();

    let mut s = String::new();
    cin.read_line(&mut s).unwrap();
    let shortcuts = s
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    let result = solve(n, &shortcuts);

    print_result(result);
}

fn check_shortcut(shortcuts: &[i32], dest: i32, position: i32) -> Option<i32> {
    let index = (position - 1) as usize;
    let shortcut = shortcuts.get(index).unwrap();

    if shortcut > &dest && dest < position - dest {
        Some(shortcut.to_owned())
    } else {
        None
    }
}

fn solve(intersections: i32, shortcuts: &[i32]) -> Vec<String> {
    // skip home and set it to 0
    let mut result: Vec<String> = vec![0.to_string()];

    for intersection in 2..=intersections {
        match check_shortcut(shortcuts, intersection, 1) {
            Some(check_shortcut) => {
                println!("{}", check_shortcut);

                let value = 1 + check_shortcut - intersection;
                result.push(value.to_string())
            }
            None => result.push((intersection - 1).to_string()),
        }
    }

    result
}

fn print_result(shortcuts: Vec<String>) {
    println!("{}", shortcuts.join(" "));
}

// fn is_valid_shortcut(shortcuts: &[i32]) -> bool {
//     unimplemented!()
// }
