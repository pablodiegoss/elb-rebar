use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Log {
    pub request_type: String, // http, https, h2
    pub close_time: String,   // Timestamp in ISO 8601 format
    pub elb_identifier: String,
    pub client: String,                // IP:port
    pub target: String,                // IP:port
    pub request_processing_time: f32, // in seconds, with millisecond precision for load balancer -> target. Can be -1 if the connection closed
    pub target_processing_time: f32, // in seconds, with millisecond precision for target -> load balancer. Can be -1 if the connection closed
    pub response_processing_time: f32, // in seconds, with millisecond precision for load balancer -> client. Can be -1 if the connection closed
    pub elb_status_code: i32,
    pub target_status_code: i32,
    pub received_bytes: i64,    // The size of the request, in bytes.
    pub sent_bytes: i64,        // The size of the response, in bytes.
    pub request_string: String, //double quotes. formatted as HTTP method + protocol://host:port/uri + HTTP version eg. "POST https://example.com.br:443/url1/ HTTP/2.0"
    pub user_agent: String,     // enclosed in double quotes.
    pub ssl_cipher: String,
    pub ssl_protocol: String,
    pub target_group_arn: String,
    pub x_amazn_trace_id: String,         // enclosed in double quotes.
    pub sni_domain_name: String,          // enclosed in double quotes.
    pub cert_arn: String,                 // enclosed in double quotes.
    pub matched_rule_priority: i32,       // a value from 1 to 50,000, default rule is 0.
    pub request_creation_time: String,    // ISO 8601 format.
    pub actions_executed: String,         // enclosed in double quotes
    pub redirect_url: String,             // enclosed in double quotes
    pub error_reason: String,             //enclosed in double quotes
    pub targets_list: String,             //enclosed in double quotes
    pub targets_status_code_list: String, //enclosed in double quotes
    pub classification: String,           //enclosed in double quotes
    pub classification_reason: String,    //enclosed in double quotes
}

#[derive(Debug)]
pub struct UrlCount {
    pub url:String,
    pub count: i64
}
impl PartialEq for UrlCount{
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for UrlCount{}
impl Hash for UrlCount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}
impl Ord for UrlCount{
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}
impl PartialOrd for UrlCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn create_log(mut log_values: Vec<String>) -> Log {
    Log {
        request_type: log_values.remove(0),
        close_time: log_values.remove(0),
        elb_identifier: log_values.remove(0),
        client: log_values.remove(0),
        target: log_values.remove(0),
        request_processing_time: log_values.remove(0).parse::<f32>().unwrap(),
        target_processing_time: log_values.remove(0).parse::<f32>().unwrap(),
        response_processing_time: log_values.remove(0).parse::<f32>().unwrap(),
        elb_status_code: log_values.remove(0).parse::<i32>().unwrap(),
        target_status_code: log_values.remove(0).parse::<i32>().unwrap_or_else(|_err|-1),
        received_bytes: log_values.remove(0).parse::<i64>().unwrap(),
        sent_bytes: log_values.remove(0).parse::<i64>().unwrap(),
        request_string: log_values.remove(0),
        user_agent: log_values.remove(0),
        ssl_cipher: log_values.remove(0),
        ssl_protocol: log_values.remove(0),
        target_group_arn: log_values.remove(0),
        x_amazn_trace_id: log_values.remove(0),
        sni_domain_name: log_values.remove(0),
        cert_arn: log_values.remove(0),
        matched_rule_priority: log_values.remove(0).parse::<i32>().unwrap_or_else(|_err|-1),
        request_creation_time: log_values.remove(0),
        actions_executed: log_values.remove(0),
        redirect_url: log_values.remove(0),
        error_reason: log_values.remove(0),
        targets_list: log_values.remove(0),
        targets_status_code_list: log_values.remove(0),
        classification: log_values.remove(0),
        classification_reason: log_values.remove(0),
    }
}
