use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::FromIterator,
};

#[derive(Debug)]
pub struct Log<'a> {
    pub request_type: &'a str, // http, https, h2
    pub close_time: &'a str,   // Timestamp in ISO 8601 format
    pub elb_identifier: &'a str,
    pub client: &'a str,                   // IP:port
    pub target: &'a str,                   // IP:port
    pub request_processing_time: &'a str, // in seconds, with millisecond precision for load balancer -> target. Can be -1 if the connection closed
    pub target_processing_time: &'a str, // in seconds, with millisecond precision for target -> load balancer. Can be -1 if the connection closed
    pub response_processing_time: &'a str, // in seconds, with millisecond precision for load balancer -> client. Can be -1 if the connection closed
    pub elb_status_code: &'a str,
    pub target_status_code: &'a str,
    pub received_bytes: &'a str, // The size of the request, in bytes.
    pub sent_bytes: &'a str,     // The size of the response, in bytes.
    pub request_method: &'a str, // HTTP method
    pub request_url: &'a str,    // protocol://host:port/uri
    pub request_http_version: &'a str, // HTTP version
    pub user_agent: &'a str,     // enclosed in double quotes.
    pub ssl_cipher: &'a str,
    pub ssl_protocol: &'a str,
    pub target_group_arn: &'a str,
    pub x_amazn_trace_id: &'a str,         // enclosed in double quotes.
    pub sni_domain_name: &'a str,          // enclosed in double quotes.
    pub cert_arn: &'a str,                 // enclosed in double quotes.
    pub matched_rule_priority: &'a str,    // a value from 1 to 50,000, default rule is 0.
    pub request_creation_time: &'a str,    // ISO 8601 format.
    pub actions_executed: &'a str,         // enclosed in double quotes
    pub redirect_url: &'a str,             // enclosed in double quotes
    pub error_reason: &'a str,             //enclosed in double quotes
    pub targets_list: &'a str,             //enclosed in double quotes
    pub targets_status_code_list: &'a str, //enclosed in double quotes
    pub classification: &'a str,           //enclosed in double quotes
    pub classification_reason: &'a str,    //enclosed in double quotes
}

#[derive(Debug)]
pub struct UrlCount {
    pub url: String,
    pub count: i64,
}
impl PartialEq for UrlCount {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}
impl Eq for UrlCount {}
impl Hash for UrlCount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}
impl Ord for UrlCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}
impl PartialOrd for UrlCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> FromIterator<&'a str> for Log<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = iter.into_iter();
        Self {
            request_type: iter.next().unwrap(),
            close_time: iter.next().unwrap(),
            elb_identifier: iter.next().unwrap(),
            client: iter.next().unwrap(),
            target: iter.next().unwrap(),
            request_processing_time: iter.next().unwrap(),
            target_processing_time: iter.next().unwrap(),
            response_processing_time: iter.next().unwrap(),
            elb_status_code: iter.next().unwrap(),
            target_status_code: iter.next().unwrap(),
            received_bytes: iter.next().unwrap(),
            sent_bytes: iter.next().unwrap(),
            request_method: iter.next().unwrap(),
            request_url: iter.next().unwrap(),
            request_http_version: iter.next().unwrap(),
            user_agent: iter.next().unwrap(),
            ssl_cipher: iter.next().unwrap(),
            ssl_protocol: iter.next().unwrap(),
            target_group_arn: iter.next().unwrap(),
            x_amazn_trace_id: iter.next().unwrap(),
            sni_domain_name: iter.next().unwrap(),
            cert_arn: iter.next().unwrap(),
            matched_rule_priority: iter.next().unwrap(),
            request_creation_time: iter.next().unwrap(),
            actions_executed: iter.next().unwrap(),
            redirect_url: iter.next().unwrap(),
            error_reason: iter.next().unwrap(),
            targets_list: iter.next().unwrap(),
            targets_status_code_list: iter.next().unwrap(),
            classification: iter.next().unwrap(),
            classification_reason: iter.next().unwrap(),
        }
    }
}
