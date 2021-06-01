/*
 * Copyright (c) 2021 TheOddGarlic <umutinanerdogan62@gmail.com>
 * Licensed under the Open Software License version 3.0
 */

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Curl(::curl::Error);
        Serde(::serde_json::Error);
        FromUtf8(::std::string::FromUtf8Error);
    }
}
