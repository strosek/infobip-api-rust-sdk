use validator::Validate;

use crate::model::email::SendRequestBody;

pub fn get_dummy_send_email_request_body() -> SendRequestBody {
    let mut request = SendRequestBody::new("some@company.com".to_string());
    request.from = Some("John Doe <john@company.com>".to_string());
    request.cc = Some("one@company.com,two@company.com".to_string());
    request.bcc = Some("three@company.com,four@some.com".to_string());
    request.subject = Some("Some subject".to_string());
    request.text = Some("Some text".to_string());
    request.html = Some("<p>Some text</p>".to_string());
    request.amp_html = Some("<p>Some text</p>".to_string());
    request.template_id = Some(2);
    request.attachment = Some("../../../tests/image.png".to_string());
    request.inline_image = Some("../../../tests/image.png".to_string());
    request.notify_url = Some("https://some.url".to_string());
    request.intermediate_report = Some(true);
    request.notify_content_type = Some("application/json".to_string());
    request.callback_data = Some("some data".to_string());
    request.track = Some(true);
    request.track_clicks = Some(true);
    request.track_opens = Some(true);
    request.tracking_url = Some("https://some.url".to_string());
    request.bulk_id = Some("some-bulk-id".to_string());
    request.message_id = Some("some-message-id".to_string());
    request.reply_to = Some("some-reply-to@company.com".to_string());
    request.default_placeholders = Some(r#"defaultPlaceholders={"ph1": "Success"}"#.to_string());
    request.preserve_recipients = Some(true);
    request.send_at = Some("2020-01-01 00:00:00".to_string());
    request.landing_page_placeholders =
        Some(r#"landingPagePlaceholders={"ph1": "Success"}"#.to_string());
    request.landing_page_id = Some("some-landing-page-id".to_string());

    request
}

#[test]
fn test_send_request_valid() {
    let request_body = SendRequestBody::new("someone@company.com".to_string());

    assert!(request_body.validate().is_ok());
}

#[test]
fn test_send_request_valid_full() {
    let request_body = get_dummy_send_email_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn test_send_request_body_long_subject() {
    let mut request_body = get_dummy_send_email_request_body();
    request_body.subject = Some("S".repeat(151));

    assert!(request_body.validate().is_err());
}

#[test]
fn tets_send_request_body_long_callback_data() {
    let mut request_body = get_dummy_send_email_request_body();
    request_body.callback_data = Some("C".repeat(4001));

    assert!(request_body.validate().is_err());
}
