#![allow(dead_code)]

//! HTTP status code phrases
//!
//! This module contains constants for HTTP status code phrases.
//! Generated from: https://raw.githubusercontent.com/prettymuchbryce/http-status-codes/refs/heads/master/codes.json

/// The request has been received but not yet acted upon.
pub const ACCEPTED: &str = "Accepted";

/// The server, while working as a gateway to get a response needed to handle the request, got an invalid response.
pub const BAD_GATEWAY: &str = "Bad Gateway";

/// The server could not understand the request due to invalid syntax.
pub const BAD_REQUEST: &str = "Bad Request";

/// The request conflicts with the current state of the server.
pub const CONFLICT: &str = "Conflict";

/// This interim response indicates that everything so far is OK and that the client should continue with the request.
pub const CONTINUE: &str = "Continue";

/// The request has succeeded and a new resource has been created as a result of it.
pub const CREATED: &str = "Created";

/// The expectation indicated by the Expect request header field can't be met by the server.
pub const EXPECTATION_FAILED: &str = "Expectation Failed";

/// The request failed due to failure of a previous request.
pub const FAILED_DEPENDENCY: &str = "Failed Dependency";

/// The client does not have access rights to the content.
pub const FORBIDDEN: &str = "Forbidden";

/// The server, while working as a gateway, cannot get a response in time.
pub const GATEWAY_TIMEOUT: &str = "Gateway Timeout";

/// The requested content has been permanently deleted from server.
pub const GONE: &str = "Gone";

/// The HTTP version used in the request is not supported by the server.
pub const HTTP_VERSION_NOT_SUPPORTED: &str = "HTTP Version Not Supported";

/// Any attempt to brew coffee with a teapot should result in this error.
pub const IM_A_TEAPOT: &str = "I'm a teapot";

/// The method could not be performed on the resource because the server is unable to store the representation needed to successfully complete the request.
pub const INSUFFICIENT_STORAGE: &str = "Insufficient Storage";

/// The server encountered an unexpected condition that prevented it from fulfilling the request.
pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// The server rejected the request because the Content-Length header field is not defined and the server requires it.
pub const LENGTH_REQUIRED: &str = "Length Required";

/// The resource that is being accessed is locked.
pub const LOCKED: &str = "Locked";

/// The request method is known by the server but has been disabled and cannot be used.
pub const METHOD_NOT_ALLOWED: &str = "Method Not Allowed";

/// The URI of requested resource has been changed. Probably, new URI would be given in the response.
pub const MOVED_PERMANENTLY: &str = "Moved Permanently";

/// The URI of requested resource has been changed temporarily.
pub const MOVED_TEMPORARILY: &str = "Moved Temporarily";

/// A Multi-Status response conveys information about multiple resources in situations where multiple status codes might be appropriate.
pub const MULTI_STATUS: &str = "Multi-Status";

/// The request has more than one possible response.
pub const MULTIPLE_CHOICES: &str = "Multiple Choices";

/// The 511 status code indicates that the client needs to authenticate to gain network access.
pub const NETWORK_AUTHENTICATION_REQUIRED: &str = "Network Authentication Required";

/// There is no content to send for this request, but the headers may be useful.
pub const NO_CONTENT: &str = "No Content";

/// The returned meta-information set is not exact set as available from the origin server.
pub const NON_AUTHORITATIVE_INFORMATION: &str = "Non-Authoritative Information";

/// The web server, after performing server-driven content negotiation, doesn't find any content following the criteria given by the user agent.
pub const NOT_ACCEPTABLE: &str = "Not Acceptable";

/// The server cannot find the requested resource.
pub const NOT_FOUND: &str = "Not Found";

/// The request method is not supported by the server and cannot be handled.
pub const NOT_IMPLEMENTED: &str = "Not Implemented";

/// This is used for caching purposes. It tells the client that the response has not been modified.
pub const NOT_MODIFIED: &str = "Not Modified";

/// The request has succeeded.
pub const OK: &str = "OK";

/// This response code is used because of range header sent by the client to separate download into multiple streams.
pub const PARTIAL_CONTENT: &str = "Partial Content";

/// This response code is reserved for future use.
pub const PAYMENT_REQUIRED: &str = "Payment Required";

/// The resource is now permanently located at another URI.
pub const PERMANENT_REDIRECT: &str = "Permanent Redirect";

/// The client has indicated preconditions in its headers which the server does not meet.
pub const PRECONDITION_FAILED: &str = "Precondition Failed";

/// The origin server requires the request to be conditional.
pub const PRECONDITION_REQUIRED: &str = "Precondition Required";

/// The server has received and is processing the request, but no response is available yet.
pub const PROCESSING: &str = "Processing";

/// The server is likely to send a final response with the header fields included in the informational response.
pub const EARLY_HINTS: &str = "Early Hints";

/// The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol.
pub const UPGRADE_REQUIRED: &str = "Upgrade Required";

/// This is similar to 401 but authentication is needed to be done by a proxy.
pub const PROXY_AUTHENTICATION_REQUIRED: &str = "Proxy Authentication Required";

/// The server is unwilling to process the request because its header fields are too large.
pub const REQUEST_HEADER_FIELDS_TOO_LARGE: &str = "Request Header Fields Too Large";

/// This response is sent on an idle connection by some servers.
pub const REQUEST_TIMEOUT: &str = "Request Timeout";

/// Request entity is larger than limits defined by server.
pub const REQUEST_TOO_LONG: &str = "Request Entity Too Large";

/// The URI requested by the client is longer than the server is willing to interpret.
pub const REQUEST_URI_TOO_LONG: &str = "Request-URI Too Long";

/// The range specified by the Range header field in the request can't be fulfilled.
pub const REQUESTED_RANGE_NOT_SATISFIABLE: &str = "Requested Range Not Satisfiable";

/// This response code is sent after accomplishing request to tell user agent reset document view.
pub const RESET_CONTENT: &str = "Reset Content";

/// Server sent this response to direct the client to get the requested resource at another URI with a GET request.
pub const SEE_OTHER: &str = "See Other";

/// The server is not ready to handle the request.
pub const SERVICE_UNAVAILABLE: &str = "Service Unavailable";

/// This code is sent in response to an Upgrade request header by the client.
pub const SWITCHING_PROTOCOLS: &str = "Switching Protocols";

/// Server sent this response to direct the client to get the requested resource at another URI with the same method.
pub const TEMPORARY_REDIRECT: &str = "Temporary Redirect";

/// The user has sent too many requests in a given amount of time ("rate limiting").
pub const TOO_MANY_REQUESTS: &str = "Too Many Requests";

/// The client must authenticate itself to get the requested response.
pub const UNAUTHORIZED: &str = "Unauthorized";

/// The user-agent requested a resource that cannot legally be provided.
pub const UNAVAILABLE_FOR_LEGAL_REASONS: &str = "Unavailable For Legal Reasons";

/// The request was well-formed but was unable to be followed due to semantic errors.
pub const UNPROCESSABLE_ENTITY: &str = "Unprocessable Entity";

/// The media format of the requested data is not supported by the server.
pub const UNSUPPORTED_MEDIA_TYPE: &str = "Unsupported Media Type";

/// The server is not able to produce a response for the combination of scheme and authority that are included in the request URI.
pub const MISDIRECTED_REQUEST: &str = "Misdirected Request";
