use serde::Serialze;

#[derive(Serialize)]
pub struct CreatePromptRequest {
    /// Email address or phone number (E.164 format) of the user to request an approval from.
    user: String,
    /// Body of the approval request to show the user.
    body: String,
    /// The title of the approval request, empty by default.
    title: Option<String,
    /// Text displayed in 'Approve' button. Defaults to 'Approve'.
    approve_text: Option<String>,
    /// Text displayed in 'Reject' button. Defaults to 'Reject'.
    reject_text: Option<String>,
    /// Whether to wait for the user to respond (waits up to 10 minutes). Default false.
    long_poll: Option<bool>,
    /// Number of seconds until this request can no longer be answered.
    expires_in: Option<u64>,

    metadata: Option<PromptMetadata>,
}

#[derive(Serialize)]
pub struct PromptMetadata {

}
