use crate::{
    config::server_config::ServerConfig,
    receiver::test_helpers::test_receiver,
    resolver::Resolver,
    rules::address::Address,
    smtp::mail::{Body, MailContext},
};

macro_rules! test_lang {
    ($lang_code:expr) => {{
        struct T;

        #[async_trait::async_trait]
        impl Resolver for T {
            async fn deliver(&mut self, _: &ServerConfig, ctx: &MailContext) -> anyhow::Result<()> {
                assert_eq!(ctx.envelop.helo, "foobar".to_string());
                assert_eq!(ctx.envelop.mail_from.full(), "john@doe".to_string());
                assert_eq!(
                    ctx.envelop.rcpt,
                    std::collections::HashSet::from([Address::new("aa@bb").unwrap()])
                );
                assert!(match &ctx.body {
                    Body::Parsed(mail) => {
                        let (headers, body) = mail.to_raw();
                        format!("{headers}\n\n{body}\n").as_str() == include_str!($lang_code)
                    }
                    _ => false,
                });

                Ok(())
            }
        }

        assert!(test_receiver(
            "127.0.0.1:0",
            T,
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
            std::sync::Arc::new(
                ServerConfig::builder()
                    .with_server_default_port("test.server.com")
                    .without_log()
                    .without_smtps()
                    .with_default_smtp()
                    .with_delivery("./tmp/delivery", crate::collection! {})
                    .with_rules("./tmp/nothing")
                    .with_default_reply_codes()
                    .build(),
            )
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
