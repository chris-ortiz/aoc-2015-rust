use crate::q12::q12;

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;
mod q10;
mod q11;
mod q12;

fn main() {
    q12();
}

// trying out permutation algo
fn permutations() {
    let arr = ['A', 'B', 'C'];
    let mut perms: Vec<Vec<char>> = Vec::new();
    let mut temp: Vec<char> = Vec::new();

    backtrack(&arr, &mut perms, &mut temp);

    println!("{:?}", perms)
}

fn backtrack(arr: &[char], mut perms: &mut Vec<Vec<char>>, temp: &mut Vec<char>) {
    if arr.len() == temp.len() {
        perms.push(temp.clone())
    }

    for el in arr {
        if !temp.contains(el) {
            temp.push(el.clone());
            backtrack(arr, perms, temp);
            temp.pop();
        }
    }
}
