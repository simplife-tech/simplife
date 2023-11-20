use akasha::error::Error;

pub const NO_LEDGER: Error = Error::ServiceError(20000, "操作失败");
pub const DELETE_LEDGER_NOT_SAME_UID: Error = Error::ServiceError(20001, "操作失败");
