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

pub fn adjust_proportions(
    dimensions_to_be_adjusted: [usize; 2],
    reference_dimensions: [usize; 2],
) -> [usize; 2] {
    let ratio = reference_dimensions[0] as f64 / reference_dimensions[1] as f64;
    let ajusted_dimensions = dimensions_to_be_adjusted.map(|x| x as f64 * ratio);
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
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [80, 50]);

        let initial_dimensions = [345, 234];
        let rounded_dimensions =
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [350, 240]);

        let initial_dimensions = [34567, 23456];
        let rounded_dimensions =
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [34570, 23460]);
    }

    #[test]
    fn rounder_maintains_proportionality() {
        let reference_for_proportionality = [16, 9];

        let initial_dimensions = [75, 50];
        let rounded_dimensions =
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [140, 90]);

        let initial_dimensions = [345, 234];
        let rounded_dimensions =
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [620, 420]);

        let initial_dimensions = [34567, 23456];
        let rounded_dimensions =
            adjust_proportions(initial_dimensions, reference_for_proportionality);
        assert_eq!(rounded_dimensions, [61460, 41700]);
    }
}
