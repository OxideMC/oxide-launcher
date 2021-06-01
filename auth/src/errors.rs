use error_chain::error_chain;

error_chain! {
    foreign_links {
        Curl(::curl::Error);
        Serde(::serde_json::Error);
        FromUtf8(::std::string::FromUtf8Error);
    }
}
