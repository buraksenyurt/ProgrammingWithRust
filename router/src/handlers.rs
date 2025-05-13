use std::{fs::OpenOptions, io::Write};

use crate::request::Request;

pub trait RouteHandler {
    fn handle(&self, request: &Request) -> anyhow::Result<()>;
}

pub struct FileStorageHandler {
    pub file_name: String,
}

impl RouteHandler for FileStorageHandler {
    fn handle(&self, request: &Request) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.file_name.clone())?;
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