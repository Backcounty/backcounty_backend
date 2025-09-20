use super::*;
pub mod categories{
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "categories")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub category_id:i16,
        pub name:String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        BlogCategory
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            match self {
                Relation::BlogCategory=>Entity::has_many(blog_category::Entity).into()
            }
        }
    }
    
  
    impl Related<blog::Entity> for Entity {
        fn to() -> RelationDef {
            blog_category::Relation::Blog.def()
        }
        fn via() -> Option<RelationDef> {
            Some(blog_category::Relation::Category.def().rev())
        }
    }
    

    impl ActiveModelBehavior for ActiveModel {}

}

pub mod tags{
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "tags")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub tag_id:i8,
        pub name:String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        BlogTag
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::BlogTag=>Entity::has_many(blog_tag::Entity).into()
            }
        }
    }

    impl Related<blog::Entity> for Entity {
        fn to() -> RelationDef {
            blog_tag::Relation::Blog.def()
        }
        fn via() -> Option<RelationDef> {
            Some(blog_tag::Relation::Tag.def().rev())
        }
    }


    impl ActiveModelBehavior for ActiveModel {}
}