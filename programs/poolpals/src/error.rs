use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Not authorized to initialize program")]
    InvalidAuthorizedInitiator,
    #[msg("Bounty must be enough to mark account rent-exempt")]
    UnauthorizedInitiator,
}
