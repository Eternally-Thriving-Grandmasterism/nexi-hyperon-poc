use hyperon::{Space, Atom};
use crate::nexi_grounded::get_nexi_grounded_atoms;

pub fn nexi_mercy_space() -> Space {
    let mut space = Space::new();
    for atom in get_nexi_grounded_atoms() {
        space.add(atom);
    }
    space
}
