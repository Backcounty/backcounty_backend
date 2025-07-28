pub enum Roles{
    Admin,
    Author,
    Reader,
}

pub enum ResourceType{
    Own(Resource),
    NonOwn(Resource)
}

pub enum Resource{
    Post,
    Comments,
    Drafts
}

pub enum Action{
    Create,
    Read,
    Update,
    Delete,
}

