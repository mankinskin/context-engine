use std::ops::Index;

use crate::{
    path::{
        accessors::child::RootedLeafToken,
        structs::rooted::root::PathRoot,
    },
    *,
};

pub trait CalcWidth: PathWidth {
    fn calc_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        self.path_width(&trav)
    }
}
impl<T: PathWidth> CalcWidth for T {}

//impl<Role: PathRole, Root: PathRoot> PathWidth for RootedRolePath<Role, Root>
//where
//    Self: RootedLeafToken<Role>,
//{
//    fn path_width<G: HasGraph>(
//        &self,
//        trav: G,
//    ) -> usize {
//        self.calc_offset(&trav) + self.rooted_leaf_token(&trav).width()
//    }
//}

pub trait PathWidth: CalcOffset + RootedPath {
    fn path_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize;
}
impl PathWidth for RootedRangePath<IndexRoot> {
    fn path_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        if self.role_root_child_index::<Start>()
            != self.role_root_child_index::<End>()
        {
            self.calc_offset(&trav)
                + self.role_rooted_leaf_token::<Start, _>(&trav).width()
                + self.role_rooted_leaf_token::<End, _>(&trav).width()
        } else {
            0
        }
    }
}

impl PathWidth for RootedRangePath<Pattern> {
    fn path_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        if self.role_root_child_index::<Start>()
            != self.role_root_child_index::<End>()
        {
            self.calc_offset(&trav)
                + self.role_rooted_leaf_token::<Start, _>(&trav).width()
                + self.role_rooted_leaf_token::<End, _>(&trav).width()
        } else {
            0
        }
    }
}

impl PathWidth for RootedRolePath<End, Pattern> {
    fn path_width<G: HasGraph>(
        &self,
        trav: G,
    ) -> usize {
        if self.role_root_child_index::<Start>()
            != self.role_root_child_index::<End>()
        {
            self.calc_offset(&trav)
                + self.role_rooted_leaf_token::<End, _>(&trav).width()
        } else {
            0
        }
    }
}
