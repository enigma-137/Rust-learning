pub fn login(creds: models::Credentials) {
        //get users
        crate::database::get_users()
    }

    fn logout(creds: Credentials) {
        //log out
    }

    pub mod models;