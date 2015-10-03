fn place_queens(n: usize, placed_queens: Vec<usize>, open_ys: Vec<usize>, intercepts_up: Vec<usize>, intercepts_down: Vec<usize>)  {
    if open_ys.is_empty() { //success!!
        for a in 0..placed_queens.len() {
            print!(" ({}, {})", a, placed_queens[a]);
        }
        println!("");
        return;
    }

    let available_ys = open_ys.clone();
    let x = placed_queens.len();

    for c in 0..open_ys.len() {
        //check if you can place a queen
        let y = open_ys[c];
        let b1 = n + y - x; //y = x + b => b = y - x, added n to make positve
        let b2 = y + x; // y = (-1)x + b => b = y + x
        if !intercepts_up.contains(&b1) && !intercepts_down.contains(&b2) {
            let mut queens = placed_queens.to_vec();
            queens.push(y);
            let mut ys = available_ys.to_vec();
            ys.remove(c);
            let mut ups = intercepts_up.to_vec();
            ups.push(b1);
            let mut downs = intercepts_down.to_vec();
            downs.push(b2);
            place_queens(n, queens, ys, ups, downs);
        }
    }
}

fn main() {
    let n = 5;
    let y_values= (0..n).collect::<Vec<usize>>();

    place_queens(n, vec![], y_values, vec![], vec![] );
}
