pub mod cooking;
mod dsl;

#[cfg(test)]
mod tests {
    use super::cooking::*;

    #[test]
    fn make_curry() {
        let recipe = Recipe::new(
            Amount::Pcs(1.)
                .of("onion".to_string())
                .cuts("dice")
                .stakes(3.)
                .joins(
                    Amount::Pcs(1.)
                        .of("beef".to_string())
                        .cuts("block")
                        .stakes(3.),
                )
                .joins(prepare("currot".to_string(), Amount::Pcs(1.)).cuts("block"))
                .joins(prepare("potato".to_string(), Amount::Pcs(1.)).cuts("block"))
                .stakes(3.)
                .joins(Amount::MilliLitter(1000.).of("water".to_string()))
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
