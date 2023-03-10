/**
 * 値オブジェクト(VO)
 */

use validator::{Validate, ValidationError};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Validate, Deserialize)]
pub struct EstimationRequestId {
    #[Validate(required)]
    id: i128
}
