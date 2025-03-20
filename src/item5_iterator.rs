// take(n)
// skip(n)
// step_by(n)
// chain(other)
// cycle()
// rev()
// map
// cloned
// copied()
// enumerate()
// zip(it)
// filter

// fold
// max
// min
// find
// nth
// any
// all

fn main() {
    let sample_data = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13];
    let total_sum = sample_data
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum::<i32>();
    println!("total_sum = {}", total_sum);

    let min_v = sample_data.into_iter().min().unwrap_or(0);
    let max_v = sample_data.into_iter().max().unwrap_or(0);
    println!("{} ~ {}", min_v, max_v);
    let sum_2x = sample_data
        .iter()
        .chain(sample_data.iter())
        .fold(0, |acc, x| acc + x);
    println!("{}", sum_2x);
}
