use super::Vote;

type PartialResults = Vec<(String, u64)>;

pub fn partial_results(votes: Vec<Vote>, options: Vec<String>) -> PartialResults {
    let mut results = Vec::new();

    for option in options {
        let mut count = 0;

        for vote in &votes {
            if vote.options.contains(&option) {
                count += 1;
            }
        }

        results.push((option, count));
    }

    results
}

/**
    Returns a string with a progress bar for each option.

    e.g.:
    Option 1: ████░░░░░░ 45%
    Option 2: ████████░░ 75%
*/
pub fn progress_bar(votes: Vec<Vote>, options: Vec<String>) -> String {
    let results = partial_results(votes, options);
    let mut progress_bar = String::new();

    let total_votes = results.iter().fold(0, |acc, (_, count)| acc + count);

    for (option, count) in results {
        let percentage = (count as f64 / total_votes as f64 * 100.0) as u64;

        progress_bar.push_str(&format!("{}: ", option));

        for _ in 0..percentage / 10 {
            progress_bar.push('█');
        }

        for _ in 0..(100 - percentage) / 10 {
            progress_bar.push('░');
        }

        progress_bar.push_str(&format!(" {}%\n", percentage));
    }

    progress_bar
}
