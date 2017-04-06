pub trait Client {
    fn test(&self);
}

pub struct ClientImpl<'a> {
    url: &'a str
}

impl<'a> Client for ClientImpl<'a> {
    fn test (&self) {
        println!("{}", self.url);
    }
}

pub struct Builder<'a> {
    url: &'a str  
}

impl<'a> Builder<'a> {
    pub fn new(url: &'a str) -> Builder<'a> {
        Builder { url: url }
    }

    pub fn finalize(&self) -> ClientImpl {
        ClientImpl { url: self.url}
    }
}
