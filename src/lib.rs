pub mod cooking;
mod dsl;

#[cfg(test)]
mod tests {
    use super::cooking::*;

    #[test]
    fn make_curry() {
        let recipe = Recipe::new(
            || {
                let beef = Amount::Gram(100.).of("beef".to_string()).cuts("block");
                let onion = Amount::Pcs(0.5).of("onion".to_string()).cuts("dice");
                let potato = Amount::Pcs(0.5).of("potato".to_string()).cuts("block");
                let currot = Amount::Pcs(0.25).of("currot".to_string()).cuts("block");
                let water = Amount::MilliLitter(400.).of("water".to_string());
                let loux = Amount::Pcs(1.).of("loux".to_string());
                let oil = Amount::Tsp(1.).of("oil".to_string());

                beef.joins(onion)
                    .joins(potato)
                    .joins(currot)
                    .stakes_with(oil, 1.)
                    .stew_with(water, 15.)
                    .stew_with(loux, 10.)
            },
            "curry".to_string(),
        );

        println!("{:#?}", recipe.parse());

        for i in recipe.needed() {
            println!("{:?}", i);
        }
    }
}
