pub trait State {
    type Id;
}

pub struct New;

impl State for New {
    type Id = ();
}

pub struct Saved;

impl State for Saved {
    type Id = crate::app::Id;
}
