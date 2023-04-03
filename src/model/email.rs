//! Models for calling Email endpoints.

use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendRequestBody {
    /// Email address with optional sender name. This field is required if `templateId` is not
    /// present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Email address of the recipient.
    #[validate(length(min = 1))]
    pub to: String,

    /// CC recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,

    /// BCC recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,

    /// Message subject. This field is required if `templateId` is not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 150))]
    pub subject: Option<String>,

    /// Body of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// HTML body of the message. If `html` and `text` fields are present, the `text` field will be
    /// ignored and `html` will be delivered as a message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    /// Amp HTML body of the message. If ampHtml is present, html is mandatory. Amp HTML is not
    /// supported by all the email clients. Please check this link for configuring gmail client
    /// `<https://developers.google.com/gmail/ampemail/>`
    pub amp_html: Option<String>,

    /// Template ID used for generating email content. The template is created over Infobip web
    /// interface. If `templateId` is present, then `html` and `text` values are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,

    /// File attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,

    /// Allows for inserting an image file inside the HTML code of the email by using
    /// `cid:FILENAME` instead of providing an external link to the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_image: Option<String>,

    /// The real-time Intermediate delivery report that will be sent on your callback server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_report: Option<bool>,

    /// The URL on your callback server on which the Delivery report will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,

    /// Preferred Delivery report content type. Can be `application/json` or `application/xml`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_content_type: Option<String>,

    /// Additional client data that will be sent on the notifyUrl.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4000))]
    pub callback_data: Option<String>,

    /// Enable or disable open and click tracking. Passing true will only enable tracking and the
    /// statistics would be visible in the web interface alone. This can be explicitly overridden
    /// by `trackClicks` and `trackOpens`. Default: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<bool>,

    /// This parameter enables or disables track click feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<bool>,

    /// This parameter enables or disables track opens feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,

    /// The URL on your callback server on which the open and click notifications will be sent.
    /// See Tracking Notifications for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub tracking_url: Option<String>,

    /// The ID uniquely identifies the sent email request. This filter will enable you to query
    /// delivery reports for all the messages using just one request. You will receive a `bulkId`
    /// in the response after sending an email request. If you don't set your own `bulkId`, unique
    /// ID will be generated by our system and returned in the API response. (Optional Field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// The ID that uniquely identifies the message sent to a recipient. (Optional Field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Email address to which recipients of the email can reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,

    /// General placeholder, given in a form of json example:
    /// `defaultPlaceholders={"ph1": "Success"}`, which will replace given key `{{ph1}}` with
    /// given value `Success` anywhere in the email (subject, text, html...). In case of more
    /// destinations in `To` field, this placeholder will resolve the same value for key `ph1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_placeholders: Option<String>,

    /// If set to `true`, the `to` recipients will see the list of all other recipients to get the
    /// email and the response will return only one `messageId`. Otherwise, each recipient will
    /// see just their own email and the response will return a unique `messageId` for each email
    /// recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_recipients: Option<bool>,

    /// To schedule message at a given time in future. Time provided should be in UTC in the
    /// following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,

    /// Personalize opt out landing page by inserting placeholders. Insert placeholder or tag while
    /// designing landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_placeholders: Option<String>,

    /// Opt out landing page which will be used and displayed once end user clicks the unsubscribe
    /// link. If not present default opt out landing page will be displayed. Create a landing page
    /// on IB’s portal and use the last 6 digits from URL to use that opt out page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_id: Option<String>,
}

impl SendRequestBody {
    pub fn new(to: &str) -> Self {
        SendRequestBody {
            from: None,
            to: to.into(),
            cc: None,
            bcc: None,
            subject: None,
            text: None,
            html: None,
            amp_html: None,
            template_id: None,
            attachment: None,
            inline_image: None,
            intermediate_report: None,
            notify_url: None,
            notify_content_type: None,
            callback_data: None,
            track: None,
            track_clicks: None,
            track_opens: None,
            tracking_url: None,
            bulk_id: None,
            message_id: None,
            reply_to: None,
            default_placeholders: None,
            preserve_recipients: None,
            send_at: None,
            landing_page_placeholders: None,
            landing_page_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentMessageDetails {
    /// The destination address of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// The ID that uniquely identifies a message response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Status group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,

    /// Status group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Status ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Status name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Status description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Action name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResponseBody {
    /// The ID that uniquely identifies a list of message responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// List of message response details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<SentMessageDetails>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetBulksQueryParameters {
    #[validate(length(min = 1))]
    pub bulk_id: String,
}

impl GetBulksQueryParameters {
    pub fn new(bulk_id: &str) -> Self {
        GetBulksQueryParameters { bulk_id: bulk_id.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBulksResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulks: Option<Vec<BulkInfo>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<u64>,
}

pub type RescheduleQueryParameters = GetBulksQueryParameters;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleRequestBody {
    #[validate(length(min = 1))]
    pub send_at: String,
}

impl RescheduleRequestBody {
    pub fn new(send_at: &str) -> Self {
        RescheduleRequestBody { send_at: send_at.into() }
    }
}

pub type RescheduleResponseBody = BulkInfo;

pub type GetScheduledStatusQueryParameters = GetBulksQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BulkStatus {
    PENDING,
    PAUSED,
    PROCESSING,
    CANCELED,
    FINISHED,
    FAILED,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BulkStatusInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BulkStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledStatusResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulks: Option<Vec<BulkStatusInfo>>,
}

pub type UpdateScheduledStatusQueryParameters = GetBulksQueryParameters;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScheduledStatusRequestBody {
    pub status: BulkStatus,
}

impl UpdateScheduledStatusRequestBody {
    pub fn new(status: BulkStatus) -> Self {
        UpdateScheduledStatusRequestBody { status }
    }
}

pub type UpdateScheduledStatusResponseBody = BulkStatusInfo;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryReportsQueryParameters {
    /// Bulk ID for which report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// The ID that uniquely identifies the sent email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Maximum number of reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GetDeliveryReportsQueryParameters {
    pub fn new() -> Self {
        GetDeliveryReportsQueryParameters {
            bulk_id: None,
            message_id: None,
            limit: None,
        }
    }
}

impl Default for GetDeliveryReportsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// Price per one email request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_message: Option<f32>,

    /// The currency in which the price is expressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// The ID that uniquely identifies bulks of request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// The ID that uniquely identifies the sent email request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// The recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Tells when the email was initiated. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,

    /// Tells when the email request was processed by Infobip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_at: Option<String>,

    /// Email request count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,

    /// Sent email price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Indicates whether the initiated email has been successfully sent, not sent, delivered,
    /// not delivered, waiting for delivery or any other possible status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ReportError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryReportsResponseBody {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Report>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsQueryParameters {
    /// The ID that uniquely identifies the sent email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// From email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// The recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Bulk ID that uniquely identifies the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Indicates whether the initiated email has been successfully sent, not sent, delivered,
    /// not delivered, waiting for delivery or any other possible status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_status: Option<String>,

    /// Tells when the email was initiated. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_since: Option<String>,

    /// Tells when the email request was processed by Infobip. Has the following format:
    /// `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_until: Option<String>,

    /// Maximum number of logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GetLogsQueryParameters {
    pub fn new() -> Self {
        GetLogsQueryParameters {
            message_id: None,
            from: None,
            to: None,
            bulk_id: None,
            general_status: None,
            sent_since: None,
            sent_until: None,
            limit: None,
        }
    }
}

impl Default for GetLogsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    /// The ID that uniquely identifies the sent email request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// The recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// From email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// The text from email body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Tells when the email was initiated. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,

    /// Tells when the email request was processed by Infobip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_at: Option<String>,

    /// Email request count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,

    /// Sent email price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Indicates whether the initiated email has been successfully sent, not sent, delivered,
    /// not delivered, waiting for delivery or any other possible status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    /// The ID that uniquely identifies the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResponseBody {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Log>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressRequestBody {
    /// Email address of the recipient.
    #[validate(length(min = 1))]
    to: String,
}

impl ValidateAddressRequestBody {
    pub fn new(to: &str) -> Self {
        ValidateAddressRequestBody { to: to.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateAddressResponseBody {
    /// Email address of the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Represents status of recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_mailbox: Option<String>,

    /// Represents syntax of recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_syntax: Option<bool>,

    /// Denotes catch all status of recipient email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_all: Option<bool>,

    /// Suggests alternate addresses that maybe valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_you_mean: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_based: Option<bool>,

    /// Reason is provided when validMailbox status is unknown.
    /// 1. INBOX_FULL - The user quota exceeded / The user inbox is full / The user doesn't accept
    /// any more requests.
    /// 2. UNEXPECTED_FAILURE - The mail Server returned a temporary error.
    /// 3. THROTTLED - The mail server is not allowing us momentarily because of too many requests.
    /// 4. TIMED_OUT - The Mail Server took a longer time to respond / there was a delay in the
    /// network.
    /// 5. TEMP_REJECTION - Mail server temporarily rejected.
    /// 6. UNABLE_TO_CONNECT - Unable to connect to the Mail Server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetDomainsQueryParameters {
    /// Maximum number of domains to be viewed per page. Default value is 10 with a maximum of 20 records per page.
    #[validate(range(min = 1, max = 20))]
    pub size: Option<i32>,

    /// Page number you want to see. Default is 0.
    #[validate(range(min = 1))]
    pub page: Option<i32>,
}

impl GetDomainsQueryParameters {
    pub fn new() -> Self {
        GetDomainsQueryParameters {
            size: None,
            page: None,
        }
    }
}

impl Default for GetDomainsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking {
    /// Indicates whether tracking of clicks is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<bool>,

    /// Indicates whether tracking of opens is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<bool>,

    /// Indicates whether tracking of unsubscribes is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    /// Type of the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,

    /// Name of the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Expected value to be set for the given record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,

    /// Boolean value representing if the record is verified or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    /// Id of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<i64>,

    /// Name of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,

    /// Activation status of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Tracking details of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<Tracking>,

    /// DNS records for the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<Vec<DnsRecord>>,

    /// Status if the domain is blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,

    /// Date the domain was created. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDomainsResponseBody {
    /// Pagination details like page number, page size, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paging: Option<Paging>,

    /// List of domains that belong to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Domain>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DkimKeyLength {
    #[serde(rename = "1024")]
    L1024 = 1024,
    #[serde(rename = "2048")]
    L2048 = 2048,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddDomainRequestBody {
    #[validate(length(min = 1))]
    pub domain_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_key_length: Option<DkimKeyLength>,
}

impl AddDomainRequestBody {
    pub fn new(domain_name: &str) -> Self {
        AddDomainRequestBody {
            domain_name: domain_name.into(),
            dkim_key_length: None,
        }
    }
}

pub type AddDomainResponseBody = Domain;

pub type GetDomainResponseBody = Domain;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTrackingRequestBody {
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub opens: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<bool>,
}

impl UpdateTrackingRequestBody {
    pub fn new() -> Self {
        UpdateTrackingRequestBody {
            opens: None,
            clicks: None,
            unsubscribe: None,
        }
    }
}

impl Default for UpdateTrackingRequestBody {
    fn default() -> Self {
        Self::new()
    }
}

pub type UpdateTrackingResponseBody = Domain;
