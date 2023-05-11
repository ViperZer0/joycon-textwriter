// @generated automatically by Diesel CLI.

diesel::table! {
    joycon_data (symbol, training_num, sample_num) {
        symbol -> Text,
        training_num -> Integer,
        sample_num -> Integer,
        time -> Nullable<Float>,
        gyro_x -> Nullable<Float>,
        gyro_y -> Nullable<Float>,
        gyro_z -> Nullable<Float>,
        accel_x -> Nullable<Float>,
        accel_y -> Nullable<Float>,
        accel_z -> Nullable<Float>,
    }
}
