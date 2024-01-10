use super::PollOption;

type PartialResults = Vec<(String, u64)>;

/**
    Returns a string with a progress bar for each option.

    e.g.:
    Option 1: ████░░░░░░ 45%
    Option 2: ████████░░ 75%
*/
pub fn progress_bar(options: Vec<PollOption>) -> String {
    let results: PartialResults = options
        .iter()
        .map(|option| (option.value.clone(), option.votes.len() as u64))
        .collect();
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
