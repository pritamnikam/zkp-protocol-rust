pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use num_bigint::BigUint;
use std::io::stdin;
use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
};
use zkp_chaum_pedersen::ZKP;

#[tokio::main]
async fn main() {
    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Could not connect to the server.");
    println!("✅ Connected to the server");

    let mut buf = String::new();
    println!("Please provide the username:");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the username from stdin");
    let username: String = buf.trim().to_string();

    println!("Please provide the password:");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the password from stdin");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());

    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        p: p.clone(),
        q,
        alpha: alpha.clone(),
        beta: beta.clone(),
    };

    let y1 = ZKP::exponentiate(&alpha, &password, &p);
    let y2 = ZKP::exponentiate(&beta, &password, &p);

    let request = RegisterRequest {
        user: username,
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };
    let _response = client.register(request).await.expect("Register fail");
    println!("{:?}", _response);
}
