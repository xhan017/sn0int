use errors::*;


#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResponse<T> {
    #[serde(rename="success")]
    Success(T),
    #[serde(rename="error")]
    Error(String),
}

impl<T> ApiResponse<T> {
    pub fn success(self) -> Result<T> {
        match self {
            ApiResponse::Success(x) => Ok(x),
            ApiResponse::Error(err) => bail!("Api returned error: {:?}", err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhoamiResponse {
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishResponse {
    pub author: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub author: String,
    pub name: String,
    pub version: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleInfoResponse {
    pub author: String,
    pub name: String,
    pub description: String,
    pub latest: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub author: String,
    pub name: String,
    pub description: String,
    pub latest: String,
    pub downloads: i64,
    pub featured: bool,
}

impl SearchResponse {
    pub fn canonical(&self) -> String {
        format!("{}/{}", self.author, self.name)
    }
}