use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn new(email: String, name: String) -> Result<NewSubscriber, String> {
        let email = SubscriberEmail::parse(email)?;
        let name = SubscriberName::parse(name)?;

        Ok(Self { email, name })
    }
}
