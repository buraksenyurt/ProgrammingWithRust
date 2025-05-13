mod handlers;
mod request;
mod router;

use handlers::*;
use request::Request;
use router::Router;

fn main() -> anyhow::Result<()> {
    // File Storage kullanımı

    let fs_handler = Box::new(FileStorageHandler {
        file_name: "route_datas.log".into(),
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
