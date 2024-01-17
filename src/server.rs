use num_bigint::BigUint;
use std::collections::HashMap;
use std::sync::Mutex;
use zkp_chaum_pedersen::ZKP;

use tonic::{transport::Server, Code, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub usersDb: Mutex<HashMap<String, UserInfo>>,
    pub authIdToUserDb: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    /// user registration
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,

    // authorization
    pub r1: BigUint,
    pub r2: BigUint,

    /// verification
    pub c: BigUint,
    pub s: BigUint,

    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: tonic::Request<RegisterRequest>,
    ) -> std::result::Result<tonic::Response<RegisterResponse>, tonic::Status> {
        println!("Processing user registration: {:?}", request);

        let request = request.into_inner();
        let user_name = request.user;

        let mut user_info = UserInfo::default();
        user_info.user_name = user_name.clone();
        user_info.y1 = BigUint::from_bytes_be(&request.y1);
        user_info.y2 = BigUint::from_bytes_be(&request.y2);

        let mut usersDb = &mut self.usersDb.lock().unwrap();
        usersDb.insert(user_name, user_info);

        Ok(Response::new(RegisterResponse {}))
    }
    async fn create_authentication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> std::result::Result<tonic::Response<AuthenticationChallengeResponse>, tonic::Status> {
        let request = request.into_inner();
        let user_name = request.user;
        println!("Processing Challenge Request username: {:?}", user_name);

        let user_info_hashmap = &mut self.usersDb.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&user_name) {
            let (_, _, _, q) = ZKP::get_constants();
            let c = ZKP::generate_random_number_below(&q);
            let auth_id = "asdfghjkl".to_string();

            user_info.c = c.clone();
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let auth_id_to_user = &mut self.authIdToUserDb.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), user_name.clone());

            println!("✅ Successful Challenge Request username: {:?}", user_name);
            
            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User: {} not found in database", user_name),
            ))
        }
    }
    async fn verify_authentication(
        &self,
        request: tonic::Request<AuthenticationAnswerRequest>,
    ) -> std::result::Result<tonic::Response<AuthenticationChallengeResponse>, tonic::Status> {
        todo!();
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:50051".to_string();

    println!("✅ Running the server in {}", addr);

    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("could not convert address"))
        .await
        .unwrap();
}
