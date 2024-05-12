pub mod cooking;
mod dsl;

#[cfg(test)]
mod tests {
    use super::cooking::*;

    #[test]
    fn make_curry() {
        let recipe = Recipe::new(
            prepare("onion".to_string(), Amount::Pcs(1.))
                .cuts("dice")
                .stakes(3.)
                .joins(
                    prepare("beef".to_string(), Amount::Pcs(1.))
                        .cuts("block")
                        .stakes(3.),
                )
                .joins(prepare("currot".to_string(), Amount::Pcs(1.)).cuts("block"))
                .joins(prepare("potato".to_string(), Amount::Pcs(1.)).cuts("block"))
                .stakes(3.)
                .joins(prepare("water".to_string(), Amount::MilliIllter(1000.)))
                .stew(15.)
                .joins(prepare("loux".to_string(), Amount::Pcs(1.)))
                .stew(3.),
            "curry".to_string(),
        );

        println!("{:#?}", recipe.instraction());

        for i in recipe.needed() {
            println!("{}", i);
        }
    }
}
