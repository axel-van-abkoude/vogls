use crate::{GlobalContext};

/// An optimization pass that merges basic blocks with the same WatchCondition.
///
/// The optimization pass merges basic blocks based on the intersections of the
/// watch conditions. We visit each combination of basic blocks with a watch terminator.
/// If one set of conditions contains or is equal the other we merge these.
/// The intersecting conditions can now be removed from the basic block with the most conditions.
/// With these rules we only remove a block, reduce the amount of conditions or keep them the same
/// (it should always terminate, and should never explode).
/// We can duplicate instructions (TODO, does this bite us?)
/// TODO: reason about if we always get the optimal solution (Church-Rosser)
///
/// The pass does this in the following way:
/// a <- Filter all basic blocks with watch terminators
/// x <- Pick first watch terminator
/// y <- for all basic blocks after x
///  if conditions of x == y -> merge y into x & remove y
///  if conditions of x are a subset of y -> merge x into y && prune y with x
///  if conditions of y are a subset of x -> merge y into x && prune x with y
///  if conditions if x and y intersect -> do nothing (we avoid making 3 basic blocks from 2)
///  if conditions are disjoint -> do nothing (nothing to merge)
/// pick next x
///
/// merge a into b:
/// combine the instructions (TODO ssa?)
/// combine temporal region (TODO references to both TR)
/// leave terminator of b
///
/// prune a with b:
/// remove all conditions of b from a
///
/// remove y:
/// remove the basic block from the search space


// pub struct BasicBlock {
//     pub instrs: Vec<Instruction>,
//     pub region: TemporalRegionKey,
//     pub terminator: BasicBlockTerminator,
// }

// "watch"
//   BasicBlockTerminator::Watch(
//    TemporalRegionKey := tr.entry() -> bb | Self::from_entry(bb) -> tr
//    Vec<WatchCondition> :=
//   )

// pub struct WatchCondition {
//     pub signal: SignalKey,
//     pub edge: WatchEdge,
//     pub offset: u32,
// }

// SlotMap: https://docs.rs/slotmap/latest/slotmap/#macros
// TemporalRegionKey -> tr.from_entry()
pub fn merge_watch_conditions(
    gl: &mut GlobalContext,
) {
    todo!("")
}
