use crate::{handlers::RouteHandler, request::Request};

pub struct Router {
    requests: Vec<Request>,
    pub handler: Box<dyn RouteHandler>,
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
            self.handler.handle(r)?;
        }
        Ok(())
    }
}
