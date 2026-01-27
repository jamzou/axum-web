use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

/// 对密码进行哈希处理
pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

/// 验证密码是否正确
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let verified = verify(password, hash)?;
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "my_secure_password";
        let hashed = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }
}