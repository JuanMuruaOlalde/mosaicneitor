pub fn get_version_text() -> String {
    // I used a variant of semantic version:
    //   - When launching to production, igone .p an use only M.m
    //   - After launch, increment M.m and start working with M.m.1
    //   - Increment .p as need during M.m development work.
    // https://www.susosise.es/documentos/Una_sugerencia_practica_de_versionado_semantico.pdf

    let parsed_version_result = semver::Version::parse(env!("CARGO_PKG_VERSION"));
    match parsed_version_result {
        Ok(version) => {
            if version.patch == 0 {
                format!("(v{}.{})", version.major, version.minor)
            } else {
                format!(
                    "(v{}.{}.forTest{})",
                    version.major, version.minor, version.patch
                )
            }
        }
        Err(_error) => String::from("(..)"),
    }
}

pub fn round_preserving_aspect_ratio(
    dimensions_to_be_adjusted: [usize; 2],
    reference_dimensions: [usize; 2],
) -> [usize; 2] {
    let adjust_amount: isize = (dimensions_to_be_adjusted[0] * reference_dimensions[1]
        / reference_dimensions[0]) as isize
        - dimensions_to_be_adjusted[1] as isize;
    let adjusted_dimensions = [
        dimensions_to_be_adjusted[0],
        (dimensions_to_be_adjusted[1] as isize + adjust_amount) as usize,
    ];
    adjusted_dimensions.map(|x| ((x as f32 / 10.0).ceil() * 10.0) as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rounder_works_for_a_realistic_image() {
        let reference_for_proportionality = [739, 381];

        let initial_dimensions = [200, 100];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [200, 110]);

        let initial_dimensions = [100, 200];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [100, 60]);

        let initial_dimensions = [401, 400];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [410, 210]);
    }

    #[test]
    fn rounder_rounds_to_the_nearest_ten() {
        let reference_for_proportionality = [1, 1];

        let initial_dimensions = [75, 50];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [80, 80]);

        let initial_dimensions = [345, 234];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [350, 350]);

        let initial_dimensions = [34567, 23456];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [34570, 34570]);
    }

    #[test]
    fn rounder_enforces_aspect_ratio() {
        let reference_for_proportionality = [16, 9];

        let initial_dimensions = [75, 50];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [80, 50]);

        let initial_dimensions = [345, 234];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [350, 200]);

        let initial_dimensions = [34567, 23456];
        let adjusted_dimensions =
            round_preserving_aspect_ratio(initial_dimensions, reference_for_proportionality);
        assert_eq!(adjusted_dimensions, [34570, 19450]);
    }
}
