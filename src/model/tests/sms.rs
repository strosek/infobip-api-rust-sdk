use validator::Validate;

use crate::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

#[test]
fn test_sms_preview_request_body_valid() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.language_code = Some("ES".to_string());
    request_body.transliteration = Some("GREEK".to_string());

    assert!(request_body.validate().is_ok())
}

#[test]
fn test_sms_preview_request_body_invalid_language_code() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.language_code = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn test_sms_preview_request_body_invalid_transliteration() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.transliteration = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn test_get_delivery_reports_query_parameters_valid() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10);

    assert!(parameters.validate().is_ok())
}

#[test]
fn test_get_delivery_reports_query_parameters_big_limit() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10000);

    assert!(parameters.validate().is_err())
}

#[test]
fn test_send_request_body_valid() {
    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.text = Some(DUMMY_TEXT.to_string());

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok())
}

#[test]
fn test_send_request_body_no_messages() {
    let request_body = SendRequestBody::new(vec![]);

    assert!(request_body.validate().is_err())
}

#[test]
fn test_send_request_body_no_destination() {
    let message = Message::new(vec![]);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_no_destination_to() {
    let message = Message::new(vec![Destination::new("".to_string())]);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_no_principal_entity_id() {
    let mut regional = RegionalOptions::new();
    regional.india_dlt = Some(IndiaDlt::new("".to_string()));
    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_no_turkey_recipient_type() {
    let mut regional = RegionalOptions::new();
    regional.turkey_iys = Some(TurkeyIys::new("".to_string()));
    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_bad_turkey_recipient_type() {
    let mut regional = RegionalOptions::new();
    regional.turkey_iys = Some(TurkeyIys::new("BAD".to_string()));
    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_message_from_str() {
    let message: Message = serde_json::from_str(
        r#"
        {
          "destinations": [
            {
              "to": "41793026727"
            }
          ],
          "from": "InfoSMS",
          "text": "This is a sample message"
        }
    "#,
    )
    .unwrap();

    assert_eq!(message.text.unwrap(), "This is a sample message");
}

#[test]
fn test_send_request_body_zero_speed_limit_amount() {
    let message = Message::new(vec![Destination::new("123456789012".to_string())]);
    let mut request_body = SendRequestBody::new(vec![message]);
    request_body.sending_speed_limit = Some(SpeedLimit::new(0));

    assert!(request_body.validate().is_ok());
}

#[test]
fn test_send_request_body_speed_limit_time_unit() {
    let message = Message::new(vec![Destination::new("123456789012".to_string())]);
    let mut speed_limit = SpeedLimit::new(5);
    speed_limit.time_unit = Some(TimeUnit::DAY);

    let mut request_body = SendRequestBody::new(vec![message]);
    request_body.sending_speed_limit = Some(speed_limit);

    let serialized = serde_json::to_string(&request_body).unwrap();
    assert!(request_body.validate().is_ok());
    assert!(serialized.contains(r#""amount":5,"timeUnit":"DAY""#));
}

#[test]
fn test_send_request_body_with_delivery_time_window() {
    let delivery_time_window =
        DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY, DeliveryDay::TUESDAY]);

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    let serialized = serde_json::to_string(&request_body).unwrap();
    assert!(request_body.validate().is_ok());
    assert!(serialized.contains(r#""days":["MONDAY","TUESDAY"]"#));
}

#[test]
fn test_send_request_body_delivery_time_window_no_days() {
    let delivery_time_window = DeliveryTimeWindow::new(vec![]);

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_delivery_time_window_to_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.to = Some(DeliveryTime::new(23, 59));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok());
}

#[test]
fn test_send_request_body_delivery_time_window_bad_to_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.to = Some(DeliveryTime::new(24, 0));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_delivery_time_window_bad_to_minute() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.to = Some(DeliveryTime::new(23, 60));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_delivery_time_window_from_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.from = Some(DeliveryTime::new(23, 59));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok());
}

#[test]
fn test_send_request_body_delivery_time_window_bad_from_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.from = Some(DeliveryTime::new(24, 0));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn test_send_request_body_delivery_time_window_bad_from_minute() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::MONDAY]);
    delivery_time_window.from = Some(DeliveryTime::new(23, 60));

    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}
