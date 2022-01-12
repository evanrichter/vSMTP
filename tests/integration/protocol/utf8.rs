#[cfg(test)]
mod tests {
    use crate::integration::protocol::get_test_config;
    use vsmtp::test_helpers::test_receiver;
    use vsmtp::{
        config::server_config::ServerConfig, model::mail::Body, model::mail::MailContext,
        resolver::DataEndResolver, rules::address::Address, smtp::code::SMTPReplyCode,
    };

    macro_rules! test_lang {
        ($lang_code:expr) => {{
            struct T;

            #[async_trait::async_trait]
            impl DataEndResolver for T {
                async fn on_data_end(
                    &mut self,
                    _: &ServerConfig,
                    ctx: &MailContext,
                ) -> Result<SMTPReplyCode, std::io::Error> {
                    assert_eq!(ctx.envelop.helo, "foobar".to_string());
                    assert_eq!(ctx.envelop.mail_from.full(), "john@doe".to_string());
                    assert_eq!(
                        ctx.envelop.rcpt,
                        std::collections::HashSet::from([Address::new("aa@bb").unwrap()])
                    );
                    assert!(match &ctx.body {
                        Body::Raw(body) => body == include_str!($lang_code),
                        _ => false,
                    });

                    Ok(SMTPReplyCode::Code250)
                }
            }

            assert!(test_receiver(
                std::sync::Arc::new(tokio::sync::Mutex::new(T)),
                [
                    "HELO foobar\r\n",
                    "MAIL FROM:<john@doe>\r\n",
                    "RCPT TO:<aa@bb>\r\n",
                    "DATA\r\n",
                    include_str!($lang_code),
                    ".\r\n",
                    "QUIT\r\n",
                ]
                .concat()
                .as_bytes(),
                [
                    "220 test.server.com Service ready\r\n",
                    "250 Ok\r\n",
                    "250 Ok\r\n",
                    "250 Ok\r\n",
                    "354 Start mail input; end with <CRLF>.<CRLF>\r\n",
                    "250 Ok\r\n",
                    "221 Service closing transmission channel\r\n",
                ]
                .concat()
                .as_bytes(),
                get_test_config()
            )
            .await
            .is_ok());
        }};
    }

    #[tokio::test]
    async fn test_receiver_utf8_zh() {
        test_lang!("mail/zh.txt");
    }

    #[tokio::test]
    async fn test_receiver_utf8_el() {
        test_lang!("mail/el.txt");
    }

    #[tokio::test]
    async fn test_receiver_utf8_ar() {
        test_lang!("mail/ar.txt");
    }

    #[tokio::test]
    async fn test_receiver_utf8_ko() {
        test_lang!("mail/ko.txt");
    }
}
