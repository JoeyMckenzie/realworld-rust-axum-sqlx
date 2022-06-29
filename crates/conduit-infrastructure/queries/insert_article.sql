with inserted_article_cte as (
    insert into articles (created_at, updated_at, title, body, slug, description, user_id)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar, $3::varchar, $4::varchar, $5::bigint)
        returning id as "id",
            created_at as "created_at",
            updated_at as "updated_at",
            title as "title",
            body as "body",
            slug as "slug",
            description as "description",
            user_id as "user_id")
select a.id          as "id!",
       a.created_at  as "created_at!",
       a.updated_at  as "updated_at!",
       a.title       as "title!",
       a.body        as "body!",
       a.slug        as "slug!",
       a.description as "description!",
       u.username    as "author_username!",
       u.bio         as "author_bio!",
       u.image       as "author_image!"
from inserted_article_cte a
         join users u on u.id = a.user_id;
