#[allow(non_snake_case)]
pub mod Model {
    use async_graphql::{*};
    use crate::settings::{self};

    pub struct User;
    #[Object]
    impl User {
        async fn me(&self) -> String {
            String::from("me")
        }
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize, SimpleObject)]
    struct JWT {
        access_token: String,
        expires_in: i32,
        refresh_expires_in: i32,
        refresh_token: String,
        token_type: String,
        #[serde(rename(deserialize = "not-before-policy"))]
        notbeforepolicy: i32,
        session_state: String,
        scope: String,
    }
    pub struct Mutation;

    #[Object]
    impl Mutation {
        async fn refresh<'ctx>(&self,ctx: &Context<'ctx>, refreshtoken: String) -> Box<JWT> {
            let cfg = ctx.data_unchecked::<settings::Settings>();

            println!("Config: {:?}", cfg.to_owned().server.port);
            let mut params = std::collections::HashMap::new();
            params.insert("client_id", cfg.to_owned().security.client_id);
            params.insert("refresh_token", refreshtoken);            
            params.insert("grant_type", "refresh_token".to_string());
           
            let client = reqwest::Client::new();
            let res = client
                .post(cfg.to_owned().security.token_url)
                .form(&params)
                .send()
                .await;

            let  foo = res.unwrap().text().await.unwrap();
            let Token: JWT = serde_json::from_str(&foo).unwrap();

            Box::new(Token)
        }

        async fn login<'ctx>(&self,ctx: &Context<'ctx>, username: String, #[graphql(secret)] password: String) -> Box<JWT> {
            let cfg = ctx.data_unchecked::<settings::Settings>();
            let mut params = std::collections::HashMap::new();
            params.insert("client_id", cfg.to_owned().security.client_id);
            params.insert("client_secret", cfg.to_owned().security.client_secret);
            params.insert("username", username.to_string());
            params.insert("password", password.to_string());
            params.insert("grant_type", "password".to_string());

            let client = reqwest::Client::new();
            let res = client
                .post(cfg.to_owned().security.token_url)
                .form(&params)
                .send()
                .await;

            let foo = res.unwrap().text().await.unwrap();
            let Token: JWT = serde_json::from_str(&foo).unwrap();

            Box::new(Token)
        }
    }
}
