pub fn round_to_the_nearest_tens(
    dimensions_to_round: [usize; 2],
    reference_dimensions_for_proportionality: [usize; 2],
) -> [usize; 2] {
    let ratio = reference_dimensions_for_proportionality[0] as f64
        / reference_dimensions_for_proportionality[1] as f64;
    let ajusted_dimensions = dimensions_to_round.map(|x| x as f64 * ratio);
    ajusted_dimensions.map(|x| ((x / 10.0).ceil() * 10.0) as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rounder_rounds() {
        let reference_for_proportionality = [1, 1];

        let initial_dimensions = [75, 50];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [80, 50]);

        let initial_dimensions = [345, 234];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [350, 240]);

        let initial_dimensions = [34567, 23456];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [34570, 23460]);
    }

    #[test]
    fn rounder_maintains_proportionality() {
        let reference_for_proportionality = [16, 9];

        let initial_dimensions = [75, 50];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [140, 90]);

        let initial_dimensions = [345, 234];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [620, 420]);

        let initial_dimensions = [34567, 23456];
        let rounded_dimensions =
            round_to_the_nearest_tens(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [61460, 41700]);
    }
}
