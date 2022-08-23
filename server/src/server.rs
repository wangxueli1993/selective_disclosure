use tonic::{transport::Server, Request, Response, Status};

use scd::{CertificateSchema, CertificateTemplate};
use wedpr_s_selective_certificate_disclosure::*;

// pub mod scd {
//     tonic::include_proto!("scd"); // 这里指定的字符串必须与proto的包名称一致
// }

use scd::selective_disclosure_server::{SelectiveDisclosure, SelectiveDisclosureServer};

#[derive(Debug, Default)]
pub struct MySelectiveDisclosure {}

#[tonic::async_trait]
impl SelectiveDisclosure for MySelectiveDisclosure {
    async fn create_template(
        &self,
        request: Request<CertificateSchema>, // 接收以模板属性为类型的请求
    ) -> Result<Response<CertificateTemplate>, Status> { // 返回以模板为类型的示例作为响应
        println!("Got a certificateschema: {:?}", request);
        let schema = request.into_inner();
        let (certificate_template, template_private_key) =
        issuer::make_certificate_template(&schema).unwrap();

        let reply = certificate_template;

        Ok(Response::new(reply)) // 发回格式化的问候语
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:64000".parse()?;
    let selecisclosure = MySelectiveDisclosure::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(SelectiveDisclosureServer::new(selecisclosure))
        .serve(addr)
        .await?;

    Ok(())
}

