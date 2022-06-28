select a.id                                                          as "id!",
       a.created_at                                                  as "created_at!",
       a.updated_at                                                  as "updated_at!",
       a.title                                                       as "title!",
       a.body                                                        as "body!",
       a.description                                                 as "description!",
       a.slug                                                        as "slug!",
       u.id                                                          as "user_id!",
       exists(
               select 1 from user_favorites af where af.user_id = $1::bigint and af.article_id = a.id
           )                                                         as "favorited!",
       (select count(*) from user_favorites where article_id = a.id) as "favorites!",
       exists(select 1
              from user_follows
              where followee_id = a.user_id
                and follower_id = $1::bigint)                           "following_author!",
       u.username                                                    as "author_username!",
       u.bio                                                         as "author_bio!",
       u.image                                                       as "author_image!"
from articles a
         join users u on u.id = a.user_id
where a.slug = $2::varchar;
