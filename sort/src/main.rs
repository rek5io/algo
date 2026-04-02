use sort::*;

fn main() {
    let sizes = [10_000, 50_000, 100_000, 500_000, 1_000_000];
    let percent_sorted = [0.0, 0.25, 0.5, 0.75, 0.95, 0.99, 0.997];
    let n_times = 100;

    for &s in &sizes {
        for &p in &percent_sorted {
            let avg_ns = sort_test_n_avg(n_times, s, p, false, merge::merge_sort);
            println!(
                "merge_sort | size: {} | sorted: {:.3} | avg time: {:.0} us",
                s, p, avg_ns
            );

            let avg_ns = sort_test_n_avg(n_times, s, p, false, quick::quick_sort);
            println!(
                "quick_sort | size: {} | sorted: {:.3} | avg time: {:.0} us",
                s, p, avg_ns
            );

            let avg_ns = sort_test_n_avg(n_times, s, p, false, intro::intro_sort);
            println!(
                "intro_sort | size: {} | sorted: {:.3} | avg time: {:.0} us",
                s, p, avg_ns
            );
        }

        let avg_ns = sort_test_n_avg(n_times, s, 0.0, true, merge::merge_sort);
        println!(
            "merge_sort | size: {} | reversed | avg time: {:.0} us",
            s, avg_ns
        );

        let avg_ns = sort_test_n_avg(n_times, s, 0.0, true, quick::quick_sort);
        println!(
            "quick_sort | size: {} | reversed | avg time: {:.0} us",
            s, avg_ns
        );

        let avg_ns = sort_test_n_avg(n_times, s, 0.0, true, intro::intro_sort);
        println!(
            "intro_sort | size: {} | reversed | avg time: {:.0} us",
            s, avg_ns
        );
    }
}
