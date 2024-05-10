use crate::dsl::Process;

#[derive(Debug, Clone)]
pub enum Ingredients {
    TbsOf {},
    PcsOf {
        what: String,
        shapes: Shapes,
        many: f32,
    },
    GramOf {},
    CupOf {},
}

#[derive(Debug, Clone, Copy)]
pub enum Shapes {
    Whatever,
    Dice,
}

pub struct Prepare(Ingredients);

impl Prepare {
    pub fn psc_of(what: &str, many: f32) -> Prepare {
        Prepare(Ingredients::PcsOf {
            what: what.to_string(),
            shapes: Shapes::Whatever,
            many,
        })
    }
}

impl Process<Ingredients> for Prepare {
    fn acquires(&self) -> impl Iterator<Item = Ingredients> {
        vec![self.0.clone()].into_iter()
    }

    fn releases(&self) -> impl Iterator<Item = Ingredients> {
        self.acquires()
    }
}

pub fn chop(shape: Shapes) -> impl Fn(Ingredients) -> Ingredients {
    move |x: Ingredients| match x {
        Ingredients::PcsOf {
            shapes: _,
            what,
            many,
        } => Ingredients::PcsOf {
            shapes: shape,
            what,
            many,
        },
        _ => x,
    }
}
