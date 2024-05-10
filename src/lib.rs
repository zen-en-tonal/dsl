// mod block_diagram;
pub mod cooking;
pub mod dsl;

#[cfg(test)]
mod tests {
    use crate::cooking::*;
    use crate::dsl::*;

    #[test]
    fn make_curry() {
        let prepare_onions = Prepare::psc_of("Onion", 1.).then(chop(Shapes::Dice), "chop");
        let prepare_carrots = Prepare::psc_of("Carrot", 1.).then(chop(Shapes::Dice), "chop");
        let prepare_potato = Prepare::psc_of("Potato", 1.).then(chop(Shapes::Dice), "chop");

        let stake = |x: Ingredients| x;

        let recipe = prepare_onions
            .then(stake, "stake until get color brown")
            .add(prepare_carrots)
            .add(prepare_potato)
            .then(stake, "stake");

        let what_we_need: Vec<Ingredients> = recipe.acquires().collect();

        println!("{:#?}", what_we_need);
    }
}
