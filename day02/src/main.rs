// use std::cmp::Ordering;
// use std::io::{self, BufRead};
//
// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//
//     let mut safe_reports = 0;
//     for line in stdin.lock().lines() {
//         let line = line?;
//
//         let report: Vec<i32> = line
//             .split_whitespace()
//             .map(|s| s.parse::<i32>().expect("Failed to parse number"))
//             .collect();
//
//         if report_is_safe_1(&report) {
//             safe_reports += 1;
//         }
//     }
//
//     println!("{}", safe_reports);
//
//     Ok(())
// }
//
// fn pair_diff_safe(curr: i32, prev: i32) -> bool {
//     let diff = (curr - prev).abs();
//     if !(0 < diff && diff < 4) {
//         return false;
//     }
//
//     return true;
// }
//
// fn pair_order_safe(curr: i32, prev: i32, order: Ordering) -> bool {
//     if curr.cmp(&prev) != order {
//         return false;
//     }
//
//     return true;
// }
//
// fn pair_is_safe(curr: i32, prev: i32, order: Ordering) -> bool {
//     pair_diff_safe(curr, prev) && pair_order_safe(curr, prev, order)
// }
//
// fn report_is_safe_2(v: &[i32]) -> bool {
//     if v.len() == 1 {
//         return true;
//     }
//
//     let mut prev = v[0];
//     let mut report_is_safe;
//
//     for &curr in v.iter().skip(1) {
//         let order = curr.cmp(&prev);
//         report_is_safe = true;
//         if !pair_is_safe(curr, prev, order) {
//             report_is_safe = false;
//             break;
//         }
//     }
//     if report_is_safe {
//         return true;
//     } 
//
//     for n in 0..(v.len() - 1)  {
//         for &curr in v.iter().enumerate().filter(|&(i, _)| i != n).map(|(_, v)| v) {
//
//         }
//     }
// }
//
// fn report_is_safe_1(v: &[i32]) -> bool {
//     let mut previous = v[0];
//     let mut start = 1;
//     let is_increasing = v[start] - previous > 0;
//
//     while start < v.len() {
//         let keeps_increasing = v[start] - previous > 0;
//
//         if is_increasing != keeps_increasing {
//             return false;
//         }
//
//         let transition = (v[start] - previous).abs();
//         let transition_is_safe = 0 < transition && transition < 4;
//
//         if !transition_is_safe {
//             return false;
//         }
//
//         previous = v[start];
//         start += 1;
//     }
//
//     return true;
// }
