use crate::{
    path::{
        accessors::child::{
            LeafToken,
            RootedLeafToken,
        },
        structs::rooted::root::PathRoot,
    },
    *,
};
use auto_impl::auto_impl;
use derive_more::{
    Deref,
    DerefMut,
};
use std::borrow::Borrow;

pub trait CalcWidth: CalcOffset + RootedPath {
    fn calc_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize;
}
impl<Role: PathRole, Root: PathRoot> CalcOffset for RootedRolePath<Role, Root> {
    fn calc_offset<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        self.role_path.calc_offset(trav)
    }
}

impl<Role: PathRole, Root: PathRoot> CalcWidth for RootedRolePath<Role, Root>
where
    Self: RootedLeafToken<Role>,
{
    fn calc_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        self.calc_offset(&trav) + self.rooted_leaf_token(&trav).width()
    }
}
