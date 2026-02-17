use crate::{
    dto::{AuthResponse, LoginRequest, RegisterDto, UserResponse},
    errors::AppError,
    repository::UserRepository,
    utils::{create_jwt, hash_password, verify_password},
};

pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    pub async fn register(&self, req: RegisterDto) -> Result<AuthResponse, AppError> {
        if let Some(_) = self.user_repo.find_user(&req.email, &req.username).await? {
            return Err(AppError::DuplicateKeyError(
                "User with this email or username already exists".to_string(),
            ));
        }

        let password_hash = hash_password(&req.password).map_err(|_| AppError::PasswordError)?;
        let user = self
            .user_repo
            .create_user(&req.username, &req.email, &password_hash)
            .await?;

        let token =
            create_jwt(user.id, &user.email, &self.jwt_secret).map_err(|_| AppError::JwtErrors)?;
        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = self.user_repo.find_user(&req.email, "").await?;
        let user = match user {
            Some(u) => u,
            None => return Err(AppError::AuthenticationError("User not found".to_string())),
        };

        if user.attempts.unwrap_or(0) >= 3 {
            return Err(AppError::AuthenticationError(
                "Account locked. Try again later.".to_string(),
            ));
        }

        if !verify_password(&req.password, &user.password) {
            self.user_repo.update_attempts(&user.email).await?;
            return Err(AppError::AuthenticationError("Invalid password".to_string()));
        }

        self.user_repo.reset_attempts(&user.email).await?;
        let token =
            create_jwt(user.id, &user.email, &self.jwt_secret).map_err(|_| AppError::JwtErrors)?;
        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }
}