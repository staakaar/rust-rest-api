/**
 * 値オブジェクト(VO)
 */

use validator::{Validate, ValidationError};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Validate)]
pub struct EstimationRequestId {
    #[Validate(estimationRequestId)]
    id: i128
}
