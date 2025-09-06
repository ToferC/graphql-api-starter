use async_graphql::*;

use crate::graphql::query::{UserQuery};

#[derive(Default, MergedObject)]
pub struct Query(
    UserQuery,
    /*
    CapabilityQuery,
    PersonQuery,
    TeamQuery,
    OrganizationQuery,
    RoleQuery,
    PublicationQuery,
    TaskQuery,
    WorkQuery,
     */
);