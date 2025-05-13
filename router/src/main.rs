use std::{fs::OpenOptions, io::Write};

fn main() -> anyhow::Result<()> {
    // File Storage kullanımı

    let fs_handler = Box::new(FileStorageHandler {
        base_path: ".".into(),
    });

    let mut router = Router::new(fs_handler);

    router.add(Request {
        path: "api/users/title".into(),
        data: b"{\"name\": \"john@doe\"}".to_vec(),
    });
    router.apply()?;

    router.add(Request {
        path: "api/products".into(),
        data: b"{\"category\": \"Books\"}".to_vec(),
    });
    router.apply()?;

    // Api Yönlendirme Kullanımı
    router.handler = Box::new(PassToRemoteHandler {
        target_uri: "https://backend-services/api/route/one".into(),
    });
    router.apply()?;

    Ok(())
}

pub struct Request {
    pub path: String,
    pub data: Vec<u8>,
}

pub trait RouteHandler {
    fn handle(&self, request: &Request) -> anyhow::Result<()>;
}

pub struct FileStorageHandler {
    pub base_path: String,
}

impl RouteHandler for FileStorageHandler {
    fn handle(&self, request: &Request) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("request_logs.dat")?;
        file.write_all(&request.data)?;
        Ok(())
    }
}

pub struct PassToRemoteHandler {
    pub target_uri: String,
}

impl RouteHandler for PassToRemoteHandler {
    fn handle(&self, request: &Request) -> anyhow::Result<()> {
        println!(
            "Pretending to POST to {}{} with {} bytes",
            self.target_uri,
            request.path,
            request.data.len()
        );

        Ok(())
    }
}

pub struct Router {
    requests: Vec<Request>,
    handler: Box<dyn RouteHandler>,
}

impl Router {
    pub fn new(handler: Box<dyn RouteHandler>) -> Self {
        Router {
            requests: Vec::new(),
            handler,
        }
    }
    pub fn add(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub fn apply(&self) -> anyhow::Result<()> {
        for r in self.requests.iter() {
            self.handler.handle(&r)?;
        }
        Ok(())
    }
}
