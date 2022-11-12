//! This module is automatically generated. Please edit unfolded_macro_gen.lua
//! instead. Or better yet, replace these with procmacros. (TODO)

#[macro_export]
macro_rules! ecs_iter_accessors {
    ($mode1:ident $comp1:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27),
        $crate::ecs_iter_accessor!($mode28 $comp28)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27),
        $crate::ecs_iter_accessor!($mode28 $comp28),
        $crate::ecs_iter_accessor!($mode29 $comp29)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27),
        $crate::ecs_iter_accessor!($mode28 $comp28),
        $crate::ecs_iter_accessor!($mode29 $comp29),
        $crate::ecs_iter_accessor!($mode30 $comp30)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27),
        $crate::ecs_iter_accessor!($mode28 $comp28),
        $crate::ecs_iter_accessor!($mode29 $comp29),
        $crate::ecs_iter_accessor!($mode30 $comp30),
        $crate::ecs_iter_accessor!($mode31 $comp31)
    ]};
    ($mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty, $mode32:ident $comp32:ty) => {[
        $crate::ecs_iter_accessor!($mode1 $comp1),
        $crate::ecs_iter_accessor!($mode2 $comp2),
        $crate::ecs_iter_accessor!($mode3 $comp3),
        $crate::ecs_iter_accessor!($mode4 $comp4),
        $crate::ecs_iter_accessor!($mode5 $comp5),
        $crate::ecs_iter_accessor!($mode6 $comp6),
        $crate::ecs_iter_accessor!($mode7 $comp7),
        $crate::ecs_iter_accessor!($mode8 $comp8),
        $crate::ecs_iter_accessor!($mode9 $comp9),
        $crate::ecs_iter_accessor!($mode10 $comp10),
        $crate::ecs_iter_accessor!($mode11 $comp11),
        $crate::ecs_iter_accessor!($mode12 $comp12),
        $crate::ecs_iter_accessor!($mode13 $comp13),
        $crate::ecs_iter_accessor!($mode14 $comp14),
        $crate::ecs_iter_accessor!($mode15 $comp15),
        $crate::ecs_iter_accessor!($mode16 $comp16),
        $crate::ecs_iter_accessor!($mode17 $comp17),
        $crate::ecs_iter_accessor!($mode18 $comp18),
        $crate::ecs_iter_accessor!($mode19 $comp19),
        $crate::ecs_iter_accessor!($mode20 $comp20),
        $crate::ecs_iter_accessor!($mode21 $comp21),
        $crate::ecs_iter_accessor!($mode22 $comp22),
        $crate::ecs_iter_accessor!($mode23 $comp23),
        $crate::ecs_iter_accessor!($mode24 $comp24),
        $crate::ecs_iter_accessor!($mode25 $comp25),
        $crate::ecs_iter_accessor!($mode26 $comp26),
        $crate::ecs_iter_accessor!($mode27 $comp27),
        $crate::ecs_iter_accessor!($mode28 $comp28),
        $crate::ecs_iter_accessor!($mode29 $comp29),
        $crate::ecs_iter_accessor!($mode30 $comp30),
        $crate::ecs_iter_accessor!($mode31 $comp31),
        $crate::ecs_iter_accessor!($mode32 $comp32)
    ]};
}

#[macro_export]
macro_rules! ecs_iterated_unfold {
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode31 $comp31)
        )}};
    ($eid:expr, $array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty, $mode32:ident $comp32:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $eid,
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode31 $comp31),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode32 $comp32)
        )}};
}

#[macro_export]
macro_rules! ecs_gotten_unfold {
    ($array:expr, $mode1:ident $comp1:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode31 $comp31)
        )}};
    ($array:expr, $mode1:ident $comp1:ty, $mode2:ident $comp2:ty, $mode3:ident $comp3:ty, $mode4:ident $comp4:ty, $mode5:ident $comp5:ty, $mode6:ident $comp6:ty, $mode7:ident $comp7:ty, $mode8:ident $comp8:ty, $mode9:ident $comp9:ty, $mode10:ident $comp10:ty, $mode11:ident $comp11:ty, $mode12:ident $comp12:ty, $mode13:ident $comp13:ty, $mode14:ident $comp14:ty, $mode15:ident $comp15:ty, $mode16:ident $comp16:ty, $mode17:ident $comp17:ty, $mode18:ident $comp18:ty, $mode19:ident $comp19:ty, $mode20:ident $comp20:ty, $mode21:ident $comp21:ty, $mode22:ident $comp22:ty, $mode23:ident $comp23:ty, $mode24:ident $comp24:ty, $mode25:ident $comp25:ty, $mode26:ident $comp26:ty, $mode27:ident $comp27:ty, $mode28:ident $comp28:ty, $mode29:ident $comp29:ty, $mode30:ident $comp30:ty, $mode31:ident $comp31:ty, $mode32:ident $comp32:ty) => {unsafe {
        let mut iter = $array.into_iter();
        (
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode1 $comp1),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode2 $comp2),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode3 $comp3),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode4 $comp4),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode5 $comp5),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode6 $comp6),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode7 $comp7),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode8 $comp8),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode9 $comp9),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode10 $comp10),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode11 $comp11),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode12 $comp12),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode13 $comp13),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode14 $comp14),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode15 $comp15),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode16 $comp16),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode17 $comp17),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode18 $comp18),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode19 $comp19),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode20 $comp20),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode21 $comp21),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode22 $comp22),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode23 $comp23),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode24 $comp24),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode25 $comp25),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode26 $comp26),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode27 $comp27),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode28 $comp28),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode29 $comp29),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode30 $comp30),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode31 $comp31),
            $crate::ecs_iter_iterated!(iter.next().unwrap(), $mode32 $comp32)
        )}};
}
