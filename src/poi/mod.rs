//! This module generate different points of interest on map like
//! * Starting point [AreaStartingPosition]
//! * Exit point [DistantExit]
//! 

mod cull_unreachable;
mod distant_exit;
mod starting_point;

pub use starting_point::{AreaStartingPosition, XStart, YStart};
pub use distant_exit::DistantExit;
pub use cull_unreachable::CullUnreachable;