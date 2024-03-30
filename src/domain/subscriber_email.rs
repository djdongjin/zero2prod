#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail { 
    pub fn parse(s: String) -> Result<SubscriberEmail, String> { 
        // TODO: add validation!
        Ok(Self(s)) 
    } 
}