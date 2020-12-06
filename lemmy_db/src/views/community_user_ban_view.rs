use crate::{
  community::{Community, CommunitySafe},
  schema::{community, community_user_ban, user_},
  user::{UserSafe, User_},
  ToSafe,
};
use diesel::{result::Error, *};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CommunityUserBanView {
  pub community: CommunitySafe,
  pub user: UserSafe,
}

impl CommunityUserBanView {
  pub fn get(
    conn: &PgConnection,
    from_user_id: i32,
    from_community_id: i32,
  ) -> Result<Self, Error> {
    let (community, user) = community_user_ban::table
      .inner_join(community::table)
      .inner_join(user_::table)
      .select((Community::safe_columns_tuple(), User_::safe_columns_tuple()))
      .filter(community_user_ban::community_id.eq(from_community_id))
      .filter(community_user_ban::user_id.eq(from_user_id))
      .order_by(community_user_ban::published)
      .first::<(CommunitySafe, UserSafe)>(conn)?;

    Ok(CommunityUserBanView { community, user })
  }
}