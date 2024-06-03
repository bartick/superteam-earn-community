// @generated automatically by Diesel CLI.

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
