use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use auth_proto::auth_server::Auth;
use auth_proto::{
    Owner, SignInRequest, SignInResponse, SignUpRequest, SignUpResponse, VerifyRequest,
    VerifyResponse,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lib::db::{get_owner_by_email, get_owner_by_id, insert_owner};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

pub mod auth_proto {
    tonic::include_proto!("auth");
}

#[derive(Serialize, Deserialize)]
struct JWTOwner {
    id: String,
    email: String,
}

pub struct AuthService {
    pool: Arc<Pool<Postgres>>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: Arc<Pool<Postgres>>, jwt_secret: String) -> Self {
        AuthService {
            pool: pool,
            jwt_secret: jwt_secret,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let req = request.into_inner();
        let owner_id = Uuid::new_v4();

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let salted_hash = match argon2.hash_password(req.password.as_bytes(), &salt) {
            Ok(password) => password.to_string(),
            Err(err) => {
                log::error!("Auth Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };

        let db_result = insert_owner(owner_id.to_string(), req.email, salted_hash)
            .execute(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(_) => SignUpResponse { success: true },
            Err(err) => {
                log::error!("Auth Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let req = request.into_inner();

        let db_result = get_owner_by_email(req.email)
            .fetch_one(self.pool.as_ref())
            .await;

        if let Err(err) = db_result {
            log::warn!("Auth Service: {}", err);
            return Err(Status::new(Code::NotFound, ""));
        }
        let row = db_result.unwrap();

        let salted_hash: String = row.get("password");
        let password_hash = PasswordHash::new(salted_hash.as_ref());
        if let Err(err) = password_hash {
            log::error!("Auth Service: {}", err);
            return Err(Status::new(Code::Internal, ""));
        }
        let parsed_hash = password_hash.unwrap();
        let argon2 = Argon2::default();
        if let Err(err) = argon2.verify_password(req.password.as_bytes(), &parsed_hash) {
            log::warn!("Auth Service: {}", err);
            return Err(Status::new(Code::PermissionDenied, ""));
        };

        let owner = JWTOwner {
            id: row.get("id"),
            email: row.get("email"),
        };
        let res = match encode(
            &Header::default(),
            &owner,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        ) {
            Ok(jwt) => SignInResponse {
                owner: Some(Owner {
                    id: owner.id,
                    email: owner.email,
                }),
                jwt: jwt,
            },
            Err(err) => {
                log::error!("Auth Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }

    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        let req = request.into_inner();

        let token_data = match decode::<JWTOwner>(
            &req.jwt,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(err) => {
                log::warn!("Auth Service: {}", err);
                return Err(Status::new(Code::PermissionDenied, ""));
            }
        };

        let db_result = get_owner_by_id(token_data.claims.id)
            .map(|row| Owner {
                id: row.get("id"),
                email: row.get("email"),
            })
            .fetch_one(self.pool.as_ref())
            .await;

        let res = match db_result {
            Ok(owner) => VerifyResponse { owner: Some(owner) },
            Err(err) => {
                log::error!("Auth Service: {}", err);
                return Err(Status::new(Code::Internal, ""));
            }
        };
        Ok(Response::new(res))
    }
}
