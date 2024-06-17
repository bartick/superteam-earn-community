// @generated automatically by Diesel CLI.

diesel::table! {
    discord_settings (id) {
        id -> Int8,
        bounty_id -> Nullable<Int8>,
        project_id -> Nullable<Int8>,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Nullable<Text>,
        slug -> Nullable<Text>,
        deadline -> Nullable<Timestamp>,
        token -> Nullable<Text>,
        rewardamount -> Nullable<Int4>,
        rewards -> Nullable<Jsonb>,
        skills -> Nullable<Array<Nullable<Jsonb>>>,
        _type -> Nullable<Text>,
        requirements -> Nullable<Text>,
        totalpaymentsmade -> Nullable<Int4>,
        totalwinnersselected -> Nullable<Int4>,
        iswinnersannounced -> Nullable<Bool>,
        region -> Nullable<Text>,
        pocsocials -> Nullable<Text>,
        hackathonprize -> Nullable<Bool>,
        timetocomplete -> Nullable<Text>,
        winners -> Nullable<Jsonb>,
        sponsor -> Nullable<Jsonb>,
    }
}

diesel::table! {
    telegram (id) {
        id -> Int8,
        thread_id -> Nullable<Int4>,
        can_send_messages -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    discord_settings,
    posts,
    telegram,
);
