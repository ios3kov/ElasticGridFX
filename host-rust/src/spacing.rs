//! New instances use the existing core's topology-safe numerical spacing floor.
//! Older projects retain their saved spacing, including animated hidden keys.
use super::*;

fn resolve(automatic: bool, legacy_percent: f64) -> f32 {
    if automatic { 0.0 } else { legacy_percent as f32 / 100.0 }
}

pub(crate) fn read(params: &ae::Parameters<Params>, checkout: bool) -> Result<f32, ae::Error> {
    let automatic = if checkout { params.checkout(Params::AutomaticSpacing)?.as_checkbox()?.value() }
        else { params.get(Params::AutomaticSpacing)?.as_checkbox()?.value() };
    // Checkout legacy value even in automatic mode: keeps the saved dependency
    // explicit, with no mutation or migration of its animation.
    let legacy = if checkout { checked_float(params, Params::MinSpacing)? }
        else { params.get(Params::MinSpacing)?.as_float_slider()?.value() };
    Ok(resolve(automatic, legacy))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_spacing_keeps_the_original_conversion_exactly() {
        for percent in [0.0, 0.5, 0.013, 4.0, 25.0] {
            assert_eq!(resolve(false,percent).to_bits(),(percent as f32 / 100.0).to_bits());
            assert_eq!(resolve(true,percent),0.0);
        }
    }
    #[test]
    fn automatic_compression_is_safe_at_every_supported_density() {
        for count in 1..=MAX_GUIDES {
            let original=GridArb::uniform(count,count);
            for delta in [-100.0,100.0] {
                let mut lines=original.column_lines.clone();
                let elastic=EgElasticParams { tension_radius:0.0,falloff:2,
                    elasticity_strength:1.0,min_spacing:resolve(true,0.5) };
                control_grid::drag(&mut lines,&original.column_pins,1.0,delta,&elastic).unwrap();
                assert_eq!(lines[0],0.0); assert_eq!(*lines.last().unwrap(),1.0);
                assert!(lines.windows(2).all(|pair|pair[1]>pair[0]));
                // Allows much stronger compression than the former 0.5% default.
                let gap=if delta<0.0 {lines[1]} else {1.0-lines[count]};
                assert!(gap>0.0 && gap<0.000002);
            }
        }
    }
}
