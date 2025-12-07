// @generated automatically by Diesel CLI.

diesel::table! {
    mo_app_user (id) {
        id -> Unsigned<Integer>,
        #[max_length = 10]
        emp_id -> Varchar,
        #[max_length = 50]
        user_name -> Varchar,
        org_id -> Nullable<Bigint>,
        age -> Nullable<Unsigned<Tinyint>>,
        #[max_length = 20]
        birthday -> Nullable<Varchar>,
        create_time -> Nullable<Datetime>,
        #[max_length = 10]
        creater_id -> Nullable<Varchar>,
        update_time -> Nullable<Datetime>,
        #[max_length = 10]
        updater_id -> Nullable<Varchar>,
    }
}

diesel::table! {
    mo_gl_org (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 50]
        org_code -> Varchar,
        #[max_length = 50]
        org_name -> Varchar,
        update_time -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(mo_app_user, mo_gl_org,);
