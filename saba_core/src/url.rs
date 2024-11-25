use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    // pub fn parse(&mut self) -> Result<Self, String> {
    //     if (!self.is_http()) {
    //         // return error
    //     }
    // }

    pub fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }

        false
    }

    pub fn extract_host(&mut self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    pub fn extract_port(&mut self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect::<Vec<&str>>();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    pub fn extract_path(&mut self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect::<Vec<&str>>();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        url_parts[1].splitn(2, "?").collect::<Vec<&str>>()[0].to_string()
    }
}

#[cfg(test)] // テスト時にのみコンパイルされることを示す
mod tests {
    use super::*; // テスト対象のモジュールをインポート

    #[test] // この関数がテスト関数であることを示す
    fn test_is_http() {
        let mut url = Url::new("http://example.com:8080".to_string());
        assert_eq!(url.is_http(), true);
    }

    #[test]
    fn test_is_not_http() {
        let mut url = Url::new("https://example.com:8080".to_string());
        assert_eq!(url.is_http(), false);
    }

    #[test]
    fn test_extract_host() {
        let mut url = Url::new("http://example.com".to_string());
        assert_eq!(url.extract_host(), "example.com".to_string());
    }

    #[test]
    fn test_extract_host_with_port() {
        let mut url = Url::new("http://example.com:8080".to_string());
        assert_eq!(url.extract_host(), "example.com".to_string());
    }

    #[test]
    fn test_extract_host_with_path() {
        let mut url = Url::new("http://example.com/path".to_string());
        assert_eq!(url.extract_host(), "example.com".to_string());
    }

    #[test]
    fn test_extract_port() {
        let mut url = Url::new("http://example.com:8080".to_string());
        assert_eq!(url.extract_port(), "8080".to_string());
    }

    #[test]
    fn test_extract_port_with_path() {
        let mut url = Url::new("http://example.com:8080/path".to_string());
        assert_eq!(url.extract_port(), "8080".to_string());
    }

    #[test]
    fn test_extract_port_without_port() {
        let mut url = Url::new("http://example.com".to_string());
        assert_eq!(url.extract_port(), "80".to_string());
    }

    #[test]
    fn test_extract_path_without_path() {
        let mut url = Url::new("http://example.com".to_string());
        assert_eq!(url.extract_path(), "".to_string());
    }

    #[test]
    fn test_extract_path_with_searchpart() {
        let mut url = Url::new("http://example.com/path/hoge?hoge=hoge".to_string());
        assert_eq!(url.extract_path(), "path/hoge".to_string());
    }

    #[test]
    fn test_extract_path_with_port() {
        let mut url = Url::new("http://example.com:8080/path/hoge".to_string());
        assert_eq!(url.extract_path(), "path/hoge".to_string());
    }
}
