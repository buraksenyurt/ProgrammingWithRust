#[allow(dead_code)]
pub struct User {
    username: String,
    password: String,
}
pub trait OpenAuth {
    fn validate(&self, user: &User) -> bool;
    fn is_user_locked(&self, user: &User) -> bool;
}

pub trait Logger {
    fn log(&self, message: &str);
}

pub fn login<A: OpenAuth, L: Logger>(auth: &A, logger: &L, user: &User) -> Result<String, String> {
    if user.username.len() < 5 {
        return Err("Username too short".to_string());
    }

    if auth.is_user_locked(user) {
        return Err("Account is locked".to_string());
    }

    if auth.validate(user) {
        logger.log(&format!("User {} logged in", user.username));
        Ok(format!("Welcome, {}!", user.username))
    } else {
        Err("Invalid credentials".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAuth {
        locked: bool,
        valid: bool,
    }

    impl OpenAuth for MockAuth {
        fn validate(&self, _: &User) -> bool {
            self.valid
        }

        fn is_user_locked(&self, _: &User) -> bool {
            self.locked
        }
    }

    struct TestLogger {
        pub messages: std::cell::RefCell<Vec<String>>,
    }

    impl Logger for TestLogger {
        fn log(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_successful_login_with_logging() {
        let auth = MockAuth {
            locked: false,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "Billi Geyts".to_string(),
                password: "1976".to_string(),
            },
        );
        assert!(result.is_ok());

        let logs = logger.messages.borrow();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("Billi Geyts"));
    }

    #[test]
    fn test_login_fails_for_locked_account() {
        let auth = MockAuth {
            locked: true,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "Jan Claud".to_string(),
                password: "1Dome".to_string(),
            },
        );
        assert_eq!(result, Err("Account is locked".to_string()));
        assert!(logger.messages.borrow().is_empty());
    }

    #[test]
    fn test_short_username_rejected_before_auth() {
        let auth = MockAuth {
            locked: false,
            valid: true,
        };

        let logger = TestLogger {
            messages: std::cell::RefCell::new(vec![]),
        };

        let result = login(
            &auth,
            &logger,
            &User {
                username: "r".to_string(),
                password: "secret".to_string(),
            },
        );
        assert_eq!(result, Err("Username too short".to_string()));
    }
}
