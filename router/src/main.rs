fn main() {
    
}

pub struct Request<'a> {
    pub path:&'a str,
    pub data:Vec<u8>
}

pub trait RouteHandler{
    fn handle(&self,request:&Request) -> anyhow::Result<()>;
}
