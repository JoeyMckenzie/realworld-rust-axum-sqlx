with updated_article_cte as (
    update articles
        set updated_at = current_timestamp,
            title = $1::varchar,
            slug = $2::varchar,
            description = $3::varchar,
            body = $4::varchar
        where id = $5
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
from updated_article_cte a
         join users u on u.id = a.user_id;
