#[macro_use]
extern crate serde_derive;

pub mod github {
    extern crate hyper;
    extern crate hyper_native_tls;
    extern crate serde;
    extern crate serde_json;

    use self::hyper::header::UserAgent;

    #[derive(Serialize, Deserialize)]
    pub struct GithubUser {
        login: String,
        pub html_url: String,
        pub name: String,
        public_repos: u32,
        followers: u32,
    }

    static GITHUB_ENDPOINT: &'static str = "https://api.github.com";

    pub fn get_user(username: &str) -> Result<GithubUser, &str> {
        let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::client::Client::with_connector(connector);
        let url = GITHUB_ENDPOINT.to_string() + "/users/" + username;

        let response = client
            .get(&url)
            .header(UserAgent("hyper.rs".to_string()))
            .send();

        match response {
            Ok(res) => {
                if res.status == hyper::Ok {
                    let gh_user_result: serde_json::Result<GithubUser> =
                        serde_json::from_reader(res);
                    match gh_user_result {
                        Ok(gh_user) => return Ok(gh_user),
                        Err(_) => return Err("JSON parse Error"),
                    }
                }
                return Err("HTTP Response Error");
            }
            Err(_) => Err("send request error"),
        }
    }
}
