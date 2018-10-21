extern crate ring;

use self::ring::{digest, pbkdf2, rand::{SystemRandom, SecureRandom}, error::Unspecified};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const PRNG_LEN: usize = 16;
const ITERATION: u32 = 123_456;

pub type Credential = [u8; CREDENTIAL_LEN];
pub type Salt = Vec<u8>;

pub enum Error {
    WrongUsernameOrPassword
}

pub struct Secret {
    salt: Salt,
    credential: Credential,
}

impl Secret {
    pub fn new(username: &str, password: &str) -> Result<Secret, Unspecified> {
        let salt: Salt = Secret::salt(username)?;
        let mut credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(DIGEST_ALG, ITERATION, &salt,
            password.as_bytes(), &mut credential);
        Ok(Secret {credential, salt})
    }

    fn salt(username: &str) -> Result<Salt, Unspecified> {
        let random = SystemRandom::new();
        let mut prng = [0u8; PRNG_LEN];
        random.fill(&mut prng)?;
        let ub = username.as_bytes();
        let mut salt = Vec::with_capacity(prng.len() + ub.len());
        salt.extend(prng.as_ref());
        salt.extend(ub);
        Ok(salt)
    }

    pub fn verify(&self, password: &str) -> Result<(), Error> {
        let Secret {credential, salt} = self;
        pbkdf2::verify(DIGEST_ALG, ITERATION, &salt, password.as_bytes(), credential)
            .map_err(|_| Error::WrongUsernameOrPassword)
    }
}