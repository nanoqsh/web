pub struct Route {
    url: String,
}

impl Route {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn make(&self) -> String {
        format!("{}/make", self.url)
    }

    pub fn balance(&self, id: &str) -> String {
        format!("{}/balance/{}", self.url, id)
    }

    pub fn put(&self, id: &str, fast: bool) -> String {
        format!("{}/put{}/{}", self.url, if fast { "_fast" } else { "" }, id)
    }

    pub fn clear(&self) -> String {
        format!("{}/clear", self.url)
    }
}
