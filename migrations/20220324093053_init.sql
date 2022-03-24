create table redirects
(
    id   serial
        constraint redirects_pk
            primary key,
    slug varchar(255) not null,
    url  varchar(255) not null
);

create unique index redirects_id_uindex
    on redirects (id);

create unique index redirects_slug_uindex
    on redirects (slug);
